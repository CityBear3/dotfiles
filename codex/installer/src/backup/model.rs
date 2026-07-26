use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::InstallerError;
use crate::content::CapturedContent;
use crate::ownership::OwnershipManifest;
use crate::path::{Locator, RootId};

pub(crate) const BACKUP_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BackupRoots {
    pub(crate) codex_home: PathBuf,
    pub(crate) skills_home: PathBuf,
    pub(crate) state_dir: PathBuf,
}

impl BackupRoots {
    pub(crate) fn resolve(&self, locator: &Locator) -> PathBuf {
        let root = match locator.root {
            RootId::CodexHome => &self.codex_home,
            RootId::SkillsHome => &self.skills_home,
            RootId::StateDir => &self.state_dir,
        };
        root.join(&locator.relative)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BackupRequest {
    pub(crate) backup_id: String,
    pub(crate) roots: BackupRoots,
    pub(crate) ownership: Option<OwnershipManifest>,
    pub(crate) locators: Vec<Locator>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BackupEntry {
    pub(crate) locator: Locator,
    pub(crate) sha256: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BackupJournal {
    pub(crate) version: u32,
    pub(crate) backup_id: String,
    pub(crate) roots: BackupRoots,
    pub(crate) ownership: Option<OwnershipManifest>,
    pub(crate) entries: Vec<BackupEntry>,
    pub(crate) payload_sha256: String,
}

impl BackupJournal {
    pub(super) fn same_current_state(&self, other: &Self) -> bool {
        self.roots == other.roots
            && self.ownership == other.ownership
            && self.entries == other.entries
            && self.payload_sha256 == other.payload_sha256
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Backup {
    pub(crate) directory: PathBuf,
    pub(crate) journal: BackupJournal,
    pub(super) contents: Vec<(Locator, Option<CapturedContent>)>,
}

impl Backup {
    pub(crate) fn content(
        &self,
        locator: &Locator,
    ) -> Result<Option<&CapturedContent>, InstallerError> {
        self.contents
            .iter()
            .find(|(candidate, _)| candidate == locator)
            .map(|(_, content)| content.as_ref())
            .ok_or_else(|| InstallerError::InvalidBackup {
                message: format!(
                    "validated backup is missing content for {:?}",
                    locator.relative
                ),
            })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum EnsureBackup {
    Published(Backup),
    Reused(Backup),
}

pub(super) struct PreparedBackup {
    pub(super) journal: BackupJournal,
    pub(super) payload: CapturedContent,
}
