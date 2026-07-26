mod model;
mod store;

pub(crate) use model::{Backup, BackupRequest, BackupRoots, EnsureBackup};
pub(crate) use store::BackupStore;

#[cfg(test)]
#[path = "backup/backup_tests.rs"]
mod tests;
