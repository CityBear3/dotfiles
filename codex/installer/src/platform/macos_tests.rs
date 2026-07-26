use std::fs;
use std::os::unix::fs::symlink;

use crate::test_support::project_tempdir;

use super::macos::MacOsPlatform;
use super::{EntryKind, Platform};

#[test]
fn no_follow_kind_distinguishes_ordinary_entries_and_absence() {
    // Arrange
    let temporary = project_tempdir("platform-entry-kinds");
    let file = temporary.path().join("file.txt");
    let directory = temporary.path().join("directory");
    let absent = temporary.path().join("absent");
    fs::write(&file, b"file").expect("write file");
    fs::create_dir(&directory).expect("create directory");
    let platform = MacOsPlatform::new();

    // Act
    let result = (
        platform.no_follow_kind(&file).expect("inspect file"),
        platform
            .no_follow_kind(&directory)
            .expect("inspect directory"),
        platform.no_follow_kind(&absent).expect("inspect absence"),
    );

    // Assert
    assert_eq!(
        result,
        (Some(EntryKind::File), Some(EntryKind::Directory), None,)
    );
}

#[test]
fn no_follow_kind_reports_a_symlink_without_reading_its_target() {
    // Arrange
    let temporary = project_tempdir("platform-symlink-kind");
    let target = temporary.path().join("target.txt");
    let linked = temporary.path().join("linked.txt");
    fs::write(&target, b"target").expect("write target");
    symlink(&target, &linked).expect("create symlink");

    // Act
    let result = MacOsPlatform::new().no_follow_kind(&linked);

    // Assert
    assert_eq!(result.expect("inspect symlink"), Some(EntryKind::Symlink));
    assert_eq!(fs::read(target).expect("read target"), b"target");
}

#[test]
fn exclusive_rename_moves_source_into_an_absent_destination() {
    // Arrange
    let temporary = project_tempdir("platform-exclusive-rename");
    let source = temporary.path().join("source.txt");
    let destination = temporary.path().join("destination.txt");
    fs::write(&source, b"source").expect("write source");

    // Act
    let result = MacOsPlatform::new().rename_exclusive(&source, &destination);

    // Assert
    assert!(result.is_ok());
    assert_eq!(
        (
            source.exists(),
            fs::read(destination).expect("read destination"),
        ),
        (false, b"source".to_vec())
    );
}

#[test]
fn exclusive_rename_preserves_an_existing_destination() {
    // Arrange
    let temporary = project_tempdir("platform-exclusive-no-overwrite");
    let source = temporary.path().join("source.txt");
    let destination = temporary.path().join("destination.txt");
    fs::write(&source, b"source").expect("write source");
    fs::write(&destination, b"destination").expect("write destination");

    // Act
    let result = MacOsPlatform::new().rename_exclusive(&source, &destination);

    // Assert
    assert_eq!(
        result
            .expect_err("exclusive rename must reject overwrite")
            .kind(),
        std::io::ErrorKind::AlreadyExists
    );
    assert_eq!(
        (
            fs::read(source).expect("read preserved source"),
            fs::read(destination).expect("read preserved destination"),
        ),
        (b"source".to_vec(), b"destination".to_vec())
    );
}

#[test]
fn sync_file_accepts_an_ordinary_file() {
    // Arrange
    let temporary = project_tempdir("platform-sync-file");
    let file = temporary.path().join("file.txt");
    fs::write(&file, b"file").expect("write file");

    // Act
    let result = MacOsPlatform::new().sync_file(&file);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn sync_directory_accepts_an_ordinary_directory() {
    // Arrange
    let temporary = project_tempdir("platform-sync-directory");

    // Act
    let result = MacOsPlatform::new().sync_directory(temporary.path());

    // Assert
    assert!(result.is_ok());
}

#[test]
fn remove_file_or_empty_directory_removes_both_supported_entry_kinds() {
    // Arrange
    let temporary = project_tempdir("platform-remove-entry");
    let file = temporary.path().join("file.txt");
    let directory = temporary.path().join("directory");
    fs::write(&file, b"file").expect("write file");
    fs::create_dir(&directory).expect("create directory");
    let platform = MacOsPlatform::new();

    // Act
    let file_result = platform.remove_file_or_empty_directory(&file);
    let directory_result = platform.remove_file_or_empty_directory(&directory);

    // Assert
    assert!(file_result.is_ok());
    assert!(directory_result.is_ok());
    assert!(!file.exists());
    assert!(!directory.exists());
}

#[test]
fn cleanup_owned_tree_removes_a_complete_ordinary_tree() {
    // Arrange
    let temporary = project_tempdir("platform-cleanup-tree");
    let owned = temporary.path().join("owned");
    let nested = owned.join("nested");
    fs::create_dir_all(&nested).expect("create owned tree");
    fs::write(owned.join("root.txt"), b"root").expect("write root file");
    fs::write(nested.join("nested.txt"), b"nested").expect("write nested file");

    // Act
    let result = MacOsPlatform::new().cleanup_owned_tree(&owned);

    // Assert
    assert!(result.is_ok());
    assert!(!owned.exists());
}

#[test]
fn cleanup_owned_tree_rejects_a_symlink_before_removing_anything() {
    // Arrange
    let temporary = project_tempdir("platform-cleanup-symlink");
    let owned = temporary.path().join("owned");
    let nested = owned.join("nested");
    let ordinary = owned.join("ordinary.txt");
    let linked = nested.join("linked.txt");
    let target = temporary.path().join("outside.txt");
    fs::create_dir_all(&nested).expect("create owned tree");
    fs::write(&ordinary, b"ordinary").expect("write ordinary file");
    fs::write(&target, b"outside").expect("write outside target");
    symlink(&target, &linked).expect("create symlink");

    // Act
    let result = MacOsPlatform::new().cleanup_owned_tree(&owned);

    // Assert
    assert_eq!(
        result.expect_err("symlink must be rejected").kind(),
        std::io::ErrorKind::InvalidData
    );
    assert_eq!(
        fs::read(ordinary).expect("ordinary file preserved"),
        b"ordinary"
    );
    assert!(
        fs::symlink_metadata(linked)
            .expect("symlink preserved")
            .file_type()
            .is_symlink()
    );
    assert_eq!(
        fs::read(target).expect("outside target preserved"),
        b"outside"
    );
}
