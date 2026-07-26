use std::io;
use std::path::Path;

use crate::InstallerError;
use crate::path::{Locator, RootId, validate_destination_ancestors};
use crate::platform::{EntryKind, Platform};

use super::model::{MoveIntent, WalDocument};
use super::wal::WalStore;

pub(crate) fn move_with_intent<P: Platform>(
    platform: &P,
    store: &WalStore<'_, P>,
    wal: &mut WalDocument,
    intent: MoveIntent,
) -> Result<(), InstallerError> {
    validate_locator_ancestors(wal, &intent.source)?;
    validate_locator_ancestors(wal, &intent.destination)?;
    let mut next = wal.clone();
    next.pending_move = Some(intent.clone());
    store.replace(wal, next)?;

    let source_path = wal.roots.resolve(&intent.source);
    let destination_path = wal.roots.resolve(&intent.destination);
    platform
        .rename_exclusive(&source_path, &destination_path)
        .map_err(|error| filesystem_error("move transaction entry", &source_path, error))?;
    sync_move_parents(platform, &source_path, &destination_path)?;

    finish_intent(store, wal, &intent)
}

fn validate_locator_ancestors(wal: &WalDocument, locator: &Locator) -> Result<(), InstallerError> {
    let root = match locator.root {
        RootId::CodexHome => &wal.roots.codex_home,
        RootId::SkillsHome => &wal.roots.skills_home,
        RootId::StateDir => &wal.roots.state_dir,
    };
    validate_destination_ancestors(root, &locator.relative)
}

pub(crate) fn resolve_pending<P: Platform>(
    platform: &P,
    store: &WalStore<'_, P>,
    wal: &mut WalDocument,
) -> Result<(), InstallerError> {
    let Some(intent) = wal.pending_move.clone() else {
        return Ok(());
    };
    let source = wal.roots.resolve(&intent.source);
    let destination = wal.roots.resolve(&intent.destination);
    let source_kind = platform
        .no_follow_kind(&source)
        .map_err(|error| filesystem_error("inspect pending move source", &source, error))?;
    let destination_kind = platform.no_follow_kind(&destination).map_err(|error| {
        filesystem_error("inspect pending move destination", &destination, error)
    })?;
    match (
        is_ordinary(source_kind),
        is_ordinary(destination_kind),
        source_kind,
        destination_kind,
    ) {
        (true, false, _, None) => {
            let mut next = wal.clone();
            next.pending_move = None;
            store.replace(wal, next)
        }
        (false, true, None, _) => {
            sync_move_parents(platform, &source, &destination)?;
            finish_intent(store, wal, &intent)
        }
        _ => Err(InstallerError::UnclassifiableTransaction {
            wal: store.canonical_path().to_owned(),
            paths: vec![source, destination],
            message: "pending move has neither exactly one ordinary source nor destination"
                .to_owned(),
        }),
    }
}

fn finish_intent<P: Platform>(
    store: &WalStore<'_, P>,
    wal: &mut WalDocument,
    intent: &MoveIntent,
) -> Result<(), InstallerError> {
    let mut next = wal.clone();
    next.entries[intent.entry_index].phase = intent.target_phase;
    next.pending_move = None;
    store.replace(wal, next)
}

fn is_ordinary(kind: Option<EntryKind>) -> bool {
    matches!(kind, Some(EntryKind::File | EntryKind::Directory))
}

fn sync_move_parents<P: Platform>(
    platform: &P,
    source: &Path,
    destination: &Path,
) -> Result<(), InstallerError> {
    let source_parent = source
        .parent()
        .ok_or_else(|| transaction_error("move source has no parent"))?;
    let destination_parent = destination
        .parent()
        .ok_or_else(|| transaction_error("move destination has no parent"))?;
    platform.sync_directory(source_parent).map_err(|error| {
        filesystem_error("synchronize move source parent", source_parent, error)
    })?;
    if source_parent != destination_parent {
        platform
            .sync_directory(destination_parent)
            .map_err(|error| {
                filesystem_error(
                    "synchronize move destination parent",
                    destination_parent,
                    error,
                )
            })?;
    }
    Ok(())
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
