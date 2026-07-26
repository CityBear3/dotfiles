use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::content::CapturedContent;
use crate::path::{InstallRoots, Locator, RootId};
use crate::plan::{AssetCategory, InstallPlan, PlanAction, PlanOperation};
use crate::platform::macos::MacOsPlatform;
use crate::platform::{EntryKind, Platform};
use crate::test_support::project_tempdir;

use super::model::{EntryPhase, MoveIntent, MoveKind, TransactionPhase, WalDocument};
use super::{FaultPoint, RecoveryOutcome, TransactionEngine, TransactionOutcome};

#[test]
fn pre_commit_restart_rolls_back_prior_state() {
    // Arrange
    let temporary = project_tempdir("transaction-pre-commit-restart");
    let source_root = temporary.path().join("source");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    fs::create_dir(&source_root).expect("create source root");
    fs::create_dir(&codex_home).expect("create Codex home");
    fs::create_dir(&skills_home).expect("create skills home");
    fs::create_dir(&state_dir).expect("create state directory");
    fs::write(codex_home.join("config.toml"), b"prior config").expect("write prior config");
    fs::write(state_dir.join("manifest-v1.json"), b"prior manifest").expect("write prior manifest");
    let roots = InstallRoots {
        source_root,
        codex_home: codex_home.clone(),
        skills_home,
        state_dir: state_dir.clone(),
    };
    let plan = InstallPlan {
        roots,
        max_threads: 6,
        actions: vec![
            PlanAction {
                operation: PlanOperation::Replace,
                category: AssetCategory::Config,
                name: None,
                locator: Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
                desired: Some(CapturedContent::file(b"desired config".to_vec())),
            },
            PlanAction {
                operation: PlanOperation::Replace,
                category: AssetCategory::Manifest,
                name: None,
                locator: Locator::new(RootId::StateDir, "manifest-v1.json")
                    .expect("manifest locator"),
                desired: Some(CapturedContent::file(b"desired manifest".to_vec())),
            },
        ],
    };
    let engine = TransactionEngine::new(MacOsPlatform::new());
    let interrupted = engine.execute(
        &plan,
        "pre-commit-restart",
        FaultPoint::AfterFirstLiveMutationBeforeCommit,
    );
    assert!(interrupted.is_err(), "fault must interrupt the transaction");
    let fresh = TransactionEngine::new(MacOsPlatform::new());

    // Act
    let result = fresh.recover(&state_dir);

    // Assert
    assert_eq!(
        result,
        Ok(RecoveryOutcome::RolledBack {
            transaction_id: "pre-commit-restart".to_owned(),
        })
    );
    assert_eq!(
        (
            fs::read(codex_home.join("config.toml")).expect("read restored config"),
            fs::read(state_dir.join("manifest-v1.json")).expect("read prior manifest"),
            state_dir.join("transaction/wal-v1.json").exists(),
            state_dir
                .join("transaction/work/pre-commit-restart")
                .exists(),
        ),
        (
            b"prior config".to_vec(),
            b"prior manifest".to_vec(),
            false,
            false,
        )
    );
}

#[test]
fn create_commits_the_desired_content_and_removes_transaction_state() {
    // Arrange
    let temporary = project_tempdir("transaction-create");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    let plan = single_action_plan(
        &roots,
        PlanOperation::Create,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"desired".to_vec())),
    );

    // Act
    let result = TransactionEngine::new(MacOsPlatform::new()).execute(
        &plan,
        "create-success",
        FaultPoint::None,
    );

    // Assert
    assert_eq!(
        result,
        Ok(TransactionOutcome {
            transaction_id: "create-success".to_owned(),
            applied_entries: 1,
        })
    );
    assert_eq!(fs::read(destination).expect("read destination"), b"desired");
    assert_transaction_state_absent(&roots.state_dir, "create-success");
}

#[test]
fn replace_commits_the_desired_content_and_removes_prior_content() {
    // Arrange
    let temporary = project_tempdir("transaction-replace");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    fs::write(&destination, b"prior").expect("write prior");
    let plan = single_action_plan(
        &roots,
        PlanOperation::Replace,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"desired".to_vec())),
    );

    // Act
    let result = TransactionEngine::new(MacOsPlatform::new()).execute(
        &plan,
        "replace-success",
        FaultPoint::None,
    );

    // Assert
    assert!(result.is_ok());
    assert_eq!(fs::read(destination).expect("read destination"), b"desired");
    assert_transaction_state_absent(&roots.state_dir, "replace-success");
}

#[test]
fn remove_commits_absence_and_removes_prior_content() {
    // Arrange
    let temporary = project_tempdir("transaction-remove");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("AGENTS.md");
    fs::write(&destination, b"prior").expect("write prior");
    let plan = single_action_plan(
        &roots,
        PlanOperation::Remove,
        AssetCategory::GlobalAgents,
        Locator::new(RootId::CodexHome, "AGENTS.md").expect("global agents locator"),
        None,
    );

    // Act
    let result = TransactionEngine::new(MacOsPlatform::new()).execute(
        &plan,
        "remove-success",
        FaultPoint::None,
    );

    // Assert
    assert!(result.is_ok());
    assert!(!destination.exists());
    assert_transaction_state_absent(&roots.state_dir, "remove-success");
}

#[test]
fn committed_restart_keeps_installed_state_and_finishes_cleanup() {
    // Arrange
    let temporary = project_tempdir("transaction-committed-restart");
    let roots = create_roots(temporary.path());
    let config = roots.codex_home.join("config.toml");
    let manifest = roots.state_dir.join("manifest-v1.json");
    fs::write(&config, b"prior config").expect("write prior config");
    fs::write(&manifest, b"prior manifest").expect("write prior manifest");
    let plan = replace_config_and_manifest_plan(&roots);
    let engine = TransactionEngine::new(MacOsPlatform::new());
    let interrupted = engine.execute(
        &plan,
        "committed-restart",
        FaultPoint::AfterCommittedBeforeCleanup,
    );
    assert!(matches!(
        interrupted,
        Err(crate::InstallerError::InjectedTransactionFault { .. })
    ));

    // Act
    let result = TransactionEngine::new(MacOsPlatform::new()).recover(&roots.state_dir);

    // Assert
    assert_eq!(
        result,
        Ok(RecoveryOutcome::CleanedCommitted {
            transaction_id: "committed-restart".to_owned(),
        })
    );
    assert_eq!(fs::read(config).expect("read config"), b"desired config");
    assert_eq!(
        fs::read(manifest).expect("read manifest"),
        b"desired manifest"
    );
    assert_transaction_state_absent(&roots.state_dir, "committed-restart");
}

#[test]
fn ordinary_temporary_wal_is_discarded_and_canonical_wal_remains_authoritative() {
    // Arrange
    let temporary = project_tempdir("transaction-single-authority");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    fs::write(&destination, b"prior").expect("write prior");
    let plan = single_action_plan(
        &roots,
        PlanOperation::Replace,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"desired".to_vec())),
    );
    let engine = TransactionEngine::new(MacOsPlatform::new());
    assert!(
        engine
            .execute(
                &plan,
                "single-authority",
                FaultPoint::AfterCommittedBeforeCleanup,
            )
            .is_err()
    );
    let temporary_wal = roots.state_dir.join("transaction/wal-v1.json.tmp");
    fs::write(&temporary_wal, b"not an alternate authority").expect("write stale temp WAL");

    // Act
    let result = TransactionEngine::new(MacOsPlatform::new()).recover(&roots.state_dir);

    // Assert
    assert_eq!(
        result,
        Ok(RecoveryOutcome::CleanedCommitted {
            transaction_id: "single-authority".to_owned(),
        })
    );
    assert_eq!(fs::read(destination).expect("read destination"), b"desired");
    assert!(!temporary_wal.exists());
}

#[test]
fn unknown_wal_version_fails_before_mutating_referenced_paths() {
    // Arrange
    let temporary = project_tempdir("transaction-unknown-wal");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    fs::write(&destination, b"prior").expect("write prior");
    let plan = single_action_plan(
        &roots,
        PlanOperation::Replace,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"desired".to_vec())),
    );
    assert!(
        TransactionEngine::new(MacOsPlatform::new())
            .execute(
                &plan,
                "unknown-version",
                FaultPoint::AfterFirstLiveMutationBeforeCommit,
            )
            .is_err()
    );
    let wal_path = roots.state_dir.join("transaction/wal-v1.json");
    let mut wal: serde_json::Value =
        serde_json::from_slice(&fs::read(&wal_path).expect("read WAL")).expect("parse WAL");
    wal["version"] = serde_json::json!(2);
    fs::write(
        &wal_path,
        serde_json::to_vec_pretty(&wal).expect("encode WAL"),
    )
    .expect("write unknown WAL");

    // Act
    let result = TransactionEngine::new(MacOsPlatform::new()).recover(&roots.state_dir);

    // Assert
    assert!(matches!(
        result,
        Err(crate::InstallerError::InvalidWal { .. })
    ));
    assert_eq!(fs::read(destination).expect("read destination"), b"desired");
    assert!(wal_path.exists());
}

#[test]
fn corrupt_canonical_wal_fails_without_mutating_live_content() {
    // Arrange
    let temporary = project_tempdir("transaction-corrupt-wal");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    fs::write(&destination, b"live").expect("write live");
    let transaction = roots.state_dir.join("transaction");
    fs::create_dir(&transaction).expect("create transaction directory");
    let wal_path = transaction.join("wal-v1.json");
    fs::write(&wal_path, b"{corrupt").expect("write corrupt WAL");

    // Act
    let result = TransactionEngine::new(MacOsPlatform::new()).recover(&roots.state_dir);

    // Assert
    assert!(matches!(
        result,
        Err(crate::InstallerError::InvalidWal { .. })
    ));
    assert_eq!(fs::read(destination).expect("read live"), b"live");
    assert!(wal_path.exists());
}

#[test]
fn unclassifiable_pending_move_leaves_wal_and_both_paths_untouched() {
    // Arrange
    let temporary = project_tempdir("transaction-unclassifiable");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    fs::write(&destination, b"prior").expect("write prior");
    let plan = single_action_plan(
        &roots,
        PlanOperation::Replace,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"desired".to_vec())),
    );
    assert!(
        TransactionEngine::new(MacOsPlatform::new())
            .execute(
                &plan,
                "unclassifiable",
                FaultPoint::AfterFirstLiveMutationBeforeCommit,
            )
            .is_err()
    );
    let wal_path = roots.state_dir.join("transaction/wal-v1.json");
    let mut wal: WalDocument =
        serde_json::from_slice(&fs::read(&wal_path).expect("read WAL")).expect("parse WAL");
    let entry = wal.entries[0].clone();
    let stage_locator = entry.stage.expect("replace stage");
    let stage = wal.roots.resolve(&stage_locator);
    fs::write(&stage, b"desired").expect("create ambiguous stage");
    wal.entries[0].phase = EntryPhase::Isolated;
    wal.pending_move = Some(MoveIntent {
        entry_index: 0,
        kind: MoveKind::Install,
        source: stage_locator,
        destination: entry.live,
        target_phase: EntryPhase::Applied,
    });
    fs::write(
        &wal_path,
        serde_json::to_vec_pretty(&wal).expect("encode WAL"),
    )
    .expect("write ambiguous WAL");

    // Act
    let result = TransactionEngine::new(MacOsPlatform::new()).recover(&roots.state_dir);

    // Assert
    assert!(matches!(
        result,
        Err(crate::InstallerError::UnclassifiableTransaction { .. })
    ));
    assert_eq!(fs::read(destination).expect("read live"), b"desired");
    assert_eq!(fs::read(stage).expect("read stage"), b"desired");
    assert!(wal_path.exists());
}

#[test]
fn every_live_rename_observes_a_durable_pending_intent_and_manifest_is_last() {
    // Arrange
    let temporary = project_tempdir("transaction-intent-order");
    let roots = create_roots(temporary.path());
    fs::write(roots.codex_home.join("config.toml"), b"prior config").expect("write prior config");
    fs::write(roots.state_dir.join("manifest-v1.json"), b"prior manifest")
        .expect("write prior manifest");
    let plan = replace_config_and_manifest_plan(&roots);
    let observations = Arc::new(Mutex::new(Vec::new()));
    let platform = InspectingPlatform {
        delegate: MacOsPlatform::new(),
        wal_path: roots.state_dir.join("transaction/wal-v1.json"),
        live_destinations: vec![
            roots.codex_home.join("config.toml"),
            roots.state_dir.join("manifest-v1.json"),
        ],
        observations: Arc::clone(&observations),
    };

    // Act
    let result = TransactionEngine::new(platform).execute(&plan, "intent-order", FaultPoint::None);

    // Assert
    assert!(result.is_ok());
    let observed = observations.lock().expect("lock observations");
    assert!(observed.iter().all(|observation| observation.pending));
    assert_eq!(
        observed
            .iter()
            .filter_map(|observation| observation.live_destination.clone())
            .collect::<Vec<_>>(),
        vec![
            roots.codex_home.join("config.toml"),
            roots.state_dir.join("manifest-v1.json"),
        ]
    );
}

#[test]
fn ordinary_rename_failure_uses_pre_commit_rollback_rules() {
    // Arrange
    let temporary = project_tempdir("transaction-rename-failure");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    fs::write(&destination, b"prior").expect("write prior");
    let plan = single_action_plan(
        &roots,
        PlanOperation::Replace,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"desired".to_vec())),
    );
    let platform = FailFirstRenamePlatform {
        delegate: MacOsPlatform::new(),
        failed: AtomicBool::new(false),
    };

    // Act
    let result =
        TransactionEngine::new(platform).execute(&plan, "rename-failure", FaultPoint::None);

    // Assert
    assert!(matches!(
        result,
        Err(crate::InstallerError::Filesystem { .. })
    ));
    assert_eq!(fs::read(destination).expect("read restored live"), b"prior");
    assert_transaction_state_absent(&roots.state_dir, "rename-failure");
}

#[test]
fn completed_transactions_can_reuse_the_persistent_work_parent() {
    // Arrange
    let temporary = project_tempdir("transaction-reuse-work-parent");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    let create = single_action_plan(
        &roots,
        PlanOperation::Create,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"first".to_vec())),
    );
    let engine = TransactionEngine::new(MacOsPlatform::new());
    engine
        .execute(&create, "first-transaction", FaultPoint::None)
        .expect("complete first transaction");
    let replace = single_action_plan(
        &roots,
        PlanOperation::Replace,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"second".to_vec())),
    );

    // Act
    let result = engine.execute(&replace, "second-transaction", FaultPoint::None);

    // Assert
    assert!(result.is_ok());
    assert_eq!(fs::read(destination).expect("read destination"), b"second");
    assert_transaction_state_absent(&roots.state_dir, "second-transaction");
}

#[test]
fn recovery_treats_an_absent_state_directory_as_no_transaction() {
    // Arrange
    let temporary = project_tempdir("transaction-absent-state");
    let state_dir = temporary.path().join("absent-state");

    // Act
    let result = TransactionEngine::new(MacOsPlatform::new()).recover(&state_dir);

    // Assert
    assert_eq!(result, Ok(RecoveryOutcome::NoTransaction));
    assert!(!state_dir.exists());
}

#[test]
fn semantically_invalid_live_locator_fails_before_mutation() {
    // Arrange
    let temporary = project_tempdir("transaction-invalid-locator");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    fs::write(&destination, b"prior").expect("write prior");
    let plan = single_action_plan(
        &roots,
        PlanOperation::Replace,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"desired".to_vec())),
    );
    assert!(
        TransactionEngine::new(MacOsPlatform::new())
            .execute(
                &plan,
                "invalid-locator",
                FaultPoint::AfterFirstLiveMutationBeforeCommit,
            )
            .is_err()
    );
    let wal_path = roots.state_dir.join("transaction/wal-v1.json");
    let mut wal: serde_json::Value =
        serde_json::from_slice(&fs::read(&wal_path).expect("read WAL")).expect("parse WAL");
    wal["entries"][0]["live"]["relative"] = serde_json::json!("AGENTS.md");
    fs::write(
        &wal_path,
        serde_json::to_vec_pretty(&wal).expect("encode WAL"),
    )
    .expect("write invalid WAL");

    // Act
    let result = TransactionEngine::new(MacOsPlatform::new()).recover(&roots.state_dir);

    // Assert
    assert!(matches!(
        result,
        Err(crate::InstallerError::InvalidWal { .. })
    ));
    assert_eq!(fs::read(destination).expect("read destination"), b"desired");
    assert!(wal_path.exists());
}

#[test]
fn synchronization_error_after_durable_commit_never_rolls_back_installed_state() {
    // Arrange
    let temporary = project_tempdir("transaction-commit-sync-failure");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    fs::write(&destination, b"prior").expect("write prior");
    let plan = single_action_plan(
        &roots,
        PlanOperation::Replace,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"desired".to_vec())),
    );
    let platform = FailCommittedWalSyncPlatform {
        delegate: MacOsPlatform::new(),
        transaction_dir: roots.state_dir.join("transaction"),
        wal_path: roots.state_dir.join("transaction/wal-v1.json"),
        failed: AtomicBool::new(false),
    };

    // Act
    let result =
        TransactionEngine::new(platform).execute(&plan, "commit-sync-failure", FaultPoint::None);

    // Assert
    assert!(matches!(
        result,
        Err(crate::InstallerError::Filesystem { .. })
    ));
    assert_eq!(
        fs::read(destination).expect("read committed live"),
        b"desired"
    );
    assert_transaction_state_absent(&roots.state_dir, "commit-sync-failure");
}

#[test]
fn failed_authority_reload_prohibits_further_transaction_mutation() {
    // Arrange
    let temporary = project_tempdir("transaction-unresolved-authority");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    fs::write(&destination, b"prior").expect("write prior");
    let plan = single_action_plan(
        &roots,
        PlanOperation::Replace,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"desired".to_vec())),
    );
    let platform = FailWalSyncAndReloadPlatform {
        delegate: MacOsPlatform::new(),
        transaction_dir: roots.state_dir.join("transaction"),
        wal_path: roots.state_dir.join("transaction/wal-v1.json"),
        failed_sync: AtomicBool::new(false),
        authority_unreadable: AtomicBool::new(false),
    };

    // Act
    let result =
        TransactionEngine::new(platform).execute(&plan, "unresolved-authority", FaultPoint::None);

    // Assert
    assert!(matches!(
        result,
        Err(crate::InstallerError::UnresolvedWalAuthority { .. })
    ));
    assert_eq!(
        fs::read(destination).expect("read untouched live"),
        b"prior"
    );
    assert!(roots.state_dir.join("transaction/wal-v1.json").exists());
    assert!(
        roots
            .state_dir
            .join("transaction/work/unresolved-authority")
            .exists()
    );
}

#[test]
fn cleaning_up_restart_accepts_an_already_removed_work_tree() {
    // Arrange
    let temporary = project_tempdir("transaction-cleanup-restart");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    fs::write(&destination, b"prior").expect("write prior");
    let plan = single_action_plan(
        &roots,
        PlanOperation::Replace,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"desired".to_vec())),
    );
    assert!(
        TransactionEngine::new(MacOsPlatform::new())
            .execute(
                &plan,
                "cleanup-restart",
                FaultPoint::AfterCommittedBeforeCleanup,
            )
            .is_err()
    );
    let wal_path = roots.state_dir.join("transaction/wal-v1.json");
    let mut wal: WalDocument =
        serde_json::from_slice(&fs::read(&wal_path).expect("read WAL")).expect("parse WAL");
    wal.phase = TransactionPhase::CleaningUp;
    fs::write(
        &wal_path,
        serde_json::to_vec_pretty(&wal).expect("encode WAL"),
    )
    .expect("write cleaning-up WAL");
    MacOsPlatform::new()
        .cleanup_owned_tree(&wal.work_root())
        .expect("simulate completed work cleanup");

    // Act
    let result = TransactionEngine::new(MacOsPlatform::new()).recover(&roots.state_dir);

    // Assert
    assert_eq!(
        result,
        Ok(RecoveryOutcome::CleanedCommitted {
            transaction_id: "cleanup-restart".to_owned(),
        })
    );
    assert_eq!(
        fs::read(destination).expect("read committed live"),
        b"desired"
    );
    assert!(!wal_path.exists());
}

#[test]
fn rolled_back_restart_accepts_an_already_removed_work_tree() {
    // Arrange
    let temporary = project_tempdir("transaction-rolled-back-restart");
    let roots = create_roots(temporary.path());
    let destination = roots.codex_home.join("config.toml");
    fs::write(&destination, b"prior").expect("write prior");
    let plan = single_action_plan(
        &roots,
        PlanOperation::Replace,
        AssetCategory::Config,
        Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
        Some(CapturedContent::file(b"desired".to_vec())),
    );
    assert!(
        TransactionEngine::new(MacOsPlatform::new())
            .execute(
                &plan,
                "rolled-back-restart",
                FaultPoint::AfterFirstLiveMutationBeforeCommit,
            )
            .is_err()
    );
    let wal_path = roots.state_dir.join("transaction/wal-v1.json");
    let mut wal: WalDocument =
        serde_json::from_slice(&fs::read(&wal_path).expect("read WAL")).expect("parse WAL");
    let stage = wal
        .roots
        .resolve(wal.entries[0].stage.as_ref().expect("replace stage"));
    let tombstone = wal.roots.resolve(
        wal.entries[0]
            .tombstone
            .as_ref()
            .expect("replace tombstone"),
    );
    MacOsPlatform::new()
        .rename_exclusive(&destination, &stage)
        .expect("simulate desired rollback");
    MacOsPlatform::new()
        .rename_exclusive(&tombstone, &destination)
        .expect("simulate prior restore");
    wal.entries[0].phase = EntryPhase::Prepared;
    wal.phase = TransactionPhase::RolledBack;
    fs::write(
        &wal_path,
        serde_json::to_vec_pretty(&wal).expect("encode WAL"),
    )
    .expect("write rolled-back WAL");
    MacOsPlatform::new()
        .cleanup_owned_tree(&wal.work_root())
        .expect("simulate completed rollback cleanup");

    // Act
    let result = TransactionEngine::new(MacOsPlatform::new()).recover(&roots.state_dir);

    // Assert
    assert_eq!(
        result,
        Ok(RecoveryOutcome::RolledBack {
            transaction_id: "rolled-back-restart".to_owned(),
        })
    );
    assert_eq!(fs::read(destination).expect("read restored live"), b"prior");
    assert!(!wal_path.exists());
}

fn create_roots(base: &Path) -> InstallRoots {
    let source_root = base.join("source");
    let codex_home = base.join("codex-home");
    let skills_home = base.join("skills-home");
    let state_dir = base.join("state");
    for directory in [&source_root, &codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create test root");
    }
    InstallRoots {
        source_root,
        codex_home,
        skills_home,
        state_dir,
    }
}

fn single_action_plan(
    roots: &InstallRoots,
    operation: PlanOperation,
    category: AssetCategory,
    locator: Locator,
    desired: Option<CapturedContent>,
) -> InstallPlan {
    InstallPlan {
        roots: roots.clone(),
        max_threads: 6,
        actions: vec![PlanAction {
            operation,
            category,
            name: None,
            locator,
            desired,
        }],
    }
}

fn replace_config_and_manifest_plan(roots: &InstallRoots) -> InstallPlan {
    InstallPlan {
        roots: roots.clone(),
        max_threads: 6,
        actions: vec![
            PlanAction {
                operation: PlanOperation::Replace,
                category: AssetCategory::Config,
                name: None,
                locator: Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
                desired: Some(CapturedContent::file(b"desired config".to_vec())),
            },
            PlanAction {
                operation: PlanOperation::Replace,
                category: AssetCategory::Manifest,
                name: None,
                locator: Locator::new(RootId::StateDir, "manifest-v1.json")
                    .expect("manifest locator"),
                desired: Some(CapturedContent::file(b"desired manifest".to_vec())),
            },
        ],
    }
}

fn assert_transaction_state_absent(state_dir: &Path, transaction_id: &str) {
    assert!(!state_dir.join("transaction/wal-v1.json").exists());
    assert!(
        !state_dir
            .join("transaction/work")
            .join(transaction_id)
            .exists()
    );
}

#[derive(Clone, Debug)]
struct RenameObservation {
    pending: bool,
    live_destination: Option<PathBuf>,
}

struct InspectingPlatform {
    delegate: MacOsPlatform,
    wal_path: PathBuf,
    live_destinations: Vec<PathBuf>,
    observations: Arc<Mutex<Vec<RenameObservation>>>,
}

impl Platform for InspectingPlatform {
    fn no_follow_kind(&self, path: &Path) -> io::Result<Option<EntryKind>> {
        self.delegate.no_follow_kind(path)
    }

    fn rename_exclusive(&self, source: &Path, destination: &Path) -> io::Result<()> {
        let wal: WalDocument =
            serde_json::from_slice(&fs::read(&self.wal_path)?).map_err(io::Error::other)?;
        let pending = wal.pending_move.as_ref().is_some_and(|intent| {
            wal.roots.resolve(&intent.source) == source
                && wal.roots.resolve(&intent.destination) == destination
        });
        let live_destination = self
            .live_destinations
            .contains(&destination.to_owned())
            .then(|| destination.to_owned());
        self.observations
            .lock()
            .expect("lock observations")
            .push(RenameObservation {
                pending,
                live_destination,
            });
        self.delegate.rename_exclusive(source, destination)
    }

    fn sync_file(&self, path: &Path) -> io::Result<()> {
        self.delegate.sync_file(path)
    }

    fn sync_directory(&self, path: &Path) -> io::Result<()> {
        self.delegate.sync_directory(path)
    }

    fn remove_file_or_empty_directory(&self, path: &Path) -> io::Result<()> {
        self.delegate.remove_file_or_empty_directory(path)
    }

    fn cleanup_owned_tree(&self, path: &Path) -> io::Result<()> {
        self.delegate.cleanup_owned_tree(path)
    }
}

struct FailFirstRenamePlatform {
    delegate: MacOsPlatform,
    failed: AtomicBool,
}

struct FailCommittedWalSyncPlatform {
    delegate: MacOsPlatform,
    transaction_dir: PathBuf,
    wal_path: PathBuf,
    failed: AtomicBool,
}

impl Platform for FailCommittedWalSyncPlatform {
    fn no_follow_kind(&self, path: &Path) -> io::Result<Option<EntryKind>> {
        self.delegate.no_follow_kind(path)
    }

    fn rename_exclusive(&self, source: &Path, destination: &Path) -> io::Result<()> {
        self.delegate.rename_exclusive(source, destination)
    }

    fn sync_file(&self, path: &Path) -> io::Result<()> {
        self.delegate.sync_file(path)
    }

    fn sync_directory(&self, path: &Path) -> io::Result<()> {
        if path == self.transaction_dir
            && !self.failed.load(Ordering::SeqCst)
            && let Ok(bytes) = fs::read(&self.wal_path)
            && let Ok(wal) = serde_json::from_slice::<WalDocument>(&bytes)
            && wal.phase == TransactionPhase::Committed
        {
            self.failed.store(true, Ordering::SeqCst);
            return Err(io::Error::other("injected committed WAL sync failure"));
        }
        self.delegate.sync_directory(path)
    }

    fn remove_file_or_empty_directory(&self, path: &Path) -> io::Result<()> {
        self.delegate.remove_file_or_empty_directory(path)
    }

    fn cleanup_owned_tree(&self, path: &Path) -> io::Result<()> {
        self.delegate.cleanup_owned_tree(path)
    }
}

struct FailWalSyncAndReloadPlatform {
    delegate: MacOsPlatform,
    transaction_dir: PathBuf,
    wal_path: PathBuf,
    failed_sync: AtomicBool,
    authority_unreadable: AtomicBool,
}

impl Platform for FailWalSyncAndReloadPlatform {
    fn no_follow_kind(&self, path: &Path) -> io::Result<Option<EntryKind>> {
        if path == self.wal_path && self.authority_unreadable.load(Ordering::SeqCst) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "injected canonical WAL reload failure",
            ));
        }
        self.delegate.no_follow_kind(path)
    }

    fn rename_exclusive(&self, source: &Path, destination: &Path) -> io::Result<()> {
        self.delegate.rename_exclusive(source, destination)
    }

    fn sync_file(&self, path: &Path) -> io::Result<()> {
        self.delegate.sync_file(path)
    }

    fn sync_directory(&self, path: &Path) -> io::Result<()> {
        if path == self.transaction_dir
            && !self.failed_sync.load(Ordering::SeqCst)
            && let Ok(bytes) = fs::read(&self.wal_path)
            && let Ok(wal) = serde_json::from_slice::<WalDocument>(&bytes)
            && wal.pending_move.is_some()
        {
            self.failed_sync.store(true, Ordering::SeqCst);
            self.authority_unreadable.store(true, Ordering::SeqCst);
            return Err(io::Error::other("injected WAL synchronization failure"));
        }
        self.delegate.sync_directory(path)
    }

    fn remove_file_or_empty_directory(&self, path: &Path) -> io::Result<()> {
        self.delegate.remove_file_or_empty_directory(path)
    }

    fn cleanup_owned_tree(&self, path: &Path) -> io::Result<()> {
        self.delegate.cleanup_owned_tree(path)
    }
}

impl Platform for FailFirstRenamePlatform {
    fn no_follow_kind(&self, path: &Path) -> io::Result<Option<EntryKind>> {
        self.delegate.no_follow_kind(path)
    }

    fn rename_exclusive(&self, source: &Path, destination: &Path) -> io::Result<()> {
        if !self.failed.swap(true, Ordering::SeqCst) {
            Err(io::Error::other("injected rename failure"))
        } else {
            self.delegate.rename_exclusive(source, destination)
        }
    }

    fn sync_file(&self, path: &Path) -> io::Result<()> {
        self.delegate.sync_file(path)
    }

    fn sync_directory(&self, path: &Path) -> io::Result<()> {
        self.delegate.sync_directory(path)
    }

    fn remove_file_or_empty_directory(&self, path: &Path) -> io::Result<()> {
        self.delegate.remove_file_or_empty_directory(path)
    }

    fn cleanup_owned_tree(&self, path: &Path) -> io::Result<()> {
        self.delegate.cleanup_owned_tree(path)
    }
}
