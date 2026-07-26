use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::InstallerError;
use crate::path::{InstallRoots, Locator, RootId};
use crate::plan::{AssetCategory, PlanAction, PlanOperation};

pub(crate) const WAL_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FaultPoint {
    None,
    AfterFirstLiveMutationBeforeCommit,
    AfterCommittedBeforeCleanup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RecoveryOutcome {
    NoTransaction,
    RolledBack { transaction_id: String },
    CleanedCommitted { transaction_id: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TransactionOutcome {
    pub(crate) transaction_id: String,
    pub(crate) applied_entries: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TransactionRoots {
    pub(crate) codex_home: PathBuf,
    pub(crate) skills_home: PathBuf,
    pub(crate) state_dir: PathBuf,
}

impl From<&InstallRoots> for TransactionRoots {
    fn from(roots: &InstallRoots) -> Self {
        Self {
            codex_home: roots.codex_home.clone(),
            skills_home: roots.skills_home.clone(),
            state_dir: roots.state_dir.clone(),
        }
    }
}

impl TransactionRoots {
    pub(crate) fn resolve(&self, locator: &Locator) -> PathBuf {
        let root = match locator.root {
            RootId::CodexHome => &self.codex_home,
            RootId::SkillsHome => &self.skills_home,
            RootId::StateDir => &self.state_dir,
        };
        root.join(&locator.relative)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TransactionPhase {
    Planned,
    Preparing,
    Prepared,
    Applying,
    RollingBack,
    RolledBack,
    Committed,
    CleaningUp,
    Complete,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EntryPhase {
    Planned,
    Prepared,
    Isolated,
    Applied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EntryOperation {
    Create,
    Replace,
    Remove,
}

impl TryFrom<PlanOperation> for EntryOperation {
    type Error = InstallerError;

    fn try_from(operation: PlanOperation) -> Result<Self, Self::Error> {
        match operation {
            PlanOperation::Create => Ok(Self::Create),
            PlanOperation::Replace => Ok(Self::Replace),
            PlanOperation::Remove => Ok(Self::Remove),
            PlanOperation::NoOp => Err(invalid_wal("no-op action cannot be persisted")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TransactionCategory {
    Config,
    GlobalAgents,
    Skill,
    Agent,
    Manifest,
}

impl From<AssetCategory> for TransactionCategory {
    fn from(category: AssetCategory) -> Self {
        match category {
            AssetCategory::Config => Self::Config,
            AssetCategory::GlobalAgents => Self::GlobalAgents,
            AssetCategory::Skill => Self::Skill,
            AssetCategory::Agent => Self::Agent,
            AssetCategory::Manifest => Self::Manifest,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum MoveKind {
    Isolate,
    Install,
    RollbackDesired,
    RestorePrior,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MoveIntent {
    pub(crate) entry_index: usize,
    pub(crate) kind: MoveKind,
    pub(crate) source: Locator,
    pub(crate) destination: Locator,
    pub(crate) target_phase: EntryPhase,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WalEntry {
    pub(crate) operation: EntryOperation,
    pub(crate) category: TransactionCategory,
    pub(crate) name: Option<String>,
    pub(crate) live: Locator,
    pub(crate) stage: Option<Locator>,
    pub(crate) tombstone: Option<Locator>,
    pub(crate) desired_sha256: Option<String>,
    pub(crate) prior_sha256: Option<String>,
    pub(crate) phase: EntryPhase,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WalDocument {
    pub(crate) version: u32,
    pub(crate) transaction_id: String,
    pub(crate) roots: TransactionRoots,
    pub(crate) phase: TransactionPhase,
    pub(crate) entries: Vec<WalEntry>,
    pub(crate) pending_move: Option<MoveIntent>,
}

impl WalDocument {
    pub(crate) fn new(
        roots: &InstallRoots,
        transaction_id: &str,
        actions: &[&PlanAction],
        prior_sha256: &[Option<String>],
    ) -> Result<Self, InstallerError> {
        validate_transaction_id(transaction_id)?;
        if actions.len() != prior_sha256.len() {
            return Err(invalid_wal("action and prior fingerprint counts differ"));
        }
        let entries = actions
            .iter()
            .enumerate()
            .map(|(index, action)| {
                let operation = EntryOperation::try_from(action.operation)?;
                let work = format!("transaction/work/{transaction_id}");
                let stage = matches!(operation, EntryOperation::Create | EntryOperation::Replace)
                    .then(|| Locator::new(RootId::StateDir, format!("{work}/stage/{index}")))
                    .transpose()?;
                let tombstone =
                    matches!(operation, EntryOperation::Replace | EntryOperation::Remove)
                        .then(|| {
                            Locator::new(RootId::StateDir, format!("{work}/tombstone/{index}"))
                        })
                        .transpose()?;
                Ok(WalEntry {
                    operation,
                    category: action.category.into(),
                    name: action.name.clone(),
                    live: action.locator.clone(),
                    stage,
                    tombstone,
                    desired_sha256: action
                        .desired
                        .as_ref()
                        .map(|content| content.sha256.clone()),
                    prior_sha256: prior_sha256[index].clone(),
                    phase: EntryPhase::Planned,
                })
            })
            .collect::<Result<Vec<_>, InstallerError>>()?;
        let document = Self {
            version: WAL_VERSION,
            transaction_id: transaction_id.to_owned(),
            roots: TransactionRoots::from(roots),
            phase: TransactionPhase::Planned,
            entries,
            pending_move: None,
        };
        document.validate()?;
        Ok(document)
    }

    pub(crate) fn validate(&self) -> Result<(), InstallerError> {
        if self.version != WAL_VERSION {
            return Err(invalid_wal(format!(
                "unsupported WAL version {}",
                self.version
            )));
        }
        validate_transaction_id(&self.transaction_id)?;
        validate_roots(&self.roots)?;
        if self.entries.is_empty() {
            return Err(invalid_wal("WAL must contain at least one entry"));
        }

        let mut referenced_paths = Vec::with_capacity(self.entries.len() * 3);
        for (index, entry) in self.entries.iter().enumerate() {
            validate_entry(self, index, entry)?;
            for locator in [
                Some(&entry.live),
                entry.stage.as_ref(),
                entry.tombstone.as_ref(),
            ]
            .into_iter()
            .flatten()
            {
                let path = self.roots.resolve(locator);
                if referenced_paths
                    .iter()
                    .any(|prior: &PathBuf| prior.starts_with(&path) || path.starts_with(prior))
                {
                    return Err(invalid_wal("transaction entry locators overlap"));
                }
                referenced_paths.push(path);
            }
        }
        if let Some(manifest_index) = self
            .entries
            .iter()
            .position(|entry| entry.category == TransactionCategory::Manifest)
            && manifest_index + 1 != self.entries.len()
        {
            return Err(invalid_wal("manifest entry must be last"));
        }
        if let Some(intent) = &self.pending_move {
            validate_intent(self, intent)?;
        }
        validate_phases(self)?;
        Ok(())
    }

    pub(crate) fn work_root(&self) -> PathBuf {
        self.roots
            .state_dir
            .join("transaction/work")
            .join(&self.transaction_id)
    }
}

fn validate_entry(
    document: &WalDocument,
    index: usize,
    entry: &WalEntry,
) -> Result<(), InstallerError> {
    let expected_base = format!("transaction/work/{}", document.transaction_id);
    let expected_stage = Locator::new(RootId::StateDir, format!("{expected_base}/stage/{index}"))?;
    let expected_tombstone = Locator::new(
        RootId::StateDir,
        format!("{expected_base}/tombstone/{index}"),
    )?;
    match entry.operation {
        EntryOperation::Create
            if entry.stage.as_ref() != Some(&expected_stage)
                || entry.tombstone.is_some()
                || entry.desired_sha256.is_none()
                || entry.prior_sha256.is_some() =>
        {
            return Err(invalid_wal("invalid create entry fields"));
        }
        EntryOperation::Replace
            if entry.stage.as_ref() != Some(&expected_stage)
                || entry.tombstone.as_ref() != Some(&expected_tombstone)
                || entry.desired_sha256.is_none()
                || entry.prior_sha256.is_none() =>
        {
            return Err(invalid_wal("invalid replace entry fields"));
        }
        EntryOperation::Remove
            if entry.stage.is_some()
                || entry.tombstone.as_ref() != Some(&expected_tombstone)
                || entry.desired_sha256.is_some()
                || entry.prior_sha256.is_none() =>
        {
            return Err(invalid_wal("invalid remove entry fields"));
        }
        _ => {}
    }
    for fingerprint in [
        entry.desired_sha256.as_deref(),
        entry.prior_sha256.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        if fingerprint.len() != 64
            || !fingerprint
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(invalid_wal("fingerprint must be lowercase SHA-256"));
        }
    }
    validate_live_locator(entry)?;
    Ok(())
}

fn validate_live_locator(entry: &WalEntry) -> Result<(), InstallerError> {
    let expected = match entry.category {
        TransactionCategory::Config if entry.name.is_none() => {
            Locator::new(RootId::CodexHome, "config.toml")?
        }
        TransactionCategory::GlobalAgents if entry.name.is_none() => {
            Locator::new(RootId::CodexHome, "AGENTS.md")?
        }
        TransactionCategory::Skill => {
            let name = entry
                .name
                .as_deref()
                .ok_or_else(|| invalid_wal("skill entry requires a name"))?;
            if name == ".system" {
                return Err(invalid_wal(".system may not be managed"));
            }
            if !is_safe_asset_name(name) {
                return Err(invalid_wal("skill name is unsafe"));
            }
            Locator::new(RootId::SkillsHome, name)?
        }
        TransactionCategory::Agent => {
            let name = entry
                .name
                .as_deref()
                .ok_or_else(|| invalid_wal("agent entry requires a name"))?;
            let stem = name
                .strip_suffix(".toml")
                .ok_or_else(|| invalid_wal("agent name must end in .toml"))?;
            if !is_safe_asset_name(stem) {
                return Err(invalid_wal("agent name is unsafe"));
            }
            Locator::new(RootId::CodexHome, Path::new("agents").join(name))?
        }
        TransactionCategory::Manifest if entry.name.is_none() => {
            Locator::new(RootId::StateDir, "manifest-v1.json")?
        }
        _ => return Err(invalid_wal("invalid category/name combination")),
    };
    if entry.live != expected {
        return Err(invalid_wal("live locator does not match its category"));
    }
    Ok(())
}

fn is_safe_asset_name(name: &str) -> bool {
    !name.is_empty()
        && !matches!(name, "." | "..")
        && name.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
        })
}

fn validate_intent(document: &WalDocument, intent: &MoveIntent) -> Result<(), InstallerError> {
    let entry = document
        .entries
        .get(intent.entry_index)
        .ok_or_else(|| invalid_wal("pending move entry index is out of bounds"))?;
    let valid = match intent.kind {
        MoveKind::Isolate => {
            intent.source == entry.live
                && entry.tombstone.as_ref() == Some(&intent.destination)
                && matches!(
                    (
                        document.phase,
                        entry.operation,
                        entry.phase,
                        intent.target_phase
                    ),
                    (
                        TransactionPhase::Applying,
                        EntryOperation::Replace,
                        EntryPhase::Prepared,
                        EntryPhase::Isolated
                    ) | (
                        TransactionPhase::Applying,
                        EntryOperation::Remove,
                        EntryPhase::Prepared,
                        EntryPhase::Applied
                    )
                )
        }
        MoveKind::Install => {
            entry.stage.as_ref() == Some(&intent.source)
                && intent.destination == entry.live
                && intent.target_phase == EntryPhase::Applied
                && matches!(
                    (document.phase, entry.operation, entry.phase),
                    (
                        TransactionPhase::Applying,
                        EntryOperation::Create,
                        EntryPhase::Prepared
                    ) | (
                        TransactionPhase::Applying,
                        EntryOperation::Replace,
                        EntryPhase::Isolated
                    )
                )
        }
        MoveKind::RollbackDesired => {
            intent.source == entry.live
                && entry.stage.as_ref() == Some(&intent.destination)
                && matches!(
                    (
                        document.phase,
                        entry.operation,
                        entry.phase,
                        intent.target_phase
                    ),
                    (
                        TransactionPhase::RollingBack,
                        EntryOperation::Create,
                        EntryPhase::Applied,
                        EntryPhase::Prepared
                    ) | (
                        TransactionPhase::RollingBack,
                        EntryOperation::Replace,
                        EntryPhase::Applied,
                        EntryPhase::Isolated
                    )
                )
        }
        MoveKind::RestorePrior => {
            entry.tombstone.as_ref() == Some(&intent.source)
                && intent.destination == entry.live
                && intent.target_phase == EntryPhase::Prepared
                && matches!(
                    (document.phase, entry.operation, entry.phase),
                    (
                        TransactionPhase::RollingBack,
                        EntryOperation::Replace,
                        EntryPhase::Isolated
                    ) | (
                        TransactionPhase::RollingBack,
                        EntryOperation::Remove,
                        EntryPhase::Applied
                    )
                )
        }
    };
    if !valid {
        return Err(invalid_wal("pending move does not match its entry"));
    }
    Ok(())
}

fn validate_phases(document: &WalDocument) -> Result<(), InstallerError> {
    let phases_are = |allowed: &[EntryPhase]| {
        document
            .entries
            .iter()
            .all(|entry| allowed.contains(&entry.phase))
    };
    let valid = match document.phase {
        TransactionPhase::Planned | TransactionPhase::Preparing => {
            phases_are(&[EntryPhase::Planned])
        }
        TransactionPhase::Prepared => phases_are(&[EntryPhase::Prepared]),
        TransactionPhase::Applying | TransactionPhase::RollingBack => phases_are(&[
            EntryPhase::Planned,
            EntryPhase::Prepared,
            EntryPhase::Isolated,
            EntryPhase::Applied,
        ]),
        TransactionPhase::RolledBack => phases_are(&[EntryPhase::Planned, EntryPhase::Prepared]),
        TransactionPhase::Committed | TransactionPhase::CleaningUp | TransactionPhase::Complete => {
            phases_are(&[EntryPhase::Applied])
        }
    };
    if !valid {
        return Err(invalid_wal(
            "entry phases are inconsistent with the transaction phase",
        ));
    }
    if document.pending_move.is_some()
        && !matches!(
            document.phase,
            TransactionPhase::Applying | TransactionPhase::RollingBack
        )
    {
        return Err(invalid_wal(
            "pending move is inconsistent with the transaction phase",
        ));
    }
    Ok(())
}

fn validate_roots(roots: &TransactionRoots) -> Result<(), InstallerError> {
    let values = [&roots.codex_home, &roots.skills_home, &roots.state_dir];
    for root in values {
        if !root.is_absolute() || root.to_str().is_none() {
            return Err(invalid_wal(
                "transaction roots must be absolute UTF-8 paths",
            ));
        }
        if root.components().any(|component| {
            matches!(
                component,
                std::path::Component::CurDir | std::path::Component::ParentDir
            )
        }) {
            return Err(invalid_wal(
                "transaction roots must not contain dot components",
            ));
        }
    }
    for (index, first) in values.iter().enumerate() {
        for second in values.iter().skip(index + 1) {
            if first.starts_with(second) || second.starts_with(first) {
                return Err(invalid_wal("transaction roots overlap"));
            }
        }
    }
    Ok(())
}

fn validate_transaction_id(transaction_id: &str) -> Result<(), InstallerError> {
    if transaction_id.is_empty()
        || !transaction_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(invalid_wal("transaction id contains unsafe characters"));
    }
    Ok(())
}

pub(crate) fn invalid_wal(message: impl Into<String>) -> InstallerError {
    InstallerError::InvalidWal {
        message: message.into(),
    }
}
