use crate::InstallerError;
use crate::OperationReport;
#[cfg(target_os = "macos")]
use crate::backup::{BackupRequest, BackupRoots, BackupStore, EnsureBackup};
use crate::command::InstallCommand;
#[cfg(target_os = "macos")]
use crate::operation_lock::OperationLock;
#[cfg(target_os = "macos")]
use crate::ownership::{ManifestState, read_manifest};
#[cfg(target_os = "macos")]
use crate::plan::{InstallPlan, PlanOperation};
use crate::plan::{InstallPlanRequest, plan_install};
#[cfg(target_os = "macos")]
use crate::platform::macos::MacOsPlatform;
#[cfg(target_os = "macos")]
use crate::transaction::{FaultPoint, TransactionEngine};

use super::ApplicationContext;
#[cfg(target_os = "macos")]
use super::{discard_if_unselected, recover_unfinished};

pub(super) fn execute_dry_run(
    command: InstallCommand,
    context: ApplicationContext,
) -> Result<OperationReport, InstallerError> {
    let plan = plan_install(InstallPlanRequest {
        source_root: context.source_root,
        codex_home: command.codex_home,
        skills_home: command.skills_home,
        state_dir: command.state_dir,
        adopt_existing: command.adopt_existing,
        requested_threads: command.agent_threads,
        resources: context.resources,
    })?;
    Ok(OperationReport::install_dry_run(&plan))
}

#[cfg(target_os = "macos")]
pub(super) fn execute_mutating(
    command: InstallCommand,
    context: ApplicationContext,
    requested_operation_id: &str,
) -> Result<OperationReport, InstallerError> {
    let _lock = OperationLock::acquire(&command.codex_home)?;
    let platform = MacOsPlatform::new();
    let engine = TransactionEngine::new(platform);
    let store = BackupStore::new(&platform, &command.state_dir);
    recover_unfinished(&engine, &store, &command.state_dir)?;

    let plan = plan_install(InstallPlanRequest {
        source_root: context.source_root,
        codex_home: command.codex_home,
        skills_home: command.skills_home,
        state_dir: command.state_dir,
        adopt_existing: command.adopt_existing,
        requested_threads: command.agent_threads,
        resources: context.resources,
    })?;
    if !has_mutating_actions(&plan) {
        return Ok(OperationReport::completed_install(&plan));
    }

    engine.initialize_state(&plan.roots.state_dir)?;
    let ownership = match read_manifest(&plan.roots.state_dir.join("manifest-v1.json"))? {
        ManifestState::Absent => None,
        ManifestState::Present { manifest, .. } => Some(manifest),
    };
    let request = BackupRequest {
        backup_id: requested_operation_id.to_owned(),
        roots: BackupRoots {
            codex_home: plan.roots.codex_home.clone(),
            skills_home: plan.roots.skills_home.clone(),
            state_dir: plan.roots.state_dir.clone(),
        },
        ownership,
        locators: plan
            .actions
            .iter()
            .map(|action| action.locator.clone())
            .collect(),
    };
    let store = BackupStore::new(&platform, &plan.roots.state_dir);
    let ensured = store.ensure_current(request)?;
    let (backup, select_after_commit) = match ensured {
        EnsureBackup::Published(backup) => (backup, true),
        EnsureBackup::Reused(backup) => (backup, false),
    };
    let execution = engine.execute_with_finalization(
        &plan,
        &backup.journal.backup_id,
        FaultPoint::None,
        |transaction_id| store.finalize_committed_transaction(transaction_id),
    );
    if let Err(error) = execution {
        if select_after_commit
            && matches!(
                engine.has_unfinished_transaction(&plan.roots.state_dir),
                Ok(false)
            )
        {
            discard_if_unselected(&store, &backup.journal.backup_id)?;
        }
        return Err(error);
    }

    Ok(OperationReport::completed_install(&plan))
}

#[cfg(not(target_os = "macos"))]
pub(super) fn execute_mutating(
    _command: InstallCommand,
    _context: ApplicationContext,
    _requested_operation_id: &str,
) -> Result<OperationReport, InstallerError> {
    Err(InstallerError::UnsupportedPlatform)
}

#[cfg(target_os = "macos")]
fn has_mutating_actions(plan: &InstallPlan) -> bool {
    plan.actions
        .iter()
        .any(|action| action.operation != PlanOperation::NoOp)
}
