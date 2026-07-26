use std::path::Path;

use crate::InstallerError;
use crate::backup::BackupStore;
use crate::command::RestoreCommand;
use crate::operation_lock::OperationLock;
use crate::path::InstallRoots;
use crate::plan::{PlanOperation, build_restore_plan};
use crate::platform::macos::MacOsPlatform;
use crate::transaction::{FaultPoint, TransactionEngine};

use super::recover_unfinished;

pub(super) fn execute_mutating(
    command: RestoreCommand,
    source_root: &Path,
    operation_id: &str,
) -> Result<String, InstallerError> {
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &command.state_dir);
    let locked_codex_home = resolve_lock_target(&store, source_root)?;
    let _lock = OperationLock::acquire(&locked_codex_home)?;
    execute_locked(
        platform,
        &store,
        &command.state_dir,
        source_root,
        operation_id,
        &locked_codex_home,
    )
}

pub(super) fn execute_locked(
    platform: MacOsPlatform,
    store: &BackupStore<'_, MacOsPlatform>,
    state_dir: &Path,
    source_root: &Path,
    operation_id: &str,
    locked_codex_home: &Path,
) -> Result<String, InstallerError> {
    let engine = TransactionEngine::new(platform);
    let post_lock = require_latest(store)?;
    validate_locked_authority(source_root, &post_lock, locked_codex_home)?;
    recover_unfinished(&engine, store, state_dir)?;
    let backup = require_latest(store)?;
    validate_locked_authority(source_root, &backup, locked_codex_home)?;
    let plan = build_restore_plan(source_root, &backup)?;
    if plan
        .actions
        .iter()
        .all(|action| action.operation == PlanOperation::NoOp)
    {
        return Ok("restore complete\n".to_owned());
    }

    engine.execute_with_finalization(&plan, operation_id, FaultPoint::None, |transaction_id| {
        store.finalize_committed_transaction(transaction_id)
    })?;
    Ok("restore complete\n".to_owned())
}

fn resolve_lock_target<P: crate::platform::Platform>(
    store: &BackupStore<'_, P>,
    source_root: &Path,
) -> Result<std::path::PathBuf, InstallerError> {
    let initial_roots = require_latest_roots(store)?;
    validate_restore_roots(source_root, &initial_roots)?;
    Ok(initial_roots.codex_home)
}

fn require_latest_roots<P: crate::platform::Platform>(
    store: &BackupStore<'_, P>,
) -> Result<crate::backup::BackupRoots, InstallerError> {
    store
        .load_latest_roots()?
        .ok_or_else(|| InstallerError::InvalidBackup {
            message: "restore requires a selected latest backup".to_owned(),
        })
}

fn require_latest<P: crate::platform::Platform>(
    store: &BackupStore<'_, P>,
) -> Result<crate::backup::Backup, InstallerError> {
    store
        .load_latest()?
        .ok_or_else(|| InstallerError::InvalidBackup {
            message: "restore requires a selected latest backup".to_owned(),
        })
}

fn validate_locked_authority(
    source_root: &Path,
    backup: &crate::backup::Backup,
    locked_codex_home: &Path,
) -> Result<(), InstallerError> {
    validate_restore_roots(source_root, &backup.journal.roots)?;
    if backup.journal.roots.codex_home != locked_codex_home {
        return Err(InstallerError::InvalidBackup {
            message:
                "latest backup changed to a different Codex home while acquiring the operation lock"
                    .to_owned(),
        });
    }
    Ok(())
}

fn validate_restore_roots(
    source_root: &Path,
    roots: &crate::backup::BackupRoots,
) -> Result<(), InstallerError> {
    InstallRoots::normalize(
        source_root,
        &roots.codex_home,
        &roots.skills_home,
        &roots.state_dir,
    )?;
    Ok(())
}
