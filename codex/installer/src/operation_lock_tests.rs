use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::sync::{Arc, Barrier};
use std::thread;

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
    let independently_opened = OpenOptions::new()
        .read(true)
        .write(true)
        .open(codex_home.join(LOCK_NAME))
        .expect("independently open lock file");
    let lock_error = independently_opened
        .try_lock()
        .expect_err("independent try-lock must observe the held lock");
    assert!(matches!(lock_error, std::fs::TryLockError::WouldBlock));
    drop(independently_opened);
    let barrier = Arc::new(Barrier::new(2));
    let worker_barrier = Arc::clone(&barrier);
    let worker_home = codex_home.clone();
    let worker = thread::spawn(move || {
        worker_barrier.wait();
        OperationLock::acquire(&worker_home).map(drop)
    });
    barrier.wait();

    // Act
    drop(first);
    let after_first_drops = worker.join().expect("join lock worker");

    // Assert
    assert_eq!(after_first_drops, Ok(()));
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
