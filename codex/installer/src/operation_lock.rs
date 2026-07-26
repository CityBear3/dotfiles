use std::fs::{File, OpenOptions};
use std::path::Path;

use crate::InstallerError;

pub(crate) struct OperationLock {
    _file: File,
}

impl OperationLock {
    pub(crate) fn acquire(codex_home: &Path) -> Result<Self, InstallerError> {
        let metadata =
            std::fs::symlink_metadata(codex_home).map_err(|error| InstallerError::Lock {
                message: format!("inspect Codex home {}: {error}", codex_home.display()),
            })?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(InstallerError::Lock {
                message: format!(
                    "Codex home is not an ordinary directory: {}",
                    codex_home.display()
                ),
            });
        }
        let path = codex_home.join("codex-manifest-installer.lock");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .map_err(|error| InstallerError::Lock {
                message: format!("open operation lock {}: {error}", path.display()),
            })?;
        file.lock().map_err(|error| InstallerError::Lock {
            message: format!("acquire operation lock {}: {error}", path.display()),
        })?;
        Ok(Self { _file: file })
    }
}

#[cfg(test)]
#[path = "operation_lock_tests.rs"]
mod tests;
