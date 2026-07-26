use std::io;
use std::path::Path;

pub(crate) mod macos;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EntryKind {
    File,
    Directory,
    Symlink,
    Other,
}

pub(crate) trait Platform {
    fn no_follow_kind(&self, path: &Path) -> io::Result<Option<EntryKind>>;
    fn rename_exclusive(&self, source: &Path, destination: &Path) -> io::Result<()>;
    fn sync_file(&self, path: &Path) -> io::Result<()>;
    fn sync_directory(&self, path: &Path) -> io::Result<()>;
    #[cfg(test)]
    fn remove_file_or_empty_directory(&self, path: &Path) -> io::Result<()>;
    fn cleanup_owned_tree(&self, path: &Path) -> io::Result<()>;
}

#[cfg(test)]
#[path = "platform/macos_tests.rs"]
mod tests;
