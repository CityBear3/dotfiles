use std::ffi::CString;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use super::{EntryKind, Platform};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct MacOsPlatform;

impl MacOsPlatform {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Platform for MacOsPlatform {
    fn no_follow_kind(&self, path: &Path) -> io::Result<Option<EntryKind>> {
        let metadata = match fs::symlink_metadata(path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error),
        };
        if metadata.file_type().is_symlink() {
            Ok(Some(EntryKind::Symlink))
        } else if metadata.is_file() {
            Ok(Some(EntryKind::File))
        } else if metadata.is_dir() {
            Ok(Some(EntryKind::Directory))
        } else {
            Ok(Some(EntryKind::Other))
        }
    }

    fn rename_exclusive(&self, source: &Path, destination: &Path) -> io::Result<()> {
        let source = path_cstring(source)?;
        let destination = path_cstring(destination)?;

        // SAFETY: Both paths are NUL-terminated C strings and remain alive for the call.
        let result =
            unsafe { libc::renamex_np(source.as_ptr(), destination.as_ptr(), libc::RENAME_EXCL) };
        if result == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn sync_file(&self, path: &Path) -> io::Result<()> {
        let file = open_no_follow(path, 0)?;
        if !file.metadata()?.is_file() {
            return Err(unsupported_entry(path, "ordinary file"));
        }
        file.sync_all()
    }

    fn sync_directory(&self, path: &Path) -> io::Result<()> {
        let directory = open_no_follow(path, libc::O_DIRECTORY)?;
        if !directory.metadata()?.is_dir() {
            return Err(unsupported_entry(path, "ordinary directory"));
        }
        directory.sync_all()
    }

    #[cfg(test)]
    fn remove_file_or_empty_directory(&self, path: &Path) -> io::Result<()> {
        match self.no_follow_kind(path)? {
            Some(EntryKind::File) => fs::remove_file(path),
            Some(EntryKind::Directory) => fs::remove_dir(path),
            Some(EntryKind::Symlink | EntryKind::Other) => {
                Err(unsupported_entry(path, "ordinary file or empty directory"))
            }
            None => Ok(()),
        }
    }

    fn cleanup_owned_tree(&self, path: &Path) -> io::Result<()> {
        let mut files = Vec::new();
        let mut directories = Vec::new();
        preflight_owned_tree(self, path, &mut files, &mut directories)?;

        for file in files {
            fs::remove_file(&file)?;
            if let Some(parent) = file.parent() {
                self.sync_directory(parent)?;
            }
        }
        directories.sort_by_key(|directory| std::cmp::Reverse(directory.components().count()));
        for directory in directories {
            fs::remove_dir(&directory)?;
            if let Some(parent) = directory.parent() {
                self.sync_directory(parent)?;
            }
        }
        Ok(())
    }
}

fn open_no_follow(path: &Path, additional_flags: i32) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW | additional_flags)
        .open(path)
}

fn preflight_owned_tree(
    platform: &MacOsPlatform,
    path: &Path,
    files: &mut Vec<std::path::PathBuf>,
    directories: &mut Vec<std::path::PathBuf>,
) -> io::Result<()> {
    match platform.no_follow_kind(path)? {
        None => Ok(()),
        Some(EntryKind::File) => {
            files.push(path.to_owned());
            Ok(())
        }
        Some(EntryKind::Directory) => {
            directories.push(path.to_owned());
            let mut children = fs::read_dir(path)?
                .map(|entry| entry.map(|entry| entry.path()))
                .collect::<io::Result<Vec<_>>>()?;
            children.sort();
            for child in children {
                preflight_owned_tree(platform, &child, files, directories)?;
            }
            Ok(())
        }
        Some(EntryKind::Symlink | EntryKind::Other) => {
            Err(unsupported_entry(path, "ordinary file or directory"))
        }
    }
}

fn path_cstring(path: &Path) -> io::Result<CString> {
    CString::new(path.as_os_str().as_bytes())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains a NUL byte"))
}

fn unsupported_entry(path: &Path, expected: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("{} is not an {expected}", path.display()),
    )
}
