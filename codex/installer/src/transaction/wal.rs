use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use crate::InstallerError;
use crate::platform::{EntryKind, Platform};

use super::model::{WalDocument, invalid_wal};

const MAX_WAL_BYTES: u64 = 1024 * 1024;

pub(crate) struct WalStore<'a, P> {
    platform: &'a P,
    transaction_dir: PathBuf,
    canonical: PathBuf,
    temporary: PathBuf,
}

impl<'a, P: Platform> WalStore<'a, P> {
    pub(crate) fn open(
        platform: &'a P,
        state_dir: &Path,
        create: bool,
    ) -> Result<Self, InstallerError> {
        require_directory(platform, state_dir, "state directory")?;
        let transaction_dir = state_dir.join("transaction");
        match platform.no_follow_kind(&transaction_dir).map_err(|error| {
            filesystem_error("inspect transaction directory", &transaction_dir, error)
        })? {
            Some(EntryKind::Directory) => {}
            None if create => {
                fs::create_dir(&transaction_dir).map_err(|error| {
                    filesystem_error("create transaction directory", &transaction_dir, error)
                })?;
                platform.sync_directory(state_dir).map_err(|error| {
                    filesystem_error("synchronize state directory", state_dir, error)
                })?;
            }
            None => {}
            Some(_) => {
                return Err(invalid_wal(format!(
                    "{} is not an ordinary directory",
                    transaction_dir.display()
                )));
            }
        }
        let store = Self {
            platform,
            canonical: transaction_dir.join("wal-v1.json"),
            temporary: transaction_dir.join("wal-v1.json.tmp"),
            transaction_dir,
        };
        store.discard_stale_temporary()?;
        Ok(store)
    }

    pub(crate) fn canonical_path(&self) -> &Path {
        &self.canonical
    }

    pub(crate) fn load(&self) -> Result<Option<WalDocument>, InstallerError> {
        match self
            .platform
            .no_follow_kind(&self.canonical)
            .map_err(|error| filesystem_error("inspect transaction WAL", &self.canonical, error))?
        {
            None => return Ok(None),
            Some(EntryKind::File) => {}
            Some(_) => {
                return Err(invalid_wal(format!(
                    "{} is not an ordinary file",
                    self.canonical.display()
                )));
            }
        }
        let file = OpenOptions::new()
            .read(true)
            .open(&self.canonical)
            .map_err(|error| filesystem_error("open transaction WAL", &self.canonical, error))?;
        let length = file
            .metadata()
            .map_err(|error| filesystem_error("inspect transaction WAL", &self.canonical, error))?
            .len();
        if length > MAX_WAL_BYTES {
            return Err(invalid_wal("transaction WAL exceeds the size limit"));
        }
        let mut bytes = Vec::with_capacity(length as usize);
        file.take(MAX_WAL_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| filesystem_error("read transaction WAL", &self.canonical, error))?;
        if bytes.len() as u64 > MAX_WAL_BYTES {
            return Err(invalid_wal("transaction WAL exceeds the size limit"));
        }
        let document: WalDocument = serde_json::from_slice(&bytes)
            .map_err(|error| invalid_wal(format!("cannot parse WAL JSON: {error}")))?;
        document.validate()?;
        self.validate_transaction_ancestors(&document)?;
        Ok(Some(document))
    }

    pub(crate) fn replace(
        &self,
        current: &mut WalDocument,
        next: WalDocument,
    ) -> Result<(), InstallerError> {
        next.validate()?;
        self.validate_transaction_ancestors(&next)?;
        let mut bytes = serde_json::to_vec_pretty(&next)
            .map_err(|error| invalid_wal(format!("cannot encode WAL JSON: {error}")))?;
        bytes.push(b'\n');
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.temporary)
            .map_err(|error| filesystem_error("create temporary WAL", &self.temporary, error))?;
        file.write_all(&bytes)
            .map_err(|error| filesystem_error("write temporary WAL", &self.temporary, error))?;
        file.sync_all().map_err(|error| {
            filesystem_error("synchronize temporary WAL", &self.temporary, error)
        })?;
        fs::rename(&self.temporary, &self.canonical)
            .map_err(|error| filesystem_error("replace transaction WAL", &self.canonical, error))?;
        *current = next;
        if let Err(sync_error) = self.platform.sync_directory(&self.transaction_dir) {
            match self.load() {
                Ok(Some(authoritative)) => *current = authoritative,
                Ok(None) => {
                    return Err(InstallerError::UnresolvedWalAuthority {
                        wal: self.canonical.clone(),
                        message: format!(
                            "WAL replacement synchronization failed and canonical authority is absent: {sync_error}"
                        ),
                    });
                }
                Err(reload_error) => {
                    return Err(InstallerError::UnresolvedWalAuthority {
                        wal: self.canonical.clone(),
                        message: format!(
                            "WAL replacement synchronization failed and canonical authority cannot be reloaded: {sync_error}; {reload_error}"
                        ),
                    });
                }
            }
            return Err(filesystem_error(
                "synchronize transaction directory",
                &self.transaction_dir,
                sync_error,
            ));
        }
        Ok(())
    }

    pub(crate) fn write_initial(
        &self,
        document: WalDocument,
    ) -> Result<WalDocument, InstallerError> {
        if self.load()?.is_some() {
            return Err(InstallerError::Transaction {
                message: "an unfinished transaction already exists".to_owned(),
            });
        }
        let mut current = document.clone();
        self.replace(&mut current, document)?;
        Ok(current)
    }

    pub(crate) fn remove(&self) -> Result<(), InstallerError> {
        match self
            .platform
            .no_follow_kind(&self.canonical)
            .map_err(|error| filesystem_error("inspect transaction WAL", &self.canonical, error))?
        {
            None => return Ok(()),
            Some(EntryKind::File) => fs::remove_file(&self.canonical).map_err(|error| {
                filesystem_error("remove transaction WAL", &self.canonical, error)
            })?,
            Some(_) => {
                return Err(invalid_wal(format!(
                    "{} is not an ordinary file",
                    self.canonical.display()
                )));
            }
        }
        self.platform
            .sync_directory(&self.transaction_dir)
            .map_err(|error| {
                filesystem_error(
                    "synchronize transaction directory",
                    &self.transaction_dir,
                    error,
                )
            })
    }

    fn discard_stale_temporary(&self) -> Result<(), InstallerError> {
        match self
            .platform
            .no_follow_kind(&self.temporary)
            .map_err(|error| filesystem_error("inspect temporary WAL", &self.temporary, error))?
        {
            None => Ok(()),
            Some(EntryKind::File) => {
                fs::remove_file(&self.temporary).map_err(|error| {
                    filesystem_error("remove stale temporary WAL", &self.temporary, error)
                })?;
                self.platform
                    .sync_directory(&self.transaction_dir)
                    .map_err(|error| {
                        filesystem_error(
                            "synchronize transaction directory",
                            &self.transaction_dir,
                            error,
                        )
                    })
            }
            Some(_) => Err(invalid_wal(format!(
                "{} is not an ordinary file",
                self.temporary.display()
            ))),
        }
    }

    fn validate_transaction_ancestors(&self, document: &WalDocument) -> Result<(), InstallerError> {
        if self.transaction_dir.parent() != Some(document.roots.state_dir.as_path()) {
            return Err(invalid_wal(
                "WAL state root does not match its canonical location",
            ));
        }
        for entry in &document.entries {
            for locator in [entry.stage.as_ref(), entry.tombstone.as_ref()]
                .into_iter()
                .flatten()
            {
                let components = locator.relative.components().collect::<Vec<_>>();
                let mut current = document.roots.state_dir.clone();
                for component in components.iter().take(components.len().saturating_sub(1)) {
                    current.push(component.as_os_str());
                    match self.platform.no_follow_kind(&current).map_err(|error| {
                        filesystem_error("inspect transaction locator ancestor", &current, error)
                    })? {
                        Some(EntryKind::Directory) => {}
                        None => break,
                        Some(_) => {
                            return Err(invalid_wal(format!(
                                "{} is not an ordinary transaction directory",
                                current.display()
                            )));
                        }
                    }
                }
            }
        }
        Ok(())
    }
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
        _ => Err(invalid_wal(format!(
            "{label} {} is not an ordinary directory",
            path.display()
        ))),
    }
}

fn filesystem_error(operation: &str, path: &Path, error: io::Error) -> InstallerError {
    InstallerError::Filesystem {
        message: format!("{operation} {}: {error}", path.display()),
    }
}
