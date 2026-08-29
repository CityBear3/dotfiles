use std::ffi::OsString;
use std::path::PathBuf;

use crate::InstallerError;
use crate::test_support::project_tempdir;

use super::{
    InstallCommand, InstallerCommand, RestoreCommand, parse_command_from_with_environment,
};

#[test]
fn no_subcommand_is_install_shorthand() {
    // Arrange
    let arguments = [
        "installer",
        "--dry-run",
        "--codex-home",
        "/codex",
        "--skills-home",
        "/skills",
        "--state-dir",
        "/state",
    ];

    // Act
    let result = parse_command_from_with_environment(arguments, |_| None)
        .map(|invocation| invocation.command);

    // Assert
    assert_eq!(
        result,
        Ok(InstallerCommand::Install(InstallCommand {
            dry_run: true,
            adopt_existing: false,
            agent_threads: String::from("auto"),
            codex_home: PathBuf::from("/codex"),
            skills_home: PathBuf::from("/skills"),
            state_dir: PathBuf::from("/state"),
        }))
    );
}

#[test]
fn explicit_install_arguments_are_preserved() {
    // Arrange
    let arguments = [
        "installer",
        "install",
        "--dry-run",
        "--adopt-existing",
        "--agent-threads",
        "32",
        "--codex-home",
        "/explicit/codex",
        "--skills-home",
        "/explicit/skills",
        "--state-dir",
        "/explicit/state",
    ];

    // Act
    let result = parse_command_from_with_environment(arguments, |_| None)
        .map(|invocation| invocation.command);

    // Assert
    assert_eq!(
        result,
        Ok(InstallerCommand::Install(InstallCommand {
            dry_run: true,
            adopt_existing: true,
            agent_threads: String::from("32"),
            codex_home: PathBuf::from("/explicit/codex"),
            skills_home: PathBuf::from("/explicit/skills"),
            state_dir: PathBuf::from("/explicit/state"),
        }))
    );
}

#[test]
fn environment_defaults_resolve_install_paths() {
    // Arrange
    let temp = project_tempdir("environment-defaults");
    let home = temp.path().join("home");
    let codex_home = temp.path().join("custom-codex");
    let xdg_state_home = temp.path().join("xdg-state");
    let arguments = ["installer", "install"];

    // Act
    let result = parse_command_from_with_environment(arguments, |name| match name {
        "HOME" => Some(home.clone().into_os_string()),
        "CODEX_HOME" => Some(codex_home.clone().into_os_string()),
        "XDG_STATE_HOME" => Some(xdg_state_home.clone().into_os_string()),
        _ => None,
    })
    .map(|invocation| invocation.command);

    // Assert
    assert_eq!(
        result,
        Ok(InstallerCommand::Install(InstallCommand {
            dry_run: false,
            adopt_existing: false,
            agent_threads: String::from("auto"),
            codex_home,
            skills_home: home.join(".agents/skills"),
            state_dir: xdg_state_home.join("dotfiles-codex-installer"),
        }))
    );
}

#[test]
fn home_defaults_resolve_install_paths() {
    // Arrange
    let temp = project_tempdir("home-defaults");
    let home = temp.path().join("home");
    let arguments = ["installer", "install"];

    // Act
    let result = parse_command_from_with_environment(arguments, |name| {
        (name == "HOME").then(|| home.clone().into_os_string())
    })
    .map(|invocation| invocation.command);

    // Assert
    assert_eq!(
        result,
        Ok(InstallerCommand::Install(InstallCommand {
            dry_run: false,
            adopt_existing: false,
            agent_threads: String::from("auto"),
            codex_home: home.join(".codex"),
            skills_home: home.join(".agents/skills"),
            state_dir: home.join(".local/state/dotfiles-codex-installer"),
        }))
    );
}

#[test]
fn restore_resolves_the_latest_selection_from_state_directory() {
    // Arrange
    let arguments = ["installer", "restore", "--state-dir", "/explicit/state"];

    // Act
    let result = parse_command_from_with_environment(arguments, |_| None)
        .map(|invocation| invocation.command);

    // Assert
    assert_eq!(
        result,
        Ok(InstallerCommand::Restore(RestoreCommand {
            state_dir: PathBuf::from("/explicit/state"),
        }))
    );
}

#[test]
fn restore_uses_xdg_state_default_without_home() {
    // Arrange
    let temp = project_tempdir("restore-xdg-default");
    let xdg_state_home = temp.path().join("xdg-state");
    let arguments = ["installer", "restore"];

    // Act
    let result = parse_command_from_with_environment(arguments, |name| {
        (name == "XDG_STATE_HOME").then(|| xdg_state_home.clone().into_os_string())
    })
    .map(|invocation| invocation.command);

    // Assert
    assert_eq!(
        result,
        Ok(InstallerCommand::Restore(RestoreCommand {
            state_dir: xdg_state_home.join("dotfiles-codex-installer"),
        }))
    );
}

#[test]
fn restore_rejects_an_arbitrary_backup_path() {
    // Arrange
    let arguments = ["installer", "restore", "--backup", "/older/backup"];

    // Act
    let result = parse_command_from_with_environment(arguments, |_| None);

    // Assert
    let error = result.expect_err("restore must not accept an arbitrary backup path");
    assert_eq!((error.exit_code(), error.use_stderr()), (1, true));
    assert!(
        error.to_string().contains("unexpected argument '--backup'"),
        "unexpected CLI error: {error}"
    );
}

#[test]
fn recover_subcommand_is_rejected() {
    // Arrange
    let arguments = ["installer", "recover"];

    // Act
    let result = parse_command_from_with_environment(arguments, |_| None);

    // Assert
    let error = result.expect_err("recover must remain automatic rather than explicit");
    assert_eq!((error.exit_code(), error.use_stderr()), (1, true));
    assert!(
        error
            .to_string()
            .contains("unrecognized subcommand 'recover'"),
        "unexpected CLI error: {error}"
    );
}

#[test]
fn invalid_agent_thread_input_is_rejected() {
    // Arrange
    let arguments = [
        "installer",
        "install",
        "--agent-threads",
        "1",
        "--codex-home",
        "/codex",
        "--skills-home",
        "/skills",
        "--state-dir",
        "/state",
    ];

    // Act
    let result = parse_command_from_with_environment(arguments, |_| None);

    // Assert
    let error = result.expect_err("thread input below two must be rejected");
    assert_eq!((error.exit_code(), error.use_stderr()), (1, true));
    assert!(
        error
            .to_string()
            .contains("--agent-threads must be auto or an integer from 2 to 32"),
        "unexpected CLI error: {error}"
    );
}

#[test]
fn help_returns_successful_stdout_exit_data() {
    // Arrange
    let arguments = ["installer", "--help"];

    // Act
    let result = parse_command_from_with_environment(arguments, |_| None);

    // Assert
    let error = result.expect_err("clap returns help as display data");
    assert_eq!((error.exit_code(), error.use_stderr()), (0, false));
    assert!(
        error.to_string().contains("Usage: installer install"),
        "unexpected help text: {error}"
    );
}

#[test]
fn invalid_option_returns_failure_stderr_exit_data() {
    // Arrange
    let arguments = ["installer", "--unknown-option"];

    // Act
    let result = parse_command_from_with_environment(arguments, |_| None);

    // Assert
    let error = result.expect_err("unknown options must fail parsing");
    assert_eq!((error.exit_code(), error.use_stderr()), (1, true));
    assert!(
        error
            .to_string()
            .contains("unexpected argument '--unknown-option'"),
        "unexpected CLI error: {error}"
    );
}

#[test]
fn missing_home_reports_a_structured_error() {
    // Arrange
    let arguments = ["installer", "install"];

    // Act
    let result = parse_command_from_with_environment(arguments, |_| Some(OsString::new()));

    // Assert
    assert_eq!(result, Err(InstallerError::MissingHome));
}

#[test]
fn parsed_install_dispatches_to_the_mutating_application() {
    // Arrange
    let temporary = project_tempdir("command-mutating-dispatch");
    let codex_home = temporary.path().join("missing-codex");
    let skills_home = temporary.path().join("skills");
    let state_dir = temporary.path().join("state");
    let arguments = vec![
        OsString::from("installer"),
        OsString::from("--codex-home"),
        codex_home.clone().into_os_string(),
        OsString::from("--skills-home"),
        skills_home.clone().into_os_string(),
        OsString::from("--state-dir"),
        state_dir.clone().into_os_string(),
    ];

    // Act
    let result = crate::run_from(arguments, |_| None);

    // Assert
    assert!(matches!(result, Err(InstallerError::Lock { .. })));
    assert_eq!(
        (
            codex_home.exists(),
            skills_home.exists(),
            state_dir.exists()
        ),
        (false, false, false)
    );
}

#[test]
fn parsed_restore_dispatches_to_the_mutating_application() {
    // Arrange
    let temporary = project_tempdir("command-restore-dispatch");
    let state_dir = temporary.path().join("missing-state");
    let arguments = vec![
        OsString::from("installer"),
        OsString::from("restore"),
        OsString::from("--state-dir"),
        state_dir.clone().into_os_string(),
    ];

    // Act
    let result = crate::run_from(arguments, |_| None);

    // Assert
    assert_eq!(
        result,
        Err(InstallerError::InvalidBackup {
            message: "restore requires a selected latest backup".to_owned(),
        })
    );
    assert!(!state_dir.exists());
}

#[test]
fn invalid_command_is_rejected_before_unimplemented_dispatch() {
    // Arrange
    let arguments = ["installer", "recover"];

    // Act
    let result = crate::run_from(arguments, |_| None);

    // Assert
    let error = result.expect_err("invalid input must fail before dispatch");
    assert_eq!((error.exit_code(), error.use_stderr()), (1, true));
    assert!(
        error
            .to_string()
            .contains("unrecognized subcommand 'recover'"),
        "unexpected CLI error: {error}"
    );
}
