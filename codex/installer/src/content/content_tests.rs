use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use crate::test_support::project_tempdir;

use super::materialize::materialize;
use super::{CapturedContent, capture_optional};

#[test]
fn captured_file_is_immutable_after_source_changes() {
    // Arrange
    let temporary = project_tempdir("content-file-immutable");
    let source = temporary.path().join("source.txt");
    fs::write(&source, b"before").expect("write source");

    // Act
    let result = capture_optional(&source);
    fs::write(&source, b"after").expect("change source after capture");

    // Assert
    assert_eq!(result, Ok(Some(CapturedContent::file(b"before".to_vec()))));
}

#[test]
fn directory_capture_records_nested_files_and_empty_directories() {
    // Arrange
    let temporary = project_tempdir("content-directory-capture");
    let source = temporary.path().join("source");
    fs::create_dir(&source).expect("create source");
    fs::create_dir_all(source.join("nested/empty")).expect("create nested directories");
    fs::write(source.join("root.txt"), b"root").expect("write root file");
    fs::write(source.join("nested/child.txt"), b"child").expect("write nested file");
    let directories = BTreeSet::from([PathBuf::from("nested"), PathBuf::from("nested/empty")]);
    let files = BTreeMap::from([
        (PathBuf::from("nested/child.txt"), b"child".to_vec()),
        (PathBuf::from("root.txt"), b"root".to_vec()),
    ]);

    // Act
    let result = capture_optional(&source);

    // Assert
    assert_eq!(
        result,
        Ok(Some(CapturedContent::directory(directories, files)))
    );
}

#[test]
fn capture_rejects_symlink_without_following_it() {
    // Arrange
    let temporary = project_tempdir("content-symlink");
    let target = temporary.path().join("target.txt");
    let source = temporary.path().join("source.txt");
    fs::write(&target, b"secret").expect("write target");
    symlink(&target, &source).expect("create source symlink");

    // Act
    let result = capture_optional(&source);

    // Assert
    assert!(result.is_err());
}

#[test]
fn materialize_reproduces_captured_directory() {
    // Arrange
    let temporary = project_tempdir("content-materialize");
    let destination = temporary.path().join("stage");
    let content = CapturedContent::directory(
        BTreeSet::from([PathBuf::from("nested"), PathBuf::from("nested/empty")]),
        BTreeMap::from([
            (PathBuf::from("nested/child.txt"), b"child".to_vec()),
            (PathBuf::from("root.txt"), b"root".to_vec()),
        ]),
    );

    // Act
    let result = materialize(&content, &destination);

    // Assert
    assert_eq!(result, Ok(()));
    assert_eq!(
        (
            fs::read(destination.join("root.txt")).expect("read root file"),
            fs::read(destination.join("nested/child.txt")).expect("read nested file"),
            destination.join("nested/empty").is_dir(),
        ),
        (b"root".to_vec(), b"child".to_vec(), true)
    );
}
