use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use crate::InstallerError;

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
