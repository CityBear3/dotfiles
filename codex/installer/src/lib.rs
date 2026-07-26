//! Rust implementation of the dotfiles Codex installer.

mod application;
mod command;
mod config_merge;
mod content;
mod error;
mod ownership;
mod path;
mod plan;
mod resources;
mod source;
#[cfg(test)]
mod test_support;

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
