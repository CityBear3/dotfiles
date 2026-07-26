use std::io;
use std::path::Path;

use crate::InstallerError;
use crate::content::capture_optional;
use crate::path::validate_destination_ancestors;
use crate::platform::{EntryKind, Platform};

use super::model::{
    EntryOperation, EntryPhase, MoveKind, RecoveryOutcome, TransactionPhase, WalDocument,
    unclassifiable_entry,
};
use super::move_protocol::{move_with_intent, resolve_pending};
use super::wal::WalStore;

pub(crate) fn recover<P: Platform>(
    platform: &P,
    state_dir: &Path,
) -> Result<RecoveryOutcome, InstallerError> {
    match platform
        .no_follow_kind(state_dir)
        .map_err(|error| filesystem_error("inspect state directory", state_dir, error))?
    {
        None => return Ok(RecoveryOutcome::NoTransaction),
        Some(EntryKind::Directory) => {}
        Some(_) => {
            return Err(super::model::invalid_wal(
                "state path is not an ordinary directory",
            ));
        }
    }
    let store = WalStore::open(platform, state_dir, false)?;
    let Some(mut wal) = store.load()? else {
        return Ok(RecoveryOutcome::NoTransaction);
    };
    preflight_roots_and_live_ancestors(platform, &wal)?;
    resolve_pending(platform, &store, &mut wal)?;
    match wal.phase {
        TransactionPhase::Committed | TransactionPhase::CleaningUp | TransactionPhase::Complete => {
            let transaction_id = wal.transaction_id.clone();
            cleanup_committed(platform, &store, &mut wal)?;
            Ok(RecoveryOutcome::CleanedCommitted { transaction_id })
        }
        TransactionPhase::Planned
        | TransactionPhase::Preparing
        | TransactionPhase::Prepared
        | TransactionPhase::Applying
        | TransactionPhase::RollingBack
        | TransactionPhase::RolledBack => {
            let transaction_id = wal.transaction_id.clone();
            rollback(platform, &store, &mut wal)?;
            Ok(RecoveryOutcome::RolledBack { transaction_id })
        }
    }
}

pub(crate) fn rollback<P: Platform>(
    platform: &P,
    store: &WalStore<'_, P>,
    wal: &mut WalDocument,
) -> Result<(), InstallerError> {
    resolve_pending(platform, store, wal)?;
    preflight_rollback(platform, store, wal)?;
    if wal.phase != TransactionPhase::RollingBack && wal.phase != TransactionPhase::RolledBack {
        let mut next = wal.clone();
        next.phase = TransactionPhase::RollingBack;
        store.replace(wal, next)?;
    }

    if wal.phase == TransactionPhase::RollingBack {
        for index in (0..wal.entries.len()).rev() {
            rollback_entry(platform, store, wal, index)?;
        }
        let mut next = wal.clone();
        next.phase = TransactionPhase::RolledBack;
        store.replace(wal, next)?;
    }

    cleanup_work(platform, store, wal)?;
    store.remove()
}

pub(crate) fn cleanup_committed<P: Platform>(
    platform: &P,
    store: &WalStore<'_, P>,
    wal: &mut WalDocument,
) -> Result<(), InstallerError> {
    resolve_pending(platform, store, wal)?;
    preflight_committed(platform, store, wal)?;
    if wal.phase != TransactionPhase::CleaningUp && wal.phase != TransactionPhase::Complete {
        let mut next = wal.clone();
        next.phase = TransactionPhase::CleaningUp;
        store.replace(wal, next)?;
    }
    cleanup_work(platform, store, wal)?;
    if wal.phase != TransactionPhase::Complete {
        let mut next = wal.clone();
        next.phase = TransactionPhase::Complete;
        store.replace(wal, next)?;
    }
    store.remove()
}

fn rollback_entry<P: Platform>(
    platform: &P,
    store: &WalStore<'_, P>,
    wal: &mut WalDocument,
    index: usize,
) -> Result<(), InstallerError> {
    let entry = wal.entries[index].clone();
    match (entry.operation, entry.phase) {
        (_, EntryPhase::Planned | EntryPhase::Prepared) => Ok(()),
        (EntryOperation::Create, EntryPhase::Applied) => move_with_intent(
            platform,
            store,
            wal,
            super::model::MoveIntent {
                entry_index: index,
                kind: MoveKind::RollbackDesired,
                source: entry.live,
                destination: entry.stage.expect("validated create stage"),
                target_phase: EntryPhase::Prepared,
            },
        ),
        (EntryOperation::Replace, EntryPhase::Applied) => {
            move_with_intent(
                platform,
                store,
                wal,
                super::model::MoveIntent {
                    entry_index: index,
                    kind: MoveKind::RollbackDesired,
                    source: entry.live.clone(),
                    destination: entry.stage.expect("validated replace stage"),
                    target_phase: EntryPhase::Isolated,
                },
            )?;
            restore_prior(platform, store, wal, index)
        }
        (EntryOperation::Replace, EntryPhase::Isolated) => {
            restore_prior(platform, store, wal, index)
        }
        (EntryOperation::Remove, EntryPhase::Applied) => restore_prior(platform, store, wal, index),
        (EntryOperation::Remove, EntryPhase::Isolated) => {
            restore_prior(platform, store, wal, index)
        }
        (EntryOperation::Create, EntryPhase::Isolated) => Err(unclassifiable(
            store,
            wal,
            index,
            "create cannot be isolated",
        )),
    }
}

fn restore_prior<P: Platform>(
    platform: &P,
    store: &WalStore<'_, P>,
    wal: &mut WalDocument,
    index: usize,
) -> Result<(), InstallerError> {
    let entry = wal.entries[index].clone();
    move_with_intent(
        platform,
        store,
        wal,
        super::model::MoveIntent {
            entry_index: index,
            kind: MoveKind::RestorePrior,
            source: entry.tombstone.expect("validated prior tombstone"),
            destination: entry.live,
            target_phase: EntryPhase::Prepared,
        },
    )
}

fn preflight_rollback<P: Platform>(
    platform: &P,
    store: &WalStore<'_, P>,
    wal: &WalDocument,
) -> Result<(), InstallerError> {
    wal.validate()?;
    preflight_roots_and_live_ancestors(platform, wal)?;
    preflight_work_paths(platform, store, wal)?;
    if wal.phase == TransactionPhase::RolledBack {
        for (index, entry) in wal.entries.iter().enumerate() {
            let live = wal.roots.resolve(&entry.live);
            match entry.operation {
                EntryOperation::Create => expect_absent(store, wal, index, &live)?,
                EntryOperation::Replace | EntryOperation::Remove => expect_fingerprint(
                    store,
                    wal,
                    index,
                    &live,
                    entry.prior_sha256.as_deref().expect("validated prior hash"),
                )?,
            }
        }
        return Ok(());
    }
    for (index, entry) in wal.entries.iter().enumerate() {
        let live = wal.roots.resolve(&entry.live);
        let stage = entry.stage.as_ref().map(|path| wal.roots.resolve(path));
        let tombstone = entry.tombstone.as_ref().map(|path| wal.roots.resolve(path));
        match (entry.operation, entry.phase) {
            (EntryOperation::Create, EntryPhase::Planned) => {
                expect_absent(store, wal, index, &live)?;
            }
            (EntryOperation::Create, EntryPhase::Prepared) => {
                expect_absent(store, wal, index, &live)?;
                expect_fingerprint(
                    store,
                    wal,
                    index,
                    stage.as_deref().expect("validated create stage"),
                    entry
                        .desired_sha256
                        .as_deref()
                        .expect("validated desired hash"),
                )?;
            }
            (EntryOperation::Create, EntryPhase::Applied) => {
                expect_fingerprint(
                    store,
                    wal,
                    index,
                    &live,
                    entry
                        .desired_sha256
                        .as_deref()
                        .expect("validated desired hash"),
                )?;
                expect_absent(
                    store,
                    wal,
                    index,
                    stage.as_deref().expect("validated create stage"),
                )?;
            }
            (EntryOperation::Replace, EntryPhase::Planned) => {
                expect_fingerprint(
                    store,
                    wal,
                    index,
                    &live,
                    entry.prior_sha256.as_deref().expect("validated prior hash"),
                )?;
                expect_absent(
                    store,
                    wal,
                    index,
                    tombstone.as_deref().expect("validated replace tombstone"),
                )?;
            }
            (EntryOperation::Replace, EntryPhase::Prepared) => {
                expect_fingerprint(
                    store,
                    wal,
                    index,
                    &live,
                    entry.prior_sha256.as_deref().expect("validated prior hash"),
                )?;
                expect_fingerprint(
                    store,
                    wal,
                    index,
                    stage.as_deref().expect("validated replace stage"),
                    entry
                        .desired_sha256
                        .as_deref()
                        .expect("validated desired hash"),
                )?;
                expect_absent(
                    store,
                    wal,
                    index,
                    tombstone.as_deref().expect("validated replace tombstone"),
                )?;
            }
            (EntryOperation::Replace, EntryPhase::Isolated) => {
                expect_absent(store, wal, index, &live)?;
                expect_fingerprint(
                    store,
                    wal,
                    index,
                    stage.as_deref().expect("validated replace stage"),
                    entry
                        .desired_sha256
                        .as_deref()
                        .expect("validated desired hash"),
                )?;
                expect_fingerprint(
                    store,
                    wal,
                    index,
                    tombstone.as_deref().expect("validated replace tombstone"),
                    entry.prior_sha256.as_deref().expect("validated prior hash"),
                )?;
            }
            (EntryOperation::Replace, EntryPhase::Applied) => {
                expect_fingerprint(
                    store,
                    wal,
                    index,
                    &live,
                    entry
                        .desired_sha256
                        .as_deref()
                        .expect("validated desired hash"),
                )?;
                expect_absent(
                    store,
                    wal,
                    index,
                    stage.as_deref().expect("validated replace stage"),
                )?;
                expect_fingerprint(
                    store,
                    wal,
                    index,
                    tombstone.as_deref().expect("validated replace tombstone"),
                    entry.prior_sha256.as_deref().expect("validated prior hash"),
                )?;
            }
            (EntryOperation::Remove, EntryPhase::Planned | EntryPhase::Prepared) => {
                expect_fingerprint(
                    store,
                    wal,
                    index,
                    &live,
                    entry.prior_sha256.as_deref().expect("validated prior hash"),
                )?;
                expect_absent(
                    store,
                    wal,
                    index,
                    tombstone.as_deref().expect("validated remove tombstone"),
                )?;
            }
            (EntryOperation::Remove, EntryPhase::Isolated | EntryPhase::Applied) => {
                expect_absent(store, wal, index, &live)?;
                expect_fingerprint(
                    store,
                    wal,
                    index,
                    tombstone.as_deref().expect("validated remove tombstone"),
                    entry.prior_sha256.as_deref().expect("validated prior hash"),
                )?;
            }
            (EntryOperation::Create, EntryPhase::Isolated) => {
                return Err(unclassifiable(
                    store,
                    wal,
                    index,
                    "create cannot be isolated",
                ));
            }
        }
    }
    Ok(())
}

fn preflight_committed<P: Platform>(
    platform: &P,
    store: &WalStore<'_, P>,
    wal: &WalDocument,
) -> Result<(), InstallerError> {
    wal.validate()?;
    preflight_roots_and_live_ancestors(platform, wal)?;
    preflight_work_paths(platform, store, wal)?;
    for (index, entry) in wal.entries.iter().enumerate() {
        let live = wal.roots.resolve(&entry.live);
        match entry.operation {
            EntryOperation::Create | EntryOperation::Replace => {
                expect_fingerprint(
                    store,
                    wal,
                    index,
                    &live,
                    entry
                        .desired_sha256
                        .as_deref()
                        .expect("validated desired hash"),
                )?;
                if wal.phase == TransactionPhase::Committed {
                    expect_absent(
                        store,
                        wal,
                        index,
                        &wal.roots
                            .resolve(entry.stage.as_ref().expect("validated committed stage")),
                    )?;
                    if entry.operation == EntryOperation::Replace {
                        expect_fingerprint(
                            store,
                            wal,
                            index,
                            &wal.roots.resolve(
                                entry
                                    .tombstone
                                    .as_ref()
                                    .expect("validated committed tombstone"),
                            ),
                            entry.prior_sha256.as_deref().expect("validated prior hash"),
                        )?;
                    }
                }
            }
            EntryOperation::Remove => {
                expect_absent(store, wal, index, &live)?;
                if wal.phase == TransactionPhase::Committed {
                    expect_fingerprint(
                        store,
                        wal,
                        index,
                        &wal.roots.resolve(
                            entry
                                .tombstone
                                .as_ref()
                                .expect("validated committed tombstone"),
                        ),
                        entry.prior_sha256.as_deref().expect("validated prior hash"),
                    )?;
                }
            }
        }
    }
    Ok(())
}

fn preflight_roots_and_live_ancestors<P: Platform>(
    platform: &P,
    wal: &WalDocument,
) -> Result<(), InstallerError> {
    for root in [
        &wal.roots.codex_home,
        &wal.roots.skills_home,
        &wal.roots.state_dir,
    ] {
        match platform
            .no_follow_kind(root)
            .map_err(|error| filesystem_error("inspect transaction root", root, error))?
        {
            Some(EntryKind::Directory) => {}
            _ => {
                return Err(InstallerError::UnclassifiableTransaction {
                    transaction_id: wal.transaction_id.clone(),
                    wal: wal.roots.state_dir.join("transaction/wal-v1.json"),
                    paths: vec![root.to_path_buf()],
                    message: "transaction root is not an ordinary directory".to_owned(),
                });
            }
        }
    }
    for (index, entry) in wal.entries.iter().enumerate() {
        let root = match entry.live.root {
            crate::path::RootId::CodexHome => &wal.roots.codex_home,
            crate::path::RootId::SkillsHome => &wal.roots.skills_home,
            crate::path::RootId::StateDir => &wal.roots.state_dir,
        };
        if let Err(error) = validate_destination_ancestors(root, &entry.live.relative) {
            return Err(unclassifiable_entry(
                &wal.roots.state_dir.join("transaction/wal-v1.json"),
                wal,
                index,
                format!("live destination ancestor is unsafe: {error}"),
            ));
        }
    }
    Ok(())
}

fn preflight_work_paths<P: Platform>(
    platform: &P,
    store: &WalStore<'_, P>,
    wal: &WalDocument,
) -> Result<(), InstallerError> {
    for (index, entry) in wal.entries.iter().enumerate() {
        for locator in [entry.stage.as_ref(), entry.tombstone.as_ref()]
            .into_iter()
            .flatten()
        {
            let path = wal.roots.resolve(locator);
            match platform
                .no_follow_kind(&path)
                .map_err(|error| filesystem_error("inspect transaction work path", &path, error))?
            {
                None | Some(EntryKind::File | EntryKind::Directory) => {}
                Some(_) => {
                    return Err(unclassifiable_entry(
                        store.canonical_path(),
                        wal,
                        index,
                        "transaction work path is not ordinary",
                    ));
                }
            }
        }
    }
    Ok(())
}

fn cleanup_work<P: Platform>(
    platform: &P,
    store: &WalStore<'_, P>,
    wal: &WalDocument,
) -> Result<(), InstallerError> {
    preflight_work_paths(platform, store, wal)?;
    let work_root = wal.work_root();
    platform
        .cleanup_owned_tree(&work_root)
        .map_err(|error| filesystem_error("clean transaction work tree", &work_root, error))
}

fn expect_fingerprint<P: Platform>(
    store: &WalStore<'_, P>,
    wal: &WalDocument,
    index: usize,
    path: &Path,
    expected: &str,
) -> Result<(), InstallerError> {
    match capture_optional(path) {
        Ok(Some(content)) if content.sha256 == expected => Ok(()),
        _ => Err(unclassifiable(
            store,
            wal,
            index,
            "required transaction content is absent, unsafe, or has changed",
        )),
    }
}

fn expect_absent<P: Platform>(
    store: &WalStore<'_, P>,
    wal: &WalDocument,
    index: usize,
    path: &Path,
) -> Result<(), InstallerError> {
    match capture_optional(path) {
        Ok(None) => Ok(()),
        _ => Err(unclassifiable(
            store,
            wal,
            index,
            "transaction path expected to be absent is present or unsafe",
        )),
    }
}

fn unclassifiable<P: Platform>(
    store: &WalStore<'_, P>,
    wal: &WalDocument,
    index: usize,
    message: &str,
) -> InstallerError {
    unclassifiable_entry(store.canonical_path(), wal, index, message)
}

fn filesystem_error(operation: &str, path: &Path, error: io::Error) -> InstallerError {
    InstallerError::Filesystem {
        message: format!("{operation} {}: {error}", path.display()),
    }
}
