//! Rust implementation of the dotfiles Codex installer.

mod application;
#[cfg(target_os = "macos")]
mod backup;
mod command;
mod config_merge;
mod content;
mod error;
#[cfg(target_os = "macos")]
mod operation_lock;
mod ownership;
mod path;
mod plan;
#[cfg(target_os = "macos")]
mod platform;
mod resources;
mod source;
#[cfg(test)]
mod test_support;
#[cfg(target_os = "macos")]
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
    let invocation = command::parse_command_from_with_environment(arguments, environment)?;
    application::execute(invocation)
}
