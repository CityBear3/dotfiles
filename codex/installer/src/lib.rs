//! Rust implementation of the dotfiles Codex installer.

mod application;
mod command;
mod config_merge;
mod content;
mod error;
#[allow(
    dead_code,
    reason = "Task 4 wires the operation lock into mutating commands"
)]
mod operation_lock;
mod ownership;
mod path;
mod plan;
#[allow(
    dead_code,
    reason = "Task 4 wires the macOS backend into the application"
)]
mod platform;
mod resources;
mod source;
#[cfg(test)]
mod test_support;
#[allow(
    dead_code,
    unused_imports,
    reason = "Task 4 wires transaction execution into mutating commands"
)]
mod transaction;

pub use command::{InstallCommand, InstallerCommand, RestoreCommand};
pub use error::InstallerError;

/// Parse and run an installer command.
pub fn run_from<I, T, F>(arguments: I, environment: F) -> Result<String, InstallerError>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString>,
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let command = command::parse_command_from_with_environment(arguments, environment)?;
    application::execute(command)
}
