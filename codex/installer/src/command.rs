use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::InstallerError;

/// A parsed installer command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InstallerCommand {
    Install(InstallCommand),
    Restore(RestoreCommand),
}

impl InstallerCommand {
    pub(crate) fn operation_name(&self) -> &'static str {
        match self {
            Self::Install(_) => "install",
            Self::Restore(_) => "restore",
        }
    }
}

/// Fully resolved arguments for an install or dry-run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstallCommand {
    pub dry_run: bool,
    pub adopt_existing: bool,
    pub agent_threads: String,
    pub codex_home: PathBuf,
    pub skills_home: PathBuf,
    pub state_dir: PathBuf,
}

/// Fully resolved arguments for restoring the latest backup.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestoreCommand {
    pub state_dir: PathBuf,
}

#[derive(Debug, Parser)]
#[command(name = "dotfiles-codex-installer", disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: RawCommand,
}

#[derive(Debug, Subcommand)]
enum RawCommand {
    Install(RawInstallCommand),
    Restore(RawRestoreCommand),
}

#[derive(Debug, Args)]
struct RawInstallCommand {
    #[arg(long)]
    dry_run: bool,

    #[arg(long)]
    adopt_existing: bool,

    #[arg(
        long,
        default_value = "auto",
        value_name = "auto|2..=32",
        value_parser = parse_agent_threads
    )]
    agent_threads: String,

    #[arg(long, value_name = "PATH")]
    codex_home: Option<PathBuf>,

    #[arg(long, value_name = "PATH")]
    skills_home: Option<PathBuf>,

    #[arg(long, value_name = "PATH")]
    state_dir: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct RawRestoreCommand {
    #[arg(long, value_name = "PATH")]
    state_dir: Option<PathBuf>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct InstallerEnvironment {
    home: Option<OsString>,
    codex_home: Option<OsString>,
    xdg_state_home: Option<OsString>,
}

pub(crate) fn parse_command_from_with_environment<I, T, F>(
    arguments: I,
    environment: F,
) -> Result<InstallerCommand, InstallerError>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString>,
    F: Fn(&str) -> Option<OsString>,
{
    let environment = InstallerEnvironment {
        home: nonempty_value(environment("HOME")),
        codex_home: nonempty_value(environment("CODEX_HOME")),
        xdg_state_home: nonempty_value(environment("XDG_STATE_HOME")),
    };
    let mut arguments = arguments.into_iter().map(Into::into).collect::<Vec<_>>();
    if arguments.is_empty() {
        arguments.push(OsString::from("dotfiles-codex-installer"));
    }
    if arguments.len() == 1 || arguments.get(1).is_some_and(|value| is_option(value)) {
        arguments.insert(1, OsString::from("install"));
    }

    let parsed = Cli::try_parse_from(arguments).map_err(InstallerError::from_clap)?;
    match parsed.command {
        RawCommand::Install(arguments) => {
            resolve_install_command(arguments, &environment).map(InstallerCommand::Install)
        }
        RawCommand::Restore(arguments) => Ok(InstallerCommand::Restore(RestoreCommand {
            state_dir: resolve_state_dir(arguments.state_dir, &environment)?,
        })),
    }
}

fn resolve_install_command(
    arguments: RawInstallCommand,
    environment: &InstallerEnvironment,
) -> Result<InstallCommand, InstallerError> {
    let codex_home = match arguments.codex_home {
        Some(path) => path,
        None => match &environment.codex_home {
            Some(path) => PathBuf::from(path),
            None => home_path(environment)?.join(".codex"),
        },
    };
    let skills_home = match arguments.skills_home {
        Some(path) => path,
        None => home_path(environment)?.join(".agents/skills"),
    };
    let state_dir = resolve_state_dir(arguments.state_dir, environment)?;

    Ok(InstallCommand {
        dry_run: arguments.dry_run,
        adopt_existing: arguments.adopt_existing,
        agent_threads: arguments.agent_threads,
        codex_home,
        skills_home,
        state_dir,
    })
}

fn resolve_state_dir(
    explicit: Option<PathBuf>,
    environment: &InstallerEnvironment,
) -> Result<PathBuf, InstallerError> {
    if let Some(path) = explicit {
        return Ok(path);
    }

    let state_root = match &environment.xdg_state_home {
        Some(path) => PathBuf::from(path),
        None => home_path(environment)?.join(".local/state"),
    };
    Ok(state_root.join("dotfiles-codex-installer"))
}

fn home_path(environment: &InstallerEnvironment) -> Result<PathBuf, InstallerError> {
    environment
        .home
        .as_ref()
        .map(PathBuf::from)
        .ok_or(InstallerError::MissingHome)
}

fn nonempty_value(value: Option<OsString>) -> Option<OsString> {
    value.filter(|value| !value.is_empty())
}

fn is_option(value: &OsStr) -> bool {
    value.as_encoded_bytes().first() == Some(&b'-')
}

fn parse_agent_threads(value: &str) -> Result<String, String> {
    crate::resources::validate_agent_threads(value).map_err(|_| agent_threads_error())?;
    Ok(value.to_owned())
}

fn agent_threads_error() -> String {
    String::from("--agent-threads must be auto or an integer from 2 to 32")
}

#[cfg(test)]
#[path = "command_tests.rs"]
mod tests;
