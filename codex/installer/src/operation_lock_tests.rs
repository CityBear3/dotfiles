use std::ffi::OsString;
use std::fs;
use std::sync::{Arc, Barrier, mpsc};
use std::thread;
use std::time::Duration;

use crate::test_support::project_tempdir;

use super::OperationLock;

const LOCK_NAME: &str = "codex-manifest-installer.lock";

#[test]
fn acquire_creates_only_the_persistent_empty_lock_file() {
    // Arrange
    let temporary = project_tempdir("operation-lock-create");
    let codex_home = temporary.path().join("codex-home");
    fs::create_dir(&codex_home).expect("create Codex home");

    // Act
    let result = OperationLock::acquire(&codex_home);

    // Assert
    let lock = result.expect("acquire lock");
    let entries = fs::read_dir(&codex_home)
        .expect("read Codex home")
        .map(|entry| entry.expect("read lock entry").file_name())
        .collect::<Vec<_>>();
    assert_eq!(entries, vec![OsString::from(LOCK_NAME)]);
    assert_eq!(
        fs::metadata(codex_home.join(LOCK_NAME))
            .expect("lock metadata")
            .len(),
        0
    );
    drop(lock);
    assert!(codex_home.join(LOCK_NAME).is_file());
}

#[test]
fn independently_opened_lock_files_serialize_blocking_acquisition() {
    // Arrange
    let temporary = project_tempdir("operation-lock-blocking");
    let codex_home = temporary.path().join("codex-home");
    fs::create_dir(&codex_home).expect("create Codex home");
    let first = OperationLock::acquire(&codex_home).expect("acquire first lock");
    let barrier = Arc::new(Barrier::new(2));
    let worker_barrier = Arc::clone(&barrier);
    let worker_home = codex_home.clone();
    let (acquired_tx, acquired_rx) = mpsc::channel();
    let worker = thread::spawn(move || {
        worker_barrier.wait();
        acquired_tx
            .send(OperationLock::acquire(&worker_home))
            .expect("send second acquisition");
    });
    barrier.wait();

    // Act
    let while_first_lives = acquired_rx.recv_timeout(Duration::from_millis(100));
    drop(first);
    let after_first_drops = acquired_rx
        .recv_timeout(Duration::from_secs(2))
        .expect("second acquisition completes");

    // Assert
    assert!(
        matches!(while_first_lives, Err(mpsc::RecvTimeoutError::Timeout)),
        "second independently opened file must block: acquisition completed early"
    );
    let second = after_first_drops.expect("second lock succeeds");
    drop(second);
    worker.join().expect("join lock worker");
}

#[test]
fn missing_codex_home_is_not_created_for_the_lock() {
    // Arrange
    let temporary = project_tempdir("operation-lock-missing-home");
    let codex_home = temporary.path().join("missing-codex-home");

    // Act
    let result = OperationLock::acquire(&codex_home);

    // Assert
    assert!(result.is_err());
    assert!(!codex_home.exists());
}
