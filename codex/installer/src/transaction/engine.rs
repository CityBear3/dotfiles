use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::InstallerError;
use crate::content::{capture_optional, materialize_durable};
use crate::path::validate_destination_ancestors;
use crate::plan::{AssetCategory, InstallPlan, PlanAction, PlanOperation};
use crate::platform::{EntryKind, Platform};

use super::model::{
    EntryOperation, EntryPhase, FaultPoint, MoveKind, RecoveryOutcome, TransactionOutcome,
    TransactionPhase, WalDocument,
};
use super::move_protocol::move_with_intent;
use super::recovery::{cleanup_committed, recover, rollback};
use super::wal::WalStore;

pub(crate) struct TransactionEngine<P> {
    platform: P,
}

impl<P: Platform> TransactionEngine<P> {
    pub(crate) fn new(platform: P) -> Self {
        Self { platform }
    }

    pub(crate) fn execute(
        &self,
        plan: &InstallPlan,
        transaction_id: &str,
        fault: FaultPoint,
    ) -> Result<TransactionOutcome, InstallerError> {
        let actions = plan
            .actions
            .iter()
            .filter(|action| action.operation != PlanOperation::NoOp)
            .collect::<Vec<_>>();
        validate_action_order(&actions)?;
        let prior = self.preflight(plan, &actions)?;
        if actions.is_empty() {
            return Ok(TransactionOutcome {
                transaction_id: transaction_id.to_owned(),
                applied_entries: 0,
            });
        }

        let initial = WalDocument::new(&plan.roots, transaction_id, &actions, &prior)?;
        ensure_directory_durable(&self.platform, &plan.roots.state_dir)?;
        ensure_directory_durable(&self.platform, &plan.roots.skills_home)?;
        ensure_live_parents(&self.platform, plan, &actions)?;
        let store = WalStore::open(&self.platform, &plan.roots.state_dir, true)?;
        if store.load()?.is_some() {
            return Err(InstallerError::Transaction {
                message: "an unfinished transaction already exists".to_owned(),
            });
        }
        ensure_transaction_work_absent(&self.platform, &initial)?;
        let mut wal = match store.write_initial(initial) {
            Ok(wal) => wal,
            Err(error) if matches!(&error, InstallerError::UnresolvedWalAuthority { .. }) => {
                return Err(error);
            }
            Err(error) => {
                let Some(mut authoritative) = store.load()? else {
                    return Err(error);
                };
                if let Err(recovery_error) = rollback(&self.platform, &store, &mut authoritative) {
                    return Err(InstallerError::Transaction {
                        message: format!(
                            "initial WAL write failed and rollback also failed: {recovery_error}"
                        ),
                    });
                }
                return Err(error);
            }
        };

        let result = self.execute_forward(&actions, &store, &mut wal, fault);
        if result.is_err()
            && !matches!(
                result,
                Err(InstallerError::InjectedTransactionFault { .. }
                    | InstallerError::UnresolvedWalAuthority { .. })
            )
        {
            let recovery_result = if matches!(
                wal.phase,
                TransactionPhase::Committed
                    | TransactionPhase::CleaningUp
                    | TransactionPhase::Complete
            ) {
                cleanup_committed(&self.platform, &store, &mut wal)
            } else {
                rollback(&self.platform, &store, &mut wal)
            };
            if let Err(recovery_error) = recovery_result {
                return Err(InstallerError::Transaction {
                    message: format!(
                        "transaction failed and recovery also failed: {recovery_error}"
                    ),
                });
            }
        }
        result
    }

    pub(crate) fn recover(&self, state_dir: &Path) -> Result<RecoveryOutcome, InstallerError> {
        recover(&self.platform, state_dir)
    }

    fn preflight(
        &self,
        plan: &InstallPlan,
        actions: &[&PlanAction],
    ) -> Result<Vec<Option<String>>, InstallerError> {
        require_directory(&self.platform, &plan.roots.codex_home, "Codex home")?;
        let mut prior = Vec::with_capacity(actions.len());
        for action in actions {
            let root = match action.locator.root {
                crate::path::RootId::CodexHome => &plan.roots.codex_home,
                crate::path::RootId::SkillsHome => &plan.roots.skills_home,
                crate::path::RootId::StateDir => &plan.roots.state_dir,
            };
            validate_destination_ancestors(root, &action.locator.relative)?;
            let live = plan.roots.resolve(&action.locator);
            let existing = capture_optional(&live)?;
            let valid = match action.operation {
                PlanOperation::Create => action.desired.is_some() && existing.is_none(),
                PlanOperation::Replace => action.desired.is_some() && existing.is_some(),
                PlanOperation::Remove => action.desired.is_none() && existing.is_some(),
                PlanOperation::NoOp => false,
            };
            if !valid {
                return Err(InstallerError::Transaction {
                    message: format!(
                        "planned operation no longer matches destination {}",
                        live.display()
                    ),
                });
            }
            prior.push(existing.map(|content| content.sha256));
        }
        Ok(prior)
    }

    fn execute_forward(
        &self,
        actions: &[&PlanAction],
        store: &WalStore<'_, P>,
        wal: &mut WalDocument,
        fault: FaultPoint,
    ) -> Result<TransactionOutcome, InstallerError> {
        set_phase(store, wal, TransactionPhase::Preparing)?;
        create_work_directories(&self.platform, wal)?;
        for (index, action) in actions.iter().enumerate() {
            if let Some(desired) = &action.desired {
                let stage = wal.entries[index]
                    .stage
                    .as_ref()
                    .expect("validated staged operation");
                materialize_durable(&self.platform, desired, &wal.roots.resolve(stage))?;
            }
        }
        let mut prepared = wal.clone();
        prepared.phase = TransactionPhase::Prepared;
        for entry in &mut prepared.entries {
            entry.phase = EntryPhase::Prepared;
        }
        store.replace(wal, prepared)?;
        set_phase(store, wal, TransactionPhase::Applying)?;

        for index in 0..wal.entries.len() {
            self.apply_entry(store, wal, index)?;
            if index == 0 && fault == FaultPoint::AfterFirstLiveMutationBeforeCommit {
                return Err(InstallerError::InjectedTransactionFault {
                    point: "after first live mutation before commit",
                });
            }
        }
        set_phase(store, wal, TransactionPhase::Committed)?;
        if fault == FaultPoint::AfterCommittedBeforeCleanup {
            return Err(InstallerError::InjectedTransactionFault {
                point: "after committed before cleanup",
            });
        }

        cleanup_committed(&self.platform, store, wal)?;
        Ok(TransactionOutcome {
            transaction_id: wal.transaction_id.clone(),
            applied_entries: actions.len(),
        })
    }

    fn apply_entry(
        &self,
        store: &WalStore<'_, P>,
        wal: &mut WalDocument,
        index: usize,
    ) -> Result<(), InstallerError> {
        let entry = wal.entries[index].clone();
        match entry.operation {
            EntryOperation::Create => move_with_intent(
                &self.platform,
                store,
                wal,
                super::model::MoveIntent {
                    entry_index: index,
                    kind: MoveKind::Install,
                    source: entry.stage.expect("validated create stage"),
                    destination: entry.live,
                    target_phase: EntryPhase::Applied,
                },
            ),
            EntryOperation::Replace => {
                move_with_intent(
                    &self.platform,
                    store,
                    wal,
                    super::model::MoveIntent {
                        entry_index: index,
                        kind: MoveKind::Isolate,
                        source: entry.live.clone(),
                        destination: entry.tombstone.expect("validated replace tombstone"),
                        target_phase: EntryPhase::Isolated,
                    },
                )?;
                let stage = wal.entries[index]
                    .stage
                    .clone()
                    .expect("validated replace stage");
                move_with_intent(
                    &self.platform,
                    store,
                    wal,
                    super::model::MoveIntent {
                        entry_index: index,
                        kind: MoveKind::Install,
                        source: stage,
                        destination: entry.live,
                        target_phase: EntryPhase::Applied,
                    },
                )
            }
            EntryOperation::Remove => move_with_intent(
                &self.platform,
                store,
                wal,
                super::model::MoveIntent {
                    entry_index: index,
                    kind: MoveKind::Isolate,
                    source: entry.live,
                    destination: entry.tombstone.expect("validated remove tombstone"),
                    target_phase: EntryPhase::Applied,
                },
            ),
        }
    }
}

fn validate_action_order(actions: &[&PlanAction]) -> Result<(), InstallerError> {
    if let Some(index) = actions
        .iter()
        .position(|action| action.category == AssetCategory::Manifest)
        && index + 1 != actions.len()
    {
        return Err(InstallerError::Transaction {
            message: "manifest action must be the last live mutation".to_owned(),
        });
    }
    Ok(())
}

fn set_phase<P: Platform>(
    store: &WalStore<'_, P>,
    wal: &mut WalDocument,
    phase: TransactionPhase,
) -> Result<(), InstallerError> {
    let mut next = wal.clone();
    next.phase = phase;
    store.replace(wal, next)
}

fn create_work_directories<P: Platform>(
    platform: &P,
    wal: &WalDocument,
) -> Result<(), InstallerError> {
    let state = &wal.roots.state_dir;
    let transaction = state.join("transaction");
    let work = transaction.join("work");
    let transaction_work = work.join(&wal.transaction_id);
    let stage = transaction_work.join("stage");
    let tombstone = transaction_work.join("tombstone");
    match platform
        .no_follow_kind(&work)
        .map_err(|error| filesystem_error("inspect work directory", &work, error))?
    {
        None => {
            fs::create_dir(&work)
                .map_err(|error| filesystem_error("create work directory", &work, error))?;
            platform.sync_directory(&transaction).map_err(|error| {
                filesystem_error("synchronize work parent", &transaction, error)
            })?;
        }
        Some(EntryKind::Directory) => {}
        Some(_) => {
            return Err(transaction_error(format!(
                "{} is not an ordinary work directory",
                work.display()
            )));
        }
    }
    for directory in [&transaction_work, &stage, &tombstone] {
        match platform
            .no_follow_kind(directory)
            .map_err(|error| filesystem_error("inspect work directory", directory, error))?
        {
            None => {
                fs::create_dir(directory)
                    .map_err(|error| filesystem_error("create work directory", directory, error))?;
                let parent = directory
                    .parent()
                    .ok_or_else(|| transaction_error("work directory has no parent"))?;
                platform
                    .sync_directory(parent)
                    .map_err(|error| filesystem_error("synchronize work parent", parent, error))?;
            }
            Some(_) => {
                return Err(InstallerError::Transaction {
                    message: format!(
                        "transaction work directory already exists: {}",
                        directory.display()
                    ),
                });
            }
        }
    }
    Ok(())
}

fn ensure_transaction_work_absent<P: Platform>(
    platform: &P,
    wal: &WalDocument,
) -> Result<(), InstallerError> {
    let work_root = wal.work_root();
    match platform
        .no_follow_kind(&work_root)
        .map_err(|error| filesystem_error("inspect transaction work root", &work_root, error))?
    {
        None => Ok(()),
        Some(_) => Err(transaction_error(format!(
            "transaction work root already exists without an authoritative WAL: {}",
            work_root.display()
        ))),
    }
}

fn ensure_live_parents<P: Platform>(
    platform: &P,
    plan: &InstallPlan,
    actions: &[&PlanAction],
) -> Result<(), InstallerError> {
    let mut parents = actions
        .iter()
        .filter_map(|action| {
            plan.roots
                .resolve(&action.locator)
                .parent()
                .map(Path::to_owned)
        })
        .collect::<Vec<PathBuf>>();
    parents.sort_by_key(|path| path.components().count());
    parents.dedup();
    for parent in parents {
        ensure_directory_durable(platform, &parent)?;
    }
    Ok(())
}

fn ensure_directory_durable<P: Platform>(platform: &P, path: &Path) -> Result<(), InstallerError> {
    let mut current = path.to_owned();
    let mut missing = Vec::new();
    loop {
        match platform
            .no_follow_kind(&current)
            .map_err(|error| filesystem_error("inspect directory", &current, error))?
        {
            Some(EntryKind::Directory) => break,
            Some(_) => {
                return Err(InstallerError::Transaction {
                    message: format!("{} is not an ordinary directory", current.display()),
                });
            }
            None => {
                missing.push(current.clone());
                current = current
                    .parent()
                    .ok_or_else(|| transaction_error("directory has no existing parent"))?
                    .to_owned();
            }
        }
    }
    for directory in missing.into_iter().rev() {
        fs::create_dir(&directory)
            .map_err(|error| filesystem_error("create directory", &directory, error))?;
        let parent = directory
            .parent()
            .ok_or_else(|| transaction_error("created directory has no parent"))?;
        platform
            .sync_directory(parent)
            .map_err(|error| filesystem_error("synchronize directory parent", parent, error))?;
    }
    Ok(())
}

fn require_directory<P: Platform>(
    platform: &P,
    path: &Path,
    label: &str,
) -> Result<(), InstallerError> {
    match platform
        .no_follow_kind(path)
        .map_err(|error| filesystem_error("inspect directory", path, error))?
    {
        Some(EntryKind::Directory) => Ok(()),
        _ => Err(InstallerError::Transaction {
            message: format!("{label} {} must be an ordinary directory", path.display()),
        }),
    }
}

fn transaction_error(message: impl Into<String>) -> InstallerError {
    InstallerError::Transaction {
        message: message.into(),
    }
}

fn filesystem_error(operation: &str, path: &Path, error: io::Error) -> InstallerError {
    InstallerError::Filesystem {
        message: format!("{operation} {}: {error}", path.display()),
    }
}
