use std::fs;
use std::path::PathBuf;

use crate::backup::{BackupRequest, BackupRoots, BackupStore};
use crate::command::{InstallCommand, InstallerCommand, RestoreCommand};
use crate::content::{CapturedContent, capture_optional};
use crate::ownership::{OwnershipManifest, manifest_content};
use crate::path::{InstallRoots, Locator, RootId};
use crate::plan::{AssetCategory, InstallPlan, PlanAction, PlanOperation};
use crate::platform::macos::MacOsPlatform;
use crate::resources::MachineResources;
use crate::test_support::project_tempdir;
use crate::transaction::{FaultPoint, TransactionEngine};

use super::{
    ApplicationContext, execute_restore_with_context_and_id, execute_with_context,
    execute_with_context_and_id,
};

const MANAGED_CONFIG: &str = concat!(
    "model = \"gpt-5.6\"\n",
    "model_reasoning_effort = \"xhigh\"\n",
    "plan_mode_reasoning_effort = \"xhigh\"\n",
    "\n",
    "[agents]\n",
    "max_threads = 6\n",
    "max_depth = 1\n",
);

#[test]
fn dry_run_creates_no_destination_or_state() {
    // Arrange
    let temporary = project_tempdir("application-dry-run");
    let source_root = temporary.path().join("source");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    fs::create_dir(&source_root).expect("create source");
    fs::write(source_root.join("config.toml"), MANAGED_CONFIG).expect("write config");
    fs::create_dir_all(source_root.join("skills/review")).expect("create source skill");
    fs::write(source_root.join("skills/review/SKILL.md"), b"review").expect("write source skill");
    let command = InstallerCommand::Install(InstallCommand {
        dry_run: true,
        adopt_existing: false,
        agent_threads: "6".to_owned(),
        codex_home: codex_home.clone(),
        skills_home: skills_home.clone(),
        state_dir: state_dir.clone(),
    });
    let context = ApplicationContext {
        source_root,
        resources: MachineResources {
            logical_cpus: 1,
            memory_bytes: 0,
        },
    };

    // Act
    let result = execute_with_context(command, context);

    // Assert
    let output = result.expect("dry-run succeeds");
    assert!(output.contains("CREATE config"));
    assert!(output.contains("CREATE skill review"));
    assert!(output.contains("CREATE manifest"));
    assert_eq!(
        (
            codex_home.exists(),
            skills_home.exists(),
            state_dir.exists(),
            codex_home.join("codex-manifest-installer.lock").exists(),
        ),
        (false, false, false, false)
    );
}

#[test]
fn mutating_install_publishes_pre_state_and_commits_owned_live_state_under_one_operation() {
    // Arrange
    let temporary = project_tempdir("application-mutating-install");
    let source_root = temporary.path().join("source");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    for directory in [&source_root, &codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create application root");
    }
    fs::write(source_root.join("config.toml"), MANAGED_CONFIG).expect("write managed config");
    fs::write(source_root.join("AGENTS.global.md"), b"desired guidance")
        .expect("write managed guidance");
    fs::create_dir_all(source_root.join("skills/adopted")).expect("create adopted source skill");
    fs::write(
        source_root.join("skills/adopted/SKILL.md"),
        b"desired adopted skill",
    )
    .expect("write adopted source skill");

    let prior_config = concat!(
        "model = \"old\"\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "approval_policy = \"on-request\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 2\n",
        "max_depth = 3\n",
        "custom = true\n",
    );
    let desired_config = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "approval_policy = \"on-request\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 6\n",
        "max_depth = 1\n",
        "custom = true\n",
    );
    fs::write(codex_home.join("config.toml"), prior_config).expect("write prior config");
    fs::create_dir(skills_home.join("adopted")).expect("create adoptable skill");
    fs::write(skills_home.join("adopted/SKILL.md"), b"prior adopted skill")
        .expect("write adoptable skill");
    fs::create_dir(skills_home.join("stale")).expect("create stale owned skill");
    fs::write(skills_home.join("stale/SKILL.md"), b"prior stale skill")
        .expect("write stale owned skill");
    fs::create_dir(skills_home.join("external")).expect("create manifest-external skill");
    fs::write(skills_home.join("external/SKILL.md"), b"external")
        .expect("write manifest-external skill");
    fs::create_dir(skills_home.join(".system")).expect("create system skill");
    fs::write(skills_home.join(".system/SKILL.md"), b"system").expect("write system skill");
    let prior_ownership = OwnershipManifest::new(false, ["stale"], []);
    let prior_manifest = manifest_content(&prior_ownership).expect("serialize prior manifest");
    fs::write(
        state_dir.join("manifest-v1.json"),
        prior_manifest.file_bytes().expect("prior manifest bytes"),
    )
    .expect("write prior manifest");
    let command = InstallerCommand::Install(InstallCommand {
        dry_run: false,
        adopt_existing: true,
        agent_threads: "6".to_owned(),
        codex_home: codex_home.clone(),
        skills_home: skills_home.clone(),
        state_dir: state_dir.clone(),
    });
    let context = ApplicationContext {
        source_root,
        resources: MachineResources {
            logical_cpus: 1,
            memory_bytes: 0,
        },
    };

    // Act
    let result = execute_with_context_and_id(command, context, "install-primary");

    // Assert
    result.expect("mutating install succeeds");
    let desired_ownership = OwnershipManifest::new(true, ["adopted"], []);
    assert_eq!(
        (
            fs::read(codex_home.join("config.toml")).expect("read installed config"),
            fs::read(codex_home.join("AGENTS.md")).expect("read installed guidance"),
            fs::read(skills_home.join("adopted/SKILL.md")).expect("read adopted skill"),
            skills_home.join("stale").exists(),
            fs::read(skills_home.join("external/SKILL.md")).expect("read external skill"),
            fs::read(skills_home.join(".system/SKILL.md")).expect("read system skill"),
            fs::read(state_dir.join("manifest-v1.json")).expect("read installed manifest"),
        ),
        (
            desired_config.as_bytes().to_vec(),
            b"desired guidance".to_vec(),
            b"desired adopted skill".to_vec(),
            false,
            b"external".to_vec(),
            b"system".to_vec(),
            manifest_content(&desired_ownership)
                .expect("serialize desired manifest")
                .file_bytes()
                .expect("desired manifest bytes")
                .to_vec(),
        )
    );

    let platform = MacOsPlatform::new();
    let backup = BackupStore::new(&platform, &state_dir)
        .load_latest()
        .expect("load latest backup")
        .expect("pre-install backup is selected");
    assert_eq!(backup.journal.backup_id, "install-primary");
    assert_eq!(backup.journal.ownership, Some(prior_ownership));
    assert_eq!(
        backup
            .content(
                &Locator::new(RootId::CodexHome, PathBuf::from("config.toml"))
                    .expect("config locator")
            )
            .expect("read backed up config"),
        Some(&CapturedContent::file(prior_config.as_bytes().to_vec()))
    );
    assert_eq!(
        fs::read(state_dir.join("backups/latest")).expect("read latest marker"),
        b"install-primary\n"
    );
    assert!(!state_dir.join("transaction/wal-v1.json").exists());
    assert!(
        fs::read_dir(state_dir.join("transaction/work"))
            .expect("read transaction work directory")
            .next()
            .is_none()
    );
    assert!(codex_home.join("codex-manifest-installer.lock").is_file());
}

#[test]
fn committed_install_retries_backup_finalization_before_transaction_cleanup() {
    // Arrange
    let temporary = project_tempdir("application-committed-finalization-retry");
    let source_root = temporary.path().join("source");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    for directory in [&source_root, &codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create application root");
    }
    fs::write(source_root.join("config.toml"), MANAGED_CONFIG).expect("write managed config");
    let prior_config = concat!(
        "model = \"old\"\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 2\n",
        "max_depth = 3\n",
    );
    fs::write(codex_home.join("config.toml"), prior_config).expect("write prior config");
    fs::create_dir_all(state_dir.join("backups/latest.tmp"))
        .expect("create invalid latest temporary directory");
    let command = || {
        InstallerCommand::Install(InstallCommand {
            dry_run: false,
            adopt_existing: false,
            agent_threads: "6".to_owned(),
            codex_home: codex_home.clone(),
            skills_home: skills_home.clone(),
            state_dir: state_dir.clone(),
        })
    };
    let context = || ApplicationContext {
        source_root: source_root.clone(),
        resources: MachineResources {
            logical_cpus: 1,
            memory_bytes: 0,
        },
    };
    let transaction_id = "committed-finalization-retry";
    let wal_path = state_dir.join("transaction/wal-v1.json");
    let work = state_dir.join("transaction/work").join(transaction_id);

    // Act
    let interrupted = execute_with_context_and_id(command(), context(), transaction_id);

    // Assert
    assert_eq!(
        interrupted,
        Err(crate::InstallerError::CommittedCleanupIncomplete {
            transaction_id: transaction_id.to_owned(),
            wal: wal_path.clone(),
            paths: vec![
                codex_home.join("config.toml"),
                work.join("stage/0"),
                work.join("tombstone/0"),
                state_dir.join("manifest-v1.json"),
                work.join("stage/1"),
            ],
            cause: None,
            cleanup_cause: Box::new(crate::InstallerError::InvalidBackup {
                message: "temporary latest marker is not an ordinary file".to_owned(),
            }),
        })
    );
    assert_eq!(
        fs::read(codex_home.join("config.toml")).expect("read committed config"),
        MANAGED_CONFIG.as_bytes()
    );
    let wal: serde_json::Value =
        serde_json::from_slice(&fs::read(&wal_path).expect("read committed WAL"))
            .expect("parse committed WAL");
    assert_eq!(wal["phase"], serde_json::json!("committed"));
    assert_eq!(
        wal["transaction_id"],
        serde_json::json!("committed-finalization-retry")
    );
    assert!(
        state_dir
            .join("transaction/work/committed-finalization-retry")
            .is_dir()
    );
    assert!(
        state_dir
            .join("backups/committed-finalization-retry")
            .is_dir()
    );
    assert!(!state_dir.join("backups/latest").exists());

    fs::remove_dir(state_dir.join("backups/latest.tmp"))
        .expect("remove injected finalization obstruction");
    let retried = execute_with_context_and_id(command(), context(), "next-operation");
    retried.expect("fresh application finalizes committed operation");
    let platform = MacOsPlatform::new();
    let latest = BackupStore::new(&platform, &state_dir)
        .load_latest()
        .expect("load finalized latest")
        .expect("committed backup is selected");
    assert_eq!(latest.journal.backup_id, "committed-finalization-retry");
    assert_eq!(
        fs::read(codex_home.join("config.toml")).expect("reread installed config"),
        MANAGED_CONFIG.as_bytes()
    );
    assert!(!wal_path.exists());
    assert!(
        !state_dir
            .join("transaction/work/committed-finalization-retry")
            .exists()
    );
    assert!(!state_dir.join("backups/next-operation").exists());
}

#[test]
fn startup_rolls_back_pre_commit_wal_before_planning_the_new_install() {
    // Arrange
    let temporary = project_tempdir("application-pre-commit-recovery");
    let source_root = temporary.path().join("source");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    for directory in [&source_root, &codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create application root");
    }
    fs::write(source_root.join("config.toml"), MANAGED_CONFIG).expect("write managed config");
    let prior_config = concat!(
        "model = \"prior\"\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 2\n",
        "max_depth = 3\n",
    );
    fs::write(codex_home.join("config.toml"), prior_config).expect("write prior config");
    let roots = InstallRoots {
        source_root: source_root.clone(),
        codex_home: codex_home.clone(),
        skills_home: skills_home.clone(),
        state_dir: state_dir.clone(),
    };
    let backup_roots = BackupRoots {
        codex_home: codex_home.clone(),
        skills_home: skills_home.clone(),
        state_dir: state_dir.clone(),
    };
    let platform = MacOsPlatform::new();
    BackupStore::new(&platform, &state_dir)
        .publish_current(BackupRequest {
            backup_id: "startup-pre-commit".to_owned(),
            roots: backup_roots,
            ownership: None,
            locators: vec![Locator::new(RootId::CodexHome, "config.toml").expect("config locator")],
        })
        .expect("publish interrupted pre-state backup");
    let interrupted_plan = InstallPlan {
        roots,
        max_threads: 6,
        actions: vec![PlanAction {
            operation: PlanOperation::Replace,
            category: AssetCategory::Config,
            name: None,
            locator: Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
            desired: Some(CapturedContent::file(b"interrupted live".to_vec())),
        }],
    };
    let interrupted = TransactionEngine::new(platform).execute(
        &interrupted_plan,
        "startup-pre-commit",
        FaultPoint::AfterFirstLiveMutationBeforeCommit,
    );
    assert!(matches!(
        interrupted,
        Err(crate::InstallerError::InjectedTransactionFault { .. })
    ));
    assert_eq!(
        fs::read(codex_home.join("config.toml")).expect("read interrupted live"),
        b"interrupted live"
    );
    let command = InstallerCommand::Install(InstallCommand {
        dry_run: false,
        adopt_existing: false,
        agent_threads: "6".to_owned(),
        codex_home: codex_home.clone(),
        skills_home,
        state_dir: state_dir.clone(),
    });
    let context = ApplicationContext {
        source_root,
        resources: MachineResources {
            logical_cpus: 1,
            memory_bytes: 0,
        },
    };

    // Act
    let result = execute_with_context_and_id(command, context, "after-recovery");

    // Assert
    result.expect("install succeeds after startup rollback");
    assert_eq!(
        fs::read(codex_home.join("config.toml")).expect("read installed config"),
        MANAGED_CONFIG.as_bytes()
    );
    let latest = BackupStore::new(&platform, &state_dir)
        .load_latest()
        .expect("load latest backup")
        .expect("new pre-install backup is selected");
    assert_eq!(latest.journal.backup_id, "after-recovery");
    assert_eq!(
        latest
            .content(&Locator::new(RootId::CodexHome, "config.toml").expect("config locator"))
            .expect("read backed up prior config"),
        Some(&CapturedContent::file(prior_config.as_bytes().to_vec()))
    );
    assert!(!state_dir.join("backups/startup-pre-commit").exists());
    assert!(!state_dir.join("transaction/wal-v1.json").exists());
    assert!(
        !state_dir
            .join("transaction/work/startup-pre-commit")
            .exists()
    );
}

#[test]
fn mutating_install_reuses_exact_latest_pre_state_without_publishing_an_extra_backup() {
    // Arrange
    let temporary = project_tempdir("application-exact-backup-reuse");
    let source_root = temporary.path().join("source");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    for directory in [&source_root, &codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create application root");
    }
    fs::write(source_root.join("config.toml"), MANAGED_CONFIG).expect("write managed config");
    let prior_config = concat!(
        "model = \"prior\"\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 2\n",
        "max_depth = 3\n",
    );
    fs::write(codex_home.join("config.toml"), prior_config).expect("write prior config");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &state_dir);
    store
        .publish_current(BackupRequest {
            backup_id: "existing-latest".to_owned(),
            roots: BackupRoots {
                codex_home: codex_home.clone(),
                skills_home: skills_home.clone(),
                state_dir: state_dir.clone(),
            },
            ownership: None,
            locators: vec![
                Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
                Locator::new(RootId::StateDir, "manifest-v1.json").expect("manifest locator"),
            ],
        })
        .expect("publish exact current backup");
    store
        .select_latest("existing-latest")
        .expect("select exact current backup");
    let command = InstallerCommand::Install(InstallCommand {
        dry_run: false,
        adopt_existing: false,
        agent_threads: "6".to_owned(),
        codex_home: codex_home.clone(),
        skills_home,
        state_dir: state_dir.clone(),
    });
    let context = ApplicationContext {
        source_root,
        resources: MachineResources {
            logical_cpus: 1,
            memory_bytes: 0,
        },
    };

    // Act
    let result = execute_with_context_and_id(command, context, "unused-candidate");

    // Assert
    result.expect("install reuses selected pre-state backup");
    let latest = store
        .load_latest()
        .expect("load latest backup")
        .expect("latest backup remains selected");
    assert_eq!(latest.journal.backup_id, "existing-latest");
    assert!(!state_dir.join("backups/unused-candidate").exists());
    assert_eq!(
        fs::read(codex_home.join("config.toml")).expect("read installed config"),
        MANAGED_CONFIG.as_bytes()
    );
    assert!(!state_dir.join("transaction/wal-v1.json").exists());
}

#[cfg(unix)]
#[test]
fn clean_transaction_rollback_discards_only_this_operations_unselected_backup() {
    // Arrange
    use std::os::unix::fs::PermissionsExt;

    let temporary = project_tempdir("application-clean-rollback-backup");
    let source_root = temporary.path().join("source");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    for directory in [&source_root, &codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create application root");
    }
    fs::write(source_root.join("config.toml"), MANAGED_CONFIG).expect("write managed config");
    fs::create_dir_all(source_root.join("skills/review")).expect("create source skill");
    fs::write(
        source_root.join("skills/review/SKILL.md"),
        b"desired review",
    )
    .expect("write source skill");
    let prior_config = concat!(
        "model = \"prior\"\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "\n",
        "[agents]\n",
        "max_threads = 2\n",
        "max_depth = 3\n",
    );
    fs::write(codex_home.join("config.toml"), b"older generation").expect("write older config");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &state_dir);
    store
        .publish_current(BackupRequest {
            backup_id: "prior-latest".to_owned(),
            roots: BackupRoots {
                codex_home: codex_home.clone(),
                skills_home: skills_home.clone(),
                state_dir: state_dir.clone(),
            },
            ownership: None,
            locators: vec![
                Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
                Locator::new(RootId::SkillsHome, "review").expect("skill locator"),
                Locator::new(RootId::StateDir, "manifest-v1.json").expect("manifest locator"),
            ],
        })
        .expect("publish prior latest");
    store
        .select_latest("prior-latest")
        .expect("select prior latest");
    fs::write(codex_home.join("config.toml"), prior_config).expect("write current prior config");
    fs::set_permissions(&skills_home, fs::Permissions::from_mode(0o555))
        .expect("make skills home read-only");
    let command = InstallerCommand::Install(InstallCommand {
        dry_run: false,
        adopt_existing: false,
        agent_threads: "6".to_owned(),
        codex_home: codex_home.clone(),
        skills_home: skills_home.clone(),
        state_dir: state_dir.clone(),
    });
    let context = ApplicationContext {
        source_root,
        resources: MachineResources {
            logical_cpus: 1,
            memory_bytes: 0,
        },
    };

    // Act
    let result = execute_with_context_and_id(command, context, "clean-rollback");
    fs::set_permissions(&skills_home, fs::Permissions::from_mode(0o755))
        .expect("restore skills home permissions");

    // Assert
    let Err(crate::InstallerError::TransactionRolledBack {
        transaction_id,
        cause,
    }) = result
    else {
        panic!("install failure must report a clean transaction rollback");
    };
    assert_eq!(transaction_id, "clean-rollback");
    assert!(matches!(*cause, crate::InstallerError::Filesystem { .. }));
    assert_eq!(
        fs::read(codex_home.join("config.toml")).expect("read rolled back config"),
        prior_config.as_bytes()
    );
    assert!(!skills_home.join("review").exists());
    assert!(!state_dir.join("transaction/wal-v1.json").exists());
    assert!(!state_dir.join("transaction/work/clean-rollback").exists());
    assert!(!state_dir.join("backups/clean-rollback").exists());
    assert!(state_dir.join("backups/prior-latest").is_dir());
    assert_eq!(
        fs::read(state_dir.join("backups/latest")).expect("read preserved latest marker"),
        b"prior-latest\n"
    );
}

#[test]
fn no_op_install_creates_neither_backup_nor_transaction_state() {
    // Arrange
    let temporary = project_tempdir("application-no-op-install");
    let source_root = temporary.path().join("source");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    for directory in [&source_root, &codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create application root");
    }
    fs::write(source_root.join("config.toml"), MANAGED_CONFIG).expect("write managed config");
    fs::write(codex_home.join("config.toml"), MANAGED_CONFIG).expect("write installed config");
    let manifest = manifest_content(&OwnershipManifest::new(false, [], []))
        .expect("serialize installed manifest");
    fs::write(
        state_dir.join("manifest-v1.json"),
        manifest.file_bytes().expect("manifest bytes"),
    )
    .expect("write installed manifest");
    let command = InstallerCommand::Install(InstallCommand {
        dry_run: false,
        adopt_existing: false,
        agent_threads: "6".to_owned(),
        codex_home: codex_home.clone(),
        skills_home,
        state_dir: state_dir.clone(),
    });
    let context = ApplicationContext {
        source_root,
        resources: MachineResources {
            logical_cpus: 1,
            memory_bytes: 0,
        },
    };

    // Act
    let result = execute_with_context_and_id(command, context, "no-op-candidate");

    // Assert
    result.expect("no-op install succeeds");
    assert!(!state_dir.join("backups").exists());
    assert!(!state_dir.join("transaction").exists());
    assert!(codex_home.join("codex-manifest-installer.lock").is_file());
    assert_eq!(
        fs::read(codex_home.join("config.toml")).expect("read unchanged config"),
        MANAGED_CONFIG.as_bytes()
    );
}

#[test]
fn restore_keeps_backup_a_selected_and_does_not_promote_b() {
    // Arrange
    let temporary = project_tempdir("application-restore-primary");
    let source_root = temporary.path().join("source");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    for directory in [&source_root, &codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create application root");
    }

    let ownership_a = OwnershipManifest::new(true, ["a-skill"], []);
    let manifest_a = manifest_content(&ownership_a).expect("serialize manifest A");
    fs::write(codex_home.join("config.toml"), b"config A").expect("write config A");
    fs::write(codex_home.join("AGENTS.md"), b"guidance A").expect("write guidance A");
    fs::create_dir(skills_home.join("a-skill")).expect("create skill A");
    fs::write(skills_home.join("a-skill/SKILL.md"), b"skill A").expect("write skill A");
    fs::write(
        state_dir.join("manifest-v1.json"),
        manifest_a.file_bytes().expect("manifest A bytes"),
    )
    .expect("write manifest A");

    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &state_dir);
    let backup_a = store
        .publish_current(BackupRequest {
            backup_id: "backup-a".to_owned(),
            roots: BackupRoots {
                codex_home: codex_home.clone(),
                skills_home: skills_home.clone(),
                state_dir: state_dir.clone(),
            },
            ownership: Some(ownership_a.clone()),
            locators: vec![
                Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
                Locator::new(RootId::CodexHome, "AGENTS.md").expect("guidance locator"),
                Locator::new(RootId::SkillsHome, "a-skill").expect("skill A locator"),
                Locator::new(RootId::StateDir, "manifest-v1.json").expect("manifest locator"),
            ],
        })
        .expect("publish backup A");
    store.select_latest("backup-a").expect("select backup A");
    let journal_a_before =
        fs::read(backup_a.directory.join("journal-v1.json")).expect("read backup A journal");
    let payload_a_before = capture_optional(&backup_a.directory.join("payload"))
        .expect("capture backup A payload")
        .expect("backup A payload exists");

    let ownership_b = OwnershipManifest::new(true, ["a-skill", "b-only"], []);
    let manifest_b = manifest_content(&ownership_b).expect("serialize manifest B");
    fs::write(codex_home.join("config.toml"), b"config B").expect("write config B");
    fs::write(codex_home.join("AGENTS.md"), b"guidance B").expect("write guidance B");
    fs::write(skills_home.join("a-skill/SKILL.md"), b"skill B").expect("write changed skill");
    fs::create_dir(skills_home.join("b-only")).expect("create B-only skill");
    fs::write(skills_home.join("b-only/SKILL.md"), b"B only").expect("write B-only skill");
    fs::create_dir(skills_home.join("external")).expect("create external skill");
    fs::write(skills_home.join("external/SKILL.md"), b"external").expect("write external skill");
    fs::create_dir(skills_home.join(".system")).expect("create system skill");
    fs::write(skills_home.join(".system/SKILL.md"), b"system").expect("write system skill");
    fs::write(
        state_dir.join("manifest-v1.json"),
        manifest_b.file_bytes().expect("manifest B bytes"),
    )
    .expect("write manifest B");

    // Act
    let result = execute_restore_with_context_and_id(
        RestoreCommand {
            state_dir: state_dir.clone(),
        },
        ApplicationContext {
            source_root,
            resources: MachineResources {
                logical_cpus: 1,
                memory_bytes: 0,
            },
        },
        "restore-primary",
    );

    // Assert
    assert_eq!(result, Ok("restore complete\n".to_owned()));
    assert_eq!(
        (
            fs::read(codex_home.join("config.toml")).expect("read restored config"),
            fs::read(codex_home.join("AGENTS.md")).expect("read restored guidance"),
            fs::read(skills_home.join("a-skill/SKILL.md")).expect("read restored skill"),
            skills_home.join("b-only").exists(),
            fs::read(skills_home.join("external/SKILL.md")).expect("read external skill"),
            fs::read(skills_home.join(".system/SKILL.md")).expect("read system skill"),
            fs::read(state_dir.join("manifest-v1.json")).expect("read restored manifest"),
        ),
        (
            b"config A".to_vec(),
            b"guidance A".to_vec(),
            b"skill A".to_vec(),
            false,
            b"external".to_vec(),
            b"system".to_vec(),
            manifest_a.file_bytes().expect("manifest A bytes").to_vec(),
        )
    );
    let latest = store
        .load_latest()
        .expect("load latest after restore")
        .expect("backup A remains selected");
    assert_eq!(latest.journal.backup_id, "backup-a");
    assert_eq!(
        fs::read(state_dir.join("backups/latest")).expect("read latest marker"),
        b"backup-a\n"
    );
    assert_eq!(
        fs::read(backup_a.directory.join("journal-v1.json")).expect("reread backup A journal"),
        journal_a_before
    );
    assert_eq!(
        capture_optional(&backup_a.directory.join("payload"))
            .expect("recapture backup A payload")
            .expect("backup A payload remains"),
        payload_a_before
    );
    assert!(!state_dir.join("backups/restore-primary").exists());
    assert!(!state_dir.join("transaction/wal-v1.json").exists());
    assert!(
        fs::read_dir(state_dir.join("transaction/work"))
            .expect("read transaction work directory")
            .next()
            .is_none()
    );
    assert!(codex_home.join("codex-manifest-installer.lock").is_file());
}

#[test]
fn install_restart_finalizes_a_committed_restore_without_promoting_live_state() {
    // Arrange
    let temporary = project_tempdir("application-restore-committed-restart");
    let source_root = temporary.path().join("source");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    for directory in [&source_root, &codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create application root");
    }
    fs::write(source_root.join("config.toml"), MANAGED_CONFIG).expect("write install source");

    let ownership_a = OwnershipManifest::new(false, [], []);
    let manifest_a = manifest_content(&ownership_a).expect("serialize manifest A");
    fs::write(codex_home.join("config.toml"), MANAGED_CONFIG).expect("write config A");
    fs::write(
        state_dir.join("manifest-v1.json"),
        manifest_a.file_bytes().expect("manifest A bytes"),
    )
    .expect("write manifest A");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &state_dir);
    store
        .publish_current(BackupRequest {
            backup_id: "backup-a".to_owned(),
            roots: BackupRoots {
                codex_home: codex_home.clone(),
                skills_home: skills_home.clone(),
                state_dir: state_dir.clone(),
            },
            ownership: Some(ownership_a),
            locators: vec![
                Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
                Locator::new(RootId::StateDir, "manifest-v1.json").expect("manifest locator"),
            ],
        })
        .expect("publish backup A");
    store.select_latest("backup-a").expect("select backup A");
    fs::write(codex_home.join("config.toml"), b"problematic B").expect("write config B");
    let finalization_blocker = state_dir.join("backups/blocker");
    fs::write(&finalization_blocker, b"not a backup directory")
        .expect("create one-shot finalization blocker");
    let restore_context = || ApplicationContext {
        source_root: source_root.clone(),
        resources: MachineResources {
            logical_cpus: 1,
            memory_bytes: 0,
        },
    };
    let transaction_id = "restore-restart";
    let wal_path = state_dir.join("transaction/wal-v1.json");
    let work = state_dir.join("transaction/work").join(transaction_id);

    // Act
    let interrupted = execute_restore_with_context_and_id(
        RestoreCommand {
            state_dir: state_dir.clone(),
        },
        restore_context(),
        transaction_id,
    );

    // Assert
    assert_eq!(
        interrupted,
        Err(crate::InstallerError::CommittedCleanupIncomplete {
            transaction_id: transaction_id.to_owned(),
            wal: wal_path.clone(),
            paths: vec![
                codex_home.join("config.toml"),
                work.join("stage/0"),
                work.join("tombstone/0"),
            ],
            cause: None,
            cleanup_cause: Box::new(crate::InstallerError::InvalidBackup {
                message: format!(
                    "selected backup is not an ordinary directory: {}",
                    finalization_blocker.display()
                ),
            }),
        })
    );
    assert_eq!(
        fs::read(codex_home.join("config.toml")).expect("read committed restored config"),
        MANAGED_CONFIG.as_bytes()
    );
    assert_eq!(
        fs::read(state_dir.join("backups/latest")).expect("read unchanged latest marker"),
        b"backup-a\n"
    );
    let wal: serde_json::Value =
        serde_json::from_slice(&fs::read(&wal_path).expect("read committed restore WAL"))
            .expect("parse committed restore WAL");
    assert_eq!(wal["phase"], serde_json::json!("committed"));
    assert_eq!(wal["transaction_id"], serde_json::json!("restore-restart"));
    assert!(state_dir.join("transaction/work/restore-restart").is_dir());
    assert!(!state_dir.join("backups/restore-restart").exists());

    fs::remove_file(finalization_blocker).expect("remove one-shot finalization blocker");
    let install = InstallerCommand::Install(InstallCommand {
        dry_run: false,
        adopt_existing: false,
        agent_threads: "6".to_owned(),
        codex_home: codex_home.clone(),
        skills_home,
        state_dir: state_dir.clone(),
    });
    let restarted =
        execute_with_context_and_id(install, restore_context(), "after-restore-restart");
    assert_eq!(restarted, Ok("install complete\n".to_owned()));
    assert_eq!(
        fs::read(codex_home.join("config.toml")).expect("reread restored config"),
        MANAGED_CONFIG.as_bytes()
    );
    assert_eq!(
        fs::read(state_dir.join("backups/latest")).expect("read retained latest marker"),
        b"backup-a\n"
    );
    assert!(state_dir.join("backups/backup-a").is_dir());
    assert!(!state_dir.join("backups/restore-restart").exists());
    assert!(!state_dir.join("backups/after-restore-restart").exists());
    assert!(!wal_path.exists());
    assert!(!state_dir.join("transaction/work/restore-restart").exists());
}

#[test]
fn no_op_restore_changes_neither_backup_selection_nor_transaction_state() {
    // Arrange
    let temporary = project_tempdir("application-no-op-restore");
    let source_root = temporary.path().join("source");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    for directory in [&source_root, &codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create application root");
    }
    let ownership = OwnershipManifest::new(false, [], []);
    let manifest = manifest_content(&ownership).expect("serialize manifest");
    fs::write(codex_home.join("config.toml"), b"selected config").expect("write selected config");
    fs::write(
        state_dir.join("manifest-v1.json"),
        manifest.file_bytes().expect("manifest bytes"),
    )
    .expect("write selected manifest");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &state_dir);
    store
        .publish_current(BackupRequest {
            backup_id: "backup-a".to_owned(),
            roots: BackupRoots {
                codex_home: codex_home.clone(),
                skills_home,
                state_dir: state_dir.clone(),
            },
            ownership: Some(ownership),
            locators: vec![
                Locator::new(RootId::CodexHome, "config.toml").expect("config locator"),
                Locator::new(RootId::StateDir, "manifest-v1.json").expect("manifest locator"),
            ],
        })
        .expect("publish backup A");
    store.select_latest("backup-a").expect("select backup A");
    let backups_before = capture_optional(&state_dir.join("backups"))
        .expect("capture backups before restore")
        .expect("backups exist");

    // Act
    let result = execute_restore_with_context_and_id(
        RestoreCommand {
            state_dir: state_dir.clone(),
        },
        ApplicationContext {
            source_root,
            resources: MachineResources {
                logical_cpus: 1,
                memory_bytes: 0,
            },
        },
        "unused-restore-id",
    );

    // Assert
    assert_eq!(result, Ok("restore complete\n".to_owned()));
    assert_eq!(
        capture_optional(&state_dir.join("backups"))
            .expect("capture backups after restore")
            .expect("backups remain"),
        backups_before
    );
    assert!(!state_dir.join("transaction").exists());
    assert!(!state_dir.join("backups/unused-restore-id").exists());
    assert!(codex_home.join("codex-manifest-installer.lock").is_file());
}
