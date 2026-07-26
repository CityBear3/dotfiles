use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use crate::InstallerError;
use crate::platform::Platform;

use super::{CapturedContent, ContentPayload};

#[allow(
    dead_code,
    reason = "Task 2 captures immutable content; Task 3 consumes materialization"
)]
pub(crate) fn materialize(
    content: &CapturedContent,
    destination: &Path,
) -> Result<(), InstallerError> {
    match &content.payload {
        ContentPayload::File(bytes) => write_new_file(destination, bytes),
        ContentPayload::Directory { directories, files } => {
            fs::create_dir(destination)
                .map_err(|error| filesystem_error("create directory", destination, error))?;
            for relative in directories {
                let path = destination.join(relative);
                fs::create_dir(&path)
                    .map_err(|error| filesystem_error("create directory", &path, error))?;
            }
            for (relative, bytes) in files {
                write_new_file(&destination.join(relative), bytes)?;
            }
            Ok(())
        }
    }
}

#[allow(
    dead_code,
    reason = "Task 4 wires durable materialization through the transaction engine"
)]
pub(crate) fn materialize_durable<P: Platform>(
    platform: &P,
    content: &CapturedContent,
    destination: &Path,
) -> Result<(), InstallerError> {
    materialize(content, destination)?;
    match &content.payload {
        ContentPayload::File(_) => platform
            .sync_file(destination)
            .map_err(|error| filesystem_error("synchronize file", destination, error))?,
        ContentPayload::Directory { directories, files } => {
            for relative in files.keys() {
                let path = destination.join(relative);
                platform
                    .sync_file(&path)
                    .map_err(|error| filesystem_error("synchronize file", &path, error))?;
            }
            let mut directory_paths = directories
                .iter()
                .map(|relative| destination.join(relative))
                .collect::<Vec<_>>();
            directory_paths.push(destination.to_owned());
            directory_paths.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
            for path in directory_paths {
                platform
                    .sync_directory(&path)
                    .map_err(|error| filesystem_error("synchronize directory", &path, error))?;
            }
        }
    }
    if let Some(parent) = destination.parent() {
        platform
            .sync_directory(parent)
            .map_err(|error| filesystem_error("synchronize directory", parent, error))?;
    }
    Ok(())
}

fn write_new_file(path: &Path, bytes: &[u8]) -> Result<(), InstallerError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| filesystem_error("create file", path, error))?;
    file.write_all(bytes)
        .map_err(|error| filesystem_error("write file", path, error))
}

fn filesystem_error(operation: &str, path: &Path, error: io::Error) -> InstallerError {
    InstallerError::Filesystem {
        message: format!("{operation} {}: {error}", path.display()),
    }
}
