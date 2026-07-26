use std::fs;
use std::path::{Path, PathBuf};

use crate::InstallerError;
use crate::path::{Locator, RootId};
use crate::platform::macos::MacOsPlatform;
use crate::test_support::project_tempdir;

use super::{BackupRequest, BackupRoots, BackupStore, EnsureBackup};

#[test]
fn publish_writes_an_immutable_versioned_payload_without_selecting_it() {
    // Arrange
    let temporary = project_tempdir("backup-immutable-publication");
    let roots = create_roots(temporary.path());
    let config = roots.codex_home.join("config.toml");
    fs::write(&config, b"generation-a").expect("write current config");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &roots.state_dir);
    let request = config_request("backup-a", &roots);

    // Act
    let backup = store
        .publish_current(request)
        .expect("publish current state");

    // Assert
    assert_eq!(backup.journal.version, 1);
    assert_eq!(backup.journal.backup_id, "backup-a");
    assert_eq!(
        fs::read(backup.directory.join("payload/codex-home/config.toml"))
            .expect("read backup payload"),
        b"generation-a"
    );
    assert!(backup.directory.join("journal-v1.json").is_file());
    assert_eq!(store.load_latest(), Ok(None));

    let journal_before =
        fs::read(backup.directory.join("journal-v1.json")).expect("read backup journal");
    fs::write(&config, b"generation-b").expect("replace current config");
    let result = store.publish_current(config_request("backup-a", &roots));
    assert_eq!(
        result,
        Err(InstallerError::InvalidBackup {
            message: "backup backup-a already exists with different content".to_owned(),
        })
    );
    assert_eq!(
        fs::read(backup.directory.join("journal-v1.json")).expect("reread backup journal"),
        journal_before
    );
    assert_eq!(
        fs::read(backup.directory.join("payload/codex-home/config.toml"))
            .expect("reread backup payload"),
        b"generation-a"
    );

    fs::write(
        backup.directory.join("payload/codex-home/config.toml"),
        b"corrupt",
    )
    .expect("corrupt backup payload");
    assert_eq!(
        store.select_latest("backup-a"),
        Err(InstallerError::InvalidBackup {
            message: "backup payload fingerprint does not match journal".to_owned(),
        })
    );
    assert!(!roots.state_dir.join("backups/latest").exists());
}

#[test]
fn select_latest_atomically_publishes_the_only_restore_authority() {
    // Arrange
    let temporary = project_tempdir("backup-latest-marker");
    let roots = create_roots(temporary.path());
    fs::write(roots.codex_home.join("config.toml"), b"generation-a").expect("write current config");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &roots.state_dir);
    let backup = store
        .publish_current(config_request("backup-a", &roots))
        .expect("publish backup");
    assert_eq!(store.load_latest(), Ok(None));

    // Act
    let result = store.select_latest("backup-a");

    // Assert
    assert_eq!(result, Ok(()));
    assert_eq!(
        fs::read(roots.state_dir.join("backups/latest")).expect("read latest marker"),
        b"backup-a\n"
    );
    assert!(!roots.state_dir.join("backups/latest.tmp").exists());
    assert_eq!(store.load_latest(), Ok(Some(backup)));
}

#[test]
fn ensure_current_reuses_an_exact_latest_backup() {
    // Arrange
    let temporary = project_tempdir("backup-exact-reuse");
    let roots = create_roots(temporary.path());
    fs::write(roots.codex_home.join("config.toml"), b"generation-a").expect("write current config");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &roots.state_dir);
    let backup = store
        .publish_current(config_request("backup-a", &roots))
        .expect("publish backup");
    store.select_latest("backup-a").expect("select backup");

    // Act
    let result = store.ensure_current(config_request("backup-b", &roots));

    // Assert
    assert_eq!(result, Ok(EnsureBackup::Reused(backup)));
    assert!(!roots.state_dir.join("backups/backup-b").exists());
    assert_eq!(
        fs::read(roots.state_dir.join("backups/latest")).expect("read latest marker"),
        b"backup-a\n"
    );
}

#[test]
fn prune_keeps_the_selected_backup_until_a_durable_replacement_is_selected() {
    // Arrange
    let temporary = project_tempdir("backup-selection-before-prune");
    let roots = create_roots(temporary.path());
    let config = roots.codex_home.join("config.toml");
    fs::write(&config, b"generation-a").expect("write generation A");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &roots.state_dir);
    store
        .publish_current(config_request("backup-a", &roots))
        .expect("publish backup A");
    store.select_latest("backup-a").expect("select backup A");
    fs::write(&config, b"generation-b").expect("write generation B");
    store
        .publish_current(config_request("backup-b", &roots))
        .expect("publish durable backup B");
    assert!(roots.state_dir.join("backups/backup-a").is_dir());
    assert_eq!(
        fs::read(roots.state_dir.join("backups/latest")).expect("read marker before selection"),
        b"backup-a\n"
    );

    // Act
    store.select_latest("backup-b").expect("select backup B");
    let result = store.prune_unselected();

    // Assert
    assert_eq!(result, Ok(vec![roots.state_dir.join("backups/backup-a")]));
    assert!(!roots.state_dir.join("backups/backup-a").exists());
    assert!(roots.state_dir.join("backups/backup-b").is_dir());
    assert_eq!(
        fs::read(roots.state_dir.join("backups/latest")).expect("read marker after prune"),
        b"backup-b\n"
    );
}

#[test]
fn unknown_backup_version_is_rejected_before_store_mutation() {
    // Arrange
    let temporary = project_tempdir("backup-unknown-version");
    let roots = create_roots(temporary.path());
    fs::write(roots.codex_home.join("config.toml"), b"generation-a").expect("write current config");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &roots.state_dir);
    let backup = store
        .publish_current(config_request("backup-a", &roots))
        .expect("publish backup");
    store.select_latest("backup-a").expect("select backup");
    let journal_path = backup.directory.join("journal-v1.json");
    let mut journal: serde_json::Value =
        serde_json::from_slice(&fs::read(&journal_path).expect("read journal"))
            .expect("parse journal");
    journal["version"] = serde_json::json!(99);
    fs::write(
        &journal_path,
        serde_json::to_vec_pretty(&journal).expect("encode unknown journal"),
    )
    .expect("write unknown journal");
    let marker_before =
        fs::read(roots.state_dir.join("backups/latest")).expect("read marker before load");
    let payload_before = fs::read(backup.directory.join("payload/codex-home/config.toml"))
        .expect("read payload before load");

    // Act
    let result = store.select_latest("backup-a");

    // Assert
    assert_eq!(
        result,
        Err(InstallerError::InvalidBackup {
            message: "unsupported backup journal version: 99".to_owned(),
        })
    );
    assert_eq!(
        fs::read(roots.state_dir.join("backups/latest")).expect("reread marker"),
        marker_before
    );
    assert_eq!(
        fs::read(backup.directory.join("payload/codex-home/config.toml")).expect("reread payload"),
        payload_before
    );
}

#[test]
fn corrupt_latest_marker_is_rejected_before_store_mutation() {
    // Arrange
    let temporary = project_tempdir("backup-corrupt-marker");
    let roots = create_roots(temporary.path());
    fs::write(roots.codex_home.join("config.toml"), b"generation-a").expect("write current config");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &roots.state_dir);
    store
        .publish_current(config_request("backup-a", &roots))
        .expect("publish backup");
    let marker = roots.state_dir.join("backups/latest");
    fs::write(&marker, b"../backup-a\n").expect("write corrupt marker");
    let journal_before = fs::read(roots.state_dir.join("backups/backup-a/journal-v1.json"))
        .expect("read journal before load");

    // Act
    let result = store.select_latest("backup-a");

    // Assert
    assert_eq!(
        result,
        Err(InstallerError::InvalidBackup {
            message: "unsafe backup ID in latest marker".to_owned(),
        })
    );
    assert_eq!(fs::read(&marker).expect("reread marker"), b"../backup-a\n");
    assert_eq!(
        fs::read(roots.state_dir.join("backups/backup-a/journal-v1.json")).expect("reread journal"),
        journal_before
    );
}

#[test]
fn latest_backup_rejects_a_lexically_non_normal_codex_root() {
    // Arrange
    let temporary = project_tempdir("backup-non-normal-root");
    let roots = create_roots(temporary.path());
    let config = roots.codex_home.join("config.toml");
    fs::write(&config, b"generation-a").expect("write current config");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &roots.state_dir);
    let backup = store
        .publish_current(config_request("backup-a", &roots))
        .expect("publish backup");
    store.select_latest("backup-a").expect("select backup");
    let journal_path = backup.directory.join("journal-v1.json");
    let mut journal: serde_json::Value =
        serde_json::from_slice(&fs::read(&journal_path).expect("read backup journal"))
            .expect("parse backup journal");
    journal["roots"]["codex_home"] =
        serde_json::json!(format!("{}/./codex-home", temporary.path().display()));
    fs::write(
        &journal_path,
        serde_json::to_vec_pretty(&journal).expect("encode backup journal"),
    )
    .expect("write backup journal with unsafe root");

    // Act
    let result = store.load_latest();

    // Assert
    assert_eq!(
        result,
        Err(InstallerError::InvalidBackup {
            message: "backup codex home root is not normalized and safe".to_owned(),
        })
    );
}

#[test]
fn discard_unselected_removes_only_the_named_valid_backup_and_preserves_latest() {
    // Arrange
    let temporary = project_tempdir("backup-discard-unselected");
    let roots = create_roots(temporary.path());
    let config = roots.codex_home.join("config.toml");
    fs::write(&config, b"generation-a").expect("write generation A");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &roots.state_dir);
    store
        .publish_current(config_request("backup-a", &roots))
        .expect("publish backup A");
    store.select_latest("backup-a").expect("select backup A");
    fs::write(&config, b"generation-b").expect("write generation B");
    store
        .publish_current(config_request("backup-b", &roots))
        .expect("publish unselected backup B");

    // Act
    let result = store.discard_unselected("backup-b");

    // Assert
    assert_eq!(result, Ok(()));
    assert!(!roots.state_dir.join("backups/backup-b").exists());
    assert!(roots.state_dir.join("backups/backup-a").is_dir());
    assert_eq!(
        fs::read(roots.state_dir.join("backups/latest")).expect("read latest marker"),
        b"backup-a\n"
    );
}

#[test]
fn committed_transaction_with_a_matching_candidate_selects_it_before_pruning() {
    // Arrange
    let temporary = project_tempdir("backup-finalize-install");
    let roots = create_roots(temporary.path());
    let config = roots.codex_home.join("config.toml");
    fs::write(&config, b"generation-a").expect("write generation A");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &roots.state_dir);
    store
        .publish_current(config_request("backup-a", &roots))
        .expect("publish backup A");
    store.select_latest("backup-a").expect("select backup A");
    fs::write(&config, b"generation-b").expect("write generation B");
    store
        .publish_current(config_request("install-b", &roots))
        .expect("publish install candidate B");

    // Act
    let result = store.finalize_committed_transaction("install-b");

    // Assert
    assert_eq!(result, Ok(()));
    assert!(!roots.state_dir.join("backups/backup-a").exists());
    assert!(roots.state_dir.join("backups/install-b").is_dir());
    assert_eq!(
        fs::read(roots.state_dir.join("backups/latest")).expect("read selected candidate"),
        b"install-b\n"
    );
}

#[test]
fn committed_transaction_without_a_candidate_retains_the_existing_latest() {
    // Arrange
    let temporary = project_tempdir("backup-finalize-restore");
    let roots = create_roots(temporary.path());
    fs::write(roots.codex_home.join("config.toml"), b"generation-a").expect("write generation A");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &roots.state_dir);
    store
        .publish_current(config_request("backup-a", &roots))
        .expect("publish backup A");
    store.select_latest("backup-a").expect("select backup A");

    // Act
    let result = store.finalize_committed_transaction("restore-a");

    // Assert
    assert_eq!(result, Ok(()));
    assert!(roots.state_dir.join("backups/backup-a").is_dir());
    assert!(!roots.state_dir.join("backups/restore-a").exists());
    assert_eq!(
        fs::read(roots.state_dir.join("backups/latest")).expect("read retained latest"),
        b"backup-a\n"
    );
}

#[test]
fn committed_transaction_without_a_candidate_or_latest_fails_closed() {
    // Arrange
    let temporary = project_tempdir("backup-finalize-missing-authority");
    let roots = create_roots(temporary.path());
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &roots.state_dir);

    // Act
    let result = store.finalize_committed_transaction("restore-a");

    // Assert
    assert_eq!(
        result,
        Err(InstallerError::InvalidBackup {
            message:
                "cannot finalize committed transaction without a matching backup or latest marker"
                    .to_owned(),
        })
    );
    assert!(!roots.state_dir.join("backups").exists());
}

fn create_roots(parent: &Path) -> BackupRoots {
    let codex_home = parent.join("codex-home");
    let skills_home = parent.join("skills-home");
    let state_dir = parent.join("state");
    for directory in [&codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create backup root");
    }
    BackupRoots {
        codex_home,
        skills_home,
        state_dir,
    }
}

fn config_request(backup_id: &str, roots: &BackupRoots) -> BackupRequest {
    BackupRequest {
        backup_id: backup_id.to_owned(),
        roots: roots.clone(),
        ownership: None,
        locators: vec![
            Locator::new(RootId::CodexHome, PathBuf::from("config.toml")).expect("config locator"),
        ],
    }
}
