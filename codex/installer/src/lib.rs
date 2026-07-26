//! Rust implementation of the dotfiles Codex installer.

mod command;
#[allow(
    dead_code,
    reason = "the configuration merger is wired into application planning in Task 2"
)]
mod config_merge;
mod error;
#[allow(
    dead_code,
    reason = "machine resource selection is wired into application planning in Task 2"
)]
mod resources;
#[cfg(test)]
mod test_support;

pub use command::{InstallCommand, InstallerCommand, RestoreCommand};
pub use error::InstallerError;

/// Run the installer command shell.
pub fn run_from<I, T, F>(arguments: I, environment: F) -> Result<(), InstallerError>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString>,
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let command = command::parse_command_from_with_environment(arguments, environment)?;
    Err(InstallerError::NotImplemented {
        operation: command.operation_name(),
    })
}
