use std::path::Path;

use crate::InstallerError;
use crate::backup::BackupStore;
use crate::command::RestoreCommand;
use crate::operation_lock::OperationLock;
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
    let initial = require_latest(&store)?;
    let locked_codex_home = initial.journal.roots.codex_home;
    let _lock = OperationLock::acquire(&locked_codex_home)?;

    let engine = TransactionEngine::new(platform);
    recover_unfinished(&engine, &store, &command.state_dir)?;
    let backup = require_latest(&store)?;
    if backup.journal.roots.codex_home != locked_codex_home {
        return Err(InstallerError::InvalidBackup {
            message:
                "latest backup changed to a different Codex home while acquiring the operation lock"
                    .to_owned(),
        });
    }
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

fn require_latest<P: crate::platform::Platform>(
    store: &BackupStore<'_, P>,
) -> Result<crate::backup::Backup, InstallerError> {
    store
        .load_latest()?
        .ok_or_else(|| InstallerError::InvalidBackup {
            message: "restore requires a selected latest backup".to_owned(),
        })
}
