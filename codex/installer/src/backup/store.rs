use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::InstallerError;
use crate::content::{CapturedContent, ContentPayload, capture_optional, materialize_durable};
use crate::ownership::validate_manifest;
use crate::path::{Locator, RootId};
use crate::platform::{EntryKind, Platform};
use crate::source::{validate_agent_name, validate_asset_name};

use super::model::{
    BACKUP_VERSION, Backup, BackupEntry, BackupJournal, BackupRequest, EnsureBackup, PreparedBackup,
};

const JOURNAL_NAME: &str = "journal-v1.json";
const PAYLOAD_NAME: &str = "payload";
const LATEST_NAME: &str = "latest";
const LATEST_TEMP_NAME: &str = "latest.tmp";
const PUBLICATION_TEMP_NAME: &str = ".publication.tmp";

pub(crate) struct BackupStore<'a, P> {
    platform: &'a P,
    state_dir: PathBuf,
    backups_dir: PathBuf,
}

impl<'a, P: Platform> BackupStore<'a, P> {
    pub(crate) fn new(platform: &'a P, state_dir: &Path) -> Self {
        Self {
            platform,
            state_dir: state_dir.to_owned(),
            backups_dir: state_dir.join("backups"),
        }
    }

    #[cfg(test)]
    pub(crate) fn publish_current(&self, request: BackupRequest) -> Result<Backup, InstallerError> {
        self.load_latest()?;
        let prepared = self.capture_current(request)?;
        self.publish(prepared)
    }

    pub(crate) fn ensure_current(
        &self,
        request: BackupRequest,
    ) -> Result<EnsureBackup, InstallerError> {
        let prepared = self.capture_current(request)?;
        if let Some(latest) = self.load_latest()?
            && latest.journal.same_current_state(&prepared.journal)
        {
            return Ok(EnsureBackup::Reused(latest));
        }
        self.publish(prepared).map(EnsureBackup::Published)
    }

    pub(crate) fn load_latest(&self) -> Result<Option<Backup>, InstallerError> {
        match self.kind(&self.backups_dir, "inspect backups directory")? {
            None => return Ok(None),
            Some(EntryKind::Directory) => {}
            Some(_) => {
                return Err(invalid_backup(format!(
                    "backups path is not an ordinary directory: {}",
                    self.backups_dir.display()
                )));
            }
        }
        let marker = self.backups_dir.join(LATEST_NAME);
        match self.kind(&marker, "inspect latest marker")? {
            None => Ok(None),
            Some(EntryKind::File) => {
                let bytes = fs::read(&marker)
                    .map_err(|error| filesystem_error("read latest marker", &marker, error))?;
                let backup_id = parse_latest_marker(&bytes)?;
                self.load_backup(backup_id).map(Some)
            }
            Some(_) => Err(invalid_backup("latest marker is not an ordinary file")),
        }
    }

    pub(crate) fn select_latest(&self, backup_id: &str) -> Result<(), InstallerError> {
        self.load_latest()?;
        validate_backup_id(backup_id)
            .map_err(|_| invalid_backup("unsafe backup ID for latest selection"))?;
        self.load_backup(backup_id)?;
        let marker = self.backups_dir.join(LATEST_NAME);
        match self.kind(&marker, "inspect latest marker")? {
            None | Some(EntryKind::File) => {}
            Some(_) => {
                return Err(invalid_backup("latest marker is not an ordinary file"));
            }
        }
        let temporary = self.backups_dir.join(LATEST_TEMP_NAME);
        match self.kind(&temporary, "inspect temporary latest marker")? {
            None => {}
            Some(EntryKind::File) => {
                fs::remove_file(&temporary).map_err(|error| {
                    filesystem_error("remove temporary latest marker", &temporary, error)
                })?;
                self.sync_directory(&self.backups_dir)?;
            }
            Some(_) => {
                return Err(invalid_backup(
                    "temporary latest marker is not an ordinary file",
                ));
            }
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|error| {
                filesystem_error("create temporary latest marker", &temporary, error)
            })?;
        file.write_all(format!("{backup_id}\n").as_bytes())
            .map_err(|error| {
                filesystem_error("write temporary latest marker", &temporary, error)
            })?;
        file.sync_all().map_err(|error| {
            filesystem_error("synchronize temporary latest marker", &temporary, error)
        })?;
        fs::rename(&temporary, &marker)
            .map_err(|error| filesystem_error("replace latest marker", &marker, error))?;
        self.sync_directory(&self.backups_dir)
    }

    pub(crate) fn prune_unselected(&self) -> Result<Vec<PathBuf>, InstallerError> {
        let selected = self
            .load_latest()?
            .ok_or_else(|| invalid_backup("cannot prune backups without a latest marker"))?;
        let selected_id = selected.journal.backup_id;
        let mut entries = fs::read_dir(&self.backups_dir)
            .map_err(|error| filesystem_error("read backups directory", &self.backups_dir, error))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                filesystem_error("read backup directory entry", &self.backups_dir, error)
            })?;
        entries.sort_by_key(fs::DirEntry::file_name);

        let mut removable = Vec::new();
        for entry in entries {
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| invalid_backup("backup entry name is not UTF-8"))?;
            let path = entry.path();
            if name == LATEST_NAME {
                if self.kind(&path, "inspect latest marker")? != Some(EntryKind::File) {
                    return Err(invalid_backup("latest marker is not an ordinary file"));
                }
                continue;
            }
            validate_backup_id(&name)
                .map_err(|_| invalid_backup(format!("unexpected backup entry: {name}")))?;
            self.load_backup(&name)?;
            if name != selected_id {
                removable.push(path);
            }
        }

        for path in &removable {
            self.platform
                .cleanup_owned_tree(path)
                .map_err(|error| filesystem_error("remove unselected backup", path, error))?;
        }
        Ok(removable)
    }

    pub(crate) fn finalize_committed_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<(), InstallerError> {
        validate_backup_id(transaction_id)
            .map_err(|_| invalid_backup("unsafe transaction ID for backup finalization"))?;
        let candidate = self.backups_dir.join(transaction_id);
        match self.kind(&candidate, "inspect committed backup candidate")? {
            Some(EntryKind::Directory) => self.select_latest(transaction_id)?,
            None => {
                self.load_latest()?.ok_or_else(|| {
                    invalid_backup(
                        "cannot finalize committed transaction without a matching backup or latest marker",
                    )
                })?;
            }
            Some(_) => {
                return Err(invalid_backup(format!(
                    "committed backup candidate is not an ordinary directory: {}",
                    candidate.display()
                )));
            }
        }
        self.prune_unselected()?;
        Ok(())
    }

    pub(crate) fn discard_unselected(&self, backup_id: &str) -> Result<(), InstallerError> {
        validate_backup_id(backup_id)
            .map_err(|_| invalid_backup("unsafe backup ID for discard"))?;
        if self
            .load_latest()?
            .is_some_and(|latest| latest.journal.backup_id == backup_id)
        {
            return Err(invalid_backup("cannot discard the selected backup"));
        }
        let backup = self.backups_dir.join(backup_id);
        match self.kind(&backup, "inspect unselected backup")? {
            None => Ok(()),
            Some(EntryKind::Directory) => {
                self.load_backup(backup_id)?;
                self.platform
                    .cleanup_owned_tree(&backup)
                    .map_err(|error| filesystem_error("discard unselected backup", &backup, error))
            }
            Some(_) => Err(invalid_backup(format!(
                "unselected backup is not an ordinary directory: {}",
                backup.display()
            ))),
        }
    }

    fn capture_current(
        &self,
        mut request: BackupRequest,
    ) -> Result<PreparedBackup, InstallerError> {
        validate_backup_id(&request.backup_id).map_err(|_| invalid_backup("unsafe backup ID"))?;
        self.validate_roots(&request.roots)?;
        if let Some(ownership) = &request.ownership {
            validate_manifest(ownership)
                .map_err(|error| invalid_backup(format!("invalid backup ownership: {error}")))?;
        }
        request.locators.sort_by(compare_locators);
        if request.locators.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(invalid_backup("backup locators contain a duplicate"));
        }

        let mut entries = Vec::with_capacity(request.locators.len());
        let mut directories = BTreeSet::new();
        let mut files = BTreeMap::new();
        for locator in request.locators {
            validate_managed_locator(&locator)?;
            let content = capture_optional(&request.roots.resolve(&locator))?;
            if let Some(content) = &content {
                merge_payload(
                    &payload_relative(&locator),
                    content,
                    &mut directories,
                    &mut files,
                )?;
            }
            entries.push(BackupEntry {
                locator,
                sha256: content.map(|content| content.sha256),
            });
        }
        let payload = CapturedContent::directory(directories, files);
        let journal = BackupJournal {
            version: BACKUP_VERSION,
            backup_id: request.backup_id,
            roots: request.roots,
            ownership: request.ownership,
            entries,
            payload_sha256: payload.sha256.clone(),
        };
        self.validate_journal(&journal, &journal.backup_id)?;
        Ok(PreparedBackup { journal, payload })
    }

    fn publish(&self, prepared: PreparedBackup) -> Result<Backup, InstallerError> {
        self.ensure_backups_directory()?;
        let backup_id = &prepared.journal.backup_id;
        let destination = self.backups_dir.join(backup_id);
        match self.kind(&destination, "inspect backup destination")? {
            None => {}
            Some(EntryKind::Directory) => {
                let existing = self.load_backup(backup_id)?;
                if existing.journal == prepared.journal {
                    return Ok(existing);
                }
                return Err(invalid_backup(format!(
                    "backup {backup_id} already exists with different content"
                )));
            }
            Some(_) => {
                return Err(invalid_backup(format!(
                    "backup destination is not an ordinary directory: {}",
                    destination.display()
                )));
            }
        }

        let temporary = self.backups_dir.join(PUBLICATION_TEMP_NAME);
        match self.kind(&temporary, "inspect backup publication temporary")? {
            None => {}
            Some(EntryKind::Directory) => {
                self.platform
                    .cleanup_owned_tree(&temporary)
                    .map_err(|error| {
                        filesystem_error("remove stale backup publication", &temporary, error)
                    })?;
            }
            Some(_) => {
                return Err(invalid_backup(
                    "backup publication temporary is not an ordinary directory",
                ));
            }
        }
        fs::create_dir(&temporary).map_err(|error| {
            filesystem_error("create backup publication directory", &temporary, error)
        })?;
        self.sync_directory(&self.backups_dir)?;

        let publication = (|| {
            let payload_path = temporary.join(PAYLOAD_NAME);
            materialize_durable(self.platform, &prepared.payload, &payload_path)?;
            let journal_path = temporary.join(JOURNAL_NAME);
            let mut bytes = serde_json::to_vec_pretty(&prepared.journal)
                .map_err(|error| invalid_backup(format!("encode backup journal: {error}")))?;
            bytes.push(b'\n');
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&journal_path)
                .map_err(|error| filesystem_error("create backup journal", &journal_path, error))?;
            file.write_all(&bytes)
                .map_err(|error| filesystem_error("write backup journal", &journal_path, error))?;
            file.sync_all().map_err(|error| {
                filesystem_error("synchronize backup journal", &journal_path, error)
            })?;
            self.sync_directory(&temporary)?;
            self.platform
                .rename_exclusive(&temporary, &destination)
                .map_err(|error| filesystem_error("publish backup", &destination, error))?;
            self.sync_directory(&self.backups_dir)
        })();
        if let Err(error) = publication {
            if self.kind(&temporary, "inspect failed backup publication")?
                == Some(EntryKind::Directory)
            {
                self.platform
                    .cleanup_owned_tree(&temporary)
                    .map_err(|cleanup| {
                        invalid_backup(format!(
                            "{error}; failed backup publication cleanup: {cleanup}"
                        ))
                    })?;
            }
            return Err(error);
        }
        self.load_backup(backup_id)
    }

    fn load_backup(&self, backup_id: &str) -> Result<Backup, InstallerError> {
        validate_backup_id(backup_id).map_err(|_| invalid_backup("unsafe backup ID"))?;
        let directory = self.backups_dir.join(backup_id);
        if self.kind(&directory, "inspect backup directory")? != Some(EntryKind::Directory) {
            return Err(invalid_backup(format!(
                "selected backup is not an ordinary directory: {}",
                directory.display()
            )));
        }
        let mut children = fs::read_dir(&directory)
            .map_err(|error| filesystem_error("read backup directory", &directory, error))?
            .map(|entry| {
                entry
                    .map(|entry| entry.file_name())
                    .map_err(|error| filesystem_error("read backup entry", &directory, error))
            })
            .collect::<Result<Vec<_>, _>>()?;
        children.sort();
        if children
            != vec![
                std::ffi::OsString::from(JOURNAL_NAME),
                std::ffi::OsString::from(PAYLOAD_NAME),
            ]
        {
            return Err(invalid_backup(
                "backup directory must contain only journal-v1.json and payload",
            ));
        }
        let journal_path = directory.join(JOURNAL_NAME);
        let journal_bytes = ordinary_file_bytes(self.platform, &journal_path, "backup journal")?;
        let journal: BackupJournal = serde_json::from_slice(&journal_bytes)
            .map_err(|error| invalid_backup(format!("invalid backup journal: {error}")))?;
        self.validate_journal(&journal, backup_id)?;

        let payload_path = directory.join(PAYLOAD_NAME);
        let payload = capture_optional(&payload_path)
            .map_err(|error| invalid_backup(format!("invalid backup payload: {error}")))?
            .ok_or_else(|| invalid_backup("backup payload is absent"))?;
        if !matches!(payload.payload, ContentPayload::Directory { .. }) {
            return Err(invalid_backup(
                "backup payload is not an ordinary directory",
            ));
        }
        if payload.sha256 != journal.payload_sha256 {
            return Err(invalid_backup(
                "backup payload fingerprint does not match journal",
            ));
        }
        let mut contents = Vec::with_capacity(journal.entries.len());
        for entry in &journal.entries {
            let content = capture_optional(&payload_path.join(payload_relative(&entry.locator)))
                .map_err(|error| invalid_backup(format!("invalid backup entry: {error}")))?;
            match (&entry.sha256, &content) {
                (None, None) => {}
                (Some(expected), Some(content)) if expected == &content.sha256 => {}
                _ => {
                    return Err(invalid_backup(
                        "backup entry fingerprint does not match journal",
                    ));
                }
            }
            contents.push((entry.locator.clone(), content));
        }
        Ok(Backup {
            directory,
            journal,
            contents,
        })
    }

    fn validate_journal(
        &self,
        journal: &BackupJournal,
        expected_id: &str,
    ) -> Result<(), InstallerError> {
        if journal.version != BACKUP_VERSION {
            return Err(invalid_backup(format!(
                "unsupported backup journal version: {}",
                journal.version
            )));
        }
        validate_backup_id(&journal.backup_id).map_err(|_| invalid_backup("unsafe backup ID"))?;
        if journal.backup_id != expected_id {
            return Err(invalid_backup(
                "backup journal ID does not match its directory",
            ));
        }
        self.validate_roots(&journal.roots)?;
        if let Some(ownership) = &journal.ownership {
            validate_manifest(ownership)
                .map_err(|error| invalid_backup(format!("invalid backup ownership: {error}")))?;
        }
        if !valid_fingerprint(&journal.payload_sha256) {
            return Err(invalid_backup("invalid backup payload fingerprint"));
        }
        let mut previous: Option<&Locator> = None;
        for entry in &journal.entries {
            validate_managed_locator(&entry.locator)?;
            if entry
                .sha256
                .as_deref()
                .is_some_and(|fingerprint| !valid_fingerprint(fingerprint))
            {
                return Err(invalid_backup("invalid backup entry fingerprint"));
            }
            if previous.is_some_and(|prior| compare_locators(prior, &entry.locator).is_ge()) {
                return Err(invalid_backup(
                    "backup entries must be unique and sorted by locator",
                ));
            }
            previous = Some(&entry.locator);
        }
        Ok(())
    }

    fn validate_roots(&self, roots: &super::BackupRoots) -> Result<(), InstallerError> {
        if roots.state_dir != self.state_dir {
            return Err(invalid_backup(
                "backup state root does not match backup store",
            ));
        }
        for root in [&roots.codex_home, &roots.skills_home, &roots.state_dir] {
            if !root.is_absolute() || root.to_str().is_none() {
                return Err(invalid_backup("backup roots must be absolute UTF-8 paths"));
            }
        }
        for (first, second) in [
            (&roots.codex_home, &roots.skills_home),
            (&roots.codex_home, &roots.state_dir),
            (&roots.skills_home, &roots.state_dir),
        ] {
            if first.starts_with(second) || second.starts_with(first) {
                return Err(invalid_backup("backup roots must not overlap"));
            }
        }
        Ok(())
    }

    fn ensure_backups_directory(&self) -> Result<(), InstallerError> {
        if self.kind(&self.state_dir, "inspect backup state directory")?
            != Some(EntryKind::Directory)
        {
            return Err(invalid_backup(
                "backup state directory is not an ordinary directory",
            ));
        }
        match self.kind(&self.backups_dir, "inspect backups directory")? {
            Some(EntryKind::Directory) => {}
            None => {
                fs::create_dir(&self.backups_dir).map_err(|error| {
                    filesystem_error("create backups directory", &self.backups_dir, error)
                })?;
            }
            Some(_) => {
                return Err(invalid_backup("backups path is not an ordinary directory"));
            }
        }
        self.sync_directory(&self.state_dir)
    }

    fn kind(&self, path: &Path, operation: &str) -> Result<Option<EntryKind>, InstallerError> {
        self.platform
            .no_follow_kind(path)
            .map_err(|error| filesystem_error(operation, path, error))
    }

    fn sync_directory(&self, path: &Path) -> Result<(), InstallerError> {
        self.platform
            .sync_directory(path)
            .map_err(|error| filesystem_error("synchronize directory", path, error))
    }
}

fn merge_payload(
    destination: &Path,
    content: &CapturedContent,
    directories: &mut BTreeSet<PathBuf>,
    files: &mut BTreeMap<PathBuf, Vec<u8>>,
) -> Result<(), InstallerError> {
    add_parent_directories(destination, directories);
    match &content.payload {
        ContentPayload::File(bytes) => {
            if files
                .insert(destination.to_owned(), bytes.clone())
                .is_some()
            {
                return Err(invalid_backup("backup payload path is duplicated"));
            }
        }
        ContentPayload::Directory {
            directories: content_directories,
            files: content_files,
        } => {
            directories.insert(destination.to_owned());
            for relative in content_directories {
                directories.insert(destination.join(relative));
            }
            for (relative, bytes) in content_files {
                let path = destination.join(relative);
                add_parent_directories(&path, directories);
                if files.insert(path, bytes.clone()).is_some() {
                    return Err(invalid_backup("backup payload path is duplicated"));
                }
            }
        }
    }
    Ok(())
}

fn add_parent_directories(path: &Path, directories: &mut BTreeSet<PathBuf>) {
    let mut parent = path.parent();
    while let Some(path) = parent {
        if path.as_os_str().is_empty() {
            break;
        }
        directories.insert(path.to_owned());
        parent = path.parent();
    }
}

fn payload_relative(locator: &Locator) -> PathBuf {
    let root = match locator.root {
        RootId::CodexHome => "codex-home",
        RootId::SkillsHome => "skills-home",
        RootId::StateDir => "state-dir",
    };
    PathBuf::from(root).join(&locator.relative)
}

fn validate_managed_locator(locator: &Locator) -> Result<(), InstallerError> {
    let components = locator.relative.components().count();
    let text = locator.relative.to_str().unwrap_or_default();
    let valid = match locator.root {
        RootId::CodexHome if text == "config.toml" || text == "AGENTS.md" => true,
        RootId::CodexHome if components == 2 => {
            let mut parts = locator.relative.iter();
            parts.next().and_then(|part| part.to_str()) == Some("agents")
                && parts
                    .next()
                    .and_then(|part| part.to_str())
                    .is_some_and(|name| validate_agent_name(name).is_ok())
        }
        RootId::SkillsHome if components == 1 => {
            text != ".system" && validate_asset_name(text).is_ok()
        }
        RootId::StateDir => text == "manifest-v1.json",
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(invalid_backup(format!(
            "backup locator is not managed: {:?}",
            locator.relative
        )))
    }
}

fn compare_locators(first: &Locator, second: &Locator) -> std::cmp::Ordering {
    first
        .root
        .cmp(&second.root)
        .then_with(|| first.relative.cmp(&second.relative))
}

fn parse_latest_marker(bytes: &[u8]) -> Result<&str, InstallerError> {
    let text =
        std::str::from_utf8(bytes).map_err(|_| invalid_backup("latest marker is not UTF-8"))?;
    let backup_id = text
        .strip_suffix('\n')
        .filter(|id| !id.contains(['\n', '\r']))
        .ok_or_else(|| invalid_backup("latest marker must contain one backup ID and newline"))?;
    validate_backup_id(backup_id)
        .map_err(|_| invalid_backup("unsafe backup ID in latest marker"))?;
    Ok(backup_id)
}

fn validate_backup_id(backup_id: &str) -> Result<(), ()> {
    if backup_id.is_empty()
        || matches!(
            backup_id,
            "." | ".." | LATEST_NAME | LATEST_TEMP_NAME | PUBLICATION_TEMP_NAME
        )
        || !backup_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(());
    }
    Ok(())
}

fn valid_fingerprint(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn ordinary_file_bytes<P: Platform>(
    platform: &P,
    path: &Path,
    label: &str,
) -> Result<Vec<u8>, InstallerError> {
    match platform
        .no_follow_kind(path)
        .map_err(|error| filesystem_error("inspect backup file", path, error))?
    {
        Some(EntryKind::File) => {
            fs::read(path).map_err(|error| filesystem_error("read backup file", path, error))
        }
        _ => Err(invalid_backup(format!("{label} is not an ordinary file"))),
    }
}

fn invalid_backup(message: impl Into<String>) -> InstallerError {
    InstallerError::InvalidBackup {
        message: message.into(),
    }
}

fn filesystem_error(operation: &str, path: &Path, error: io::Error) -> InstallerError {
    InstallerError::Filesystem {
        message: format!("{operation} {}: {error}", path.display()),
    }
}
