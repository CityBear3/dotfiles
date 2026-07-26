use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::InstallerError;
use crate::path::validate_relative;

use super::CapturedContent;

pub(crate) fn capture_optional(path: &Path) -> Result<Option<CapturedContent>, InstallerError> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(filesystem_error("inspect content", path, error)),
    };
    if metadata.file_type().is_symlink() {
        return Err(unsafe_path(path, "symlink content is not allowed"));
    }
    if metadata.is_file() {
        return fs::read(path)
            .map(CapturedContent::file)
            .map(Some)
            .map_err(|error| filesystem_error("read content", path, error));
    }
    if metadata.is_dir() {
        let mut directories = BTreeSet::new();
        let mut files = BTreeMap::new();
        capture_directory(path, path, &mut directories, &mut files)?;
        return Ok(Some(CapturedContent::directory(directories, files)));
    }
    Err(unsafe_path(path, "special content is not allowed"))
}

fn capture_directory(
    root: &Path,
    directory: &Path,
    directories: &mut BTreeSet<PathBuf>,
    files: &mut BTreeMap<PathBuf, Vec<u8>>,
) -> Result<(), InstallerError> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| filesystem_error("read directory", directory, error))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| filesystem_error("read directory entry", directory, error))?;
    entries.sort_by_key(fs::DirEntry::file_name);

    for entry in entries {
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .map_err(|error| InstallerError::Filesystem {
                message: format!("derive relative content path {}: {error}", path.display()),
            })?
            .to_path_buf();
        validate_relative(&relative)?;
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| filesystem_error("inspect content", &path, error))?;
        if metadata.file_type().is_symlink() {
            return Err(unsafe_path(&path, "symlink content is not allowed"));
        }
        if metadata.is_dir() {
            directories.insert(relative);
            capture_directory(root, &path, directories, files)?;
        } else if metadata.is_file() {
            let bytes =
                fs::read(&path).map_err(|error| filesystem_error("read content", &path, error))?;
            files.insert(relative, bytes);
        } else {
            return Err(unsafe_path(&path, "special content is not allowed"));
        }
    }
    Ok(())
}

fn unsafe_path(path: &Path, message: impl Into<String>) -> InstallerError {
    InstallerError::UnsafePath {
        path: path.to_path_buf(),
        message: message.into(),
    }
}

fn filesystem_error(operation: &str, path: &Path, error: io::Error) -> InstallerError {
    InstallerError::Filesystem {
        message: format!("{operation} {}: {error}", path.display()),
    }
}
