mod support;

use std::fs;
use std::process::Command;

use support::{process_tempdir, source_fixture};

#[test]
fn compiled_binary_dry_run_is_non_mutating() {
    // Arrange
    let temporary = process_tempdir("compiled-dry-run");
    let source_root = source_fixture(temporary.path());
    let destination_root = temporary.path().join("test-destination");
    assert!(
        !destination_root.exists(),
        "destination root must start absent"
    );
    let home = destination_root.clone();
    let codex_home = destination_root.join("codex-home");
    let skills_home = destination_root.join("skills-home");
    let state_root = destination_root.join("xdg-state");
    let state_dir = state_root.join("installer");
    let output = Command::new(env!("CARGO_BIN_EXE_dotfiles-codex-installer"))
        .env_clear()
        .env("HOME", &home)
        .env("CODEX_HOME", &codex_home)
        .env("XDG_STATE_HOME", &state_root)
        .env("TMPDIR", temporary.path())
        .env("PATH", "/usr/bin:/bin")
        .arg("--source-root")
        .arg(&source_root)
        .args([
            "install",
            "--dry-run",
            "--agent-threads",
            "6",
            "--codex-home",
        ])
        .arg(&codex_home)
        .arg("--skills-home")
        .arg(&skills_home)
        .arg("--state-dir")
        .arg(&state_dir)
        .output()
        .expect("run compiled installer");

    // Act
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 stdout");
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 stderr");

    // Assert
    assert!(
        output.status.success(),
        "status={:?}\nstdout={stdout}\nstderr={stderr}",
        output.status.code()
    );
    assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
    assert!(stdout.starts_with("Dry run · max threads 6\n\n"));
    assert!(stdout.contains("STATUS  ACTION"));
    assert!(stdout.contains("ASSET"));
    assert!(stdout.contains("PATH"));
    assert!(stdout.contains("•"));
    assert!(stdout.contains("CREATE"));
    assert!(stdout.contains("config"));
    assert!(stdout.contains("global-agents"));
    assert!(stdout.contains("skill/fixture-skill"));
    assert!(stdout.contains("agent/fixture-agent.toml"));
    assert!(stdout.contains("manifest"));
    assert!(stdout.contains("~/codex-home/config.toml"));
    assert!(stdout.contains("~/skills-home/fixture-skill"));
    assert!(stdout.contains("~/xdg-state/installer/manifest-v1.json"));
    assert!(!stdout.contains('✓'));
    assert!(!stdout.contains('🍺'));
    assert!(!stdout.contains('\u{1b}'));
    assert_eq!(
        (
            home.exists(),
            codex_home.exists(),
            skills_home.exists(),
            state_root.exists(),
            codex_home.join("codex-manifest-installer.lock").exists(),
            destination_root.exists(),
        ),
        (false, false, false, false, false, false)
    );
}

#[test]
fn compiled_binary_dry_run_escapes_path_controls_without_faking_output_lines() {
    // Arrange
    let temporary = process_tempdir("compiled-dry-run-controls");
    let source_root = source_fixture(temporary.path());
    let destination_root = temporary.path().join("control-destination");
    let home = destination_root.clone();
    let codex_home = destination_root.join(concat!(
        "codex\n🍺 Install complete\rSTATUS ACTION\t",
        "\u{1b}[31m\\literal",
    ));
    let skills_home = destination_root.join("skills-home");
    let state_root = destination_root.join("xdg-state");
    let state_dir = state_root.join("installer");

    let output = Command::new(env!("CARGO_BIN_EXE_dotfiles-codex-installer"))
        .env_clear()
        .env("HOME", &home)
        .env("CODEX_HOME", &codex_home)
        .env("XDG_STATE_HOME", &state_root)
        .env("TMPDIR", temporary.path())
        .env("PATH", "/usr/bin:/bin")
        .arg("--source-root")
        .arg(&source_root)
        .args([
            "install",
            "--dry-run",
            "--agent-threads",
            "6",
            "--codex-home",
        ])
        .arg(&codex_home)
        .arg("--skills-home")
        .arg(&skills_home)
        .arg("--state-dir")
        .arg(&state_dir)
        .output()
        .expect("run compiled installer with path controls");

    // Act
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 stdout");
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 stderr");

    // Assert
    assert_eq!(output.status.code(), Some(0));
    assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
    assert_eq!(stdout.lines().count(), 9);
    assert!(stdout.contains(concat!(
        "~/codex\\n🍺 Install complete\\rSTATUS ACTION\\t",
        "\\x1B[31m\\\\literal/config.toml",
    )));
    assert!(!stdout.contains('\r'));
    assert!(!stdout.contains('\t'));
    assert!(!stdout.contains('\u{1b}'));
    assert!(!stdout.lines().any(|line| line.starts_with('🍺')));
    assert_eq!(
        (
            home.exists(),
            codex_home.exists(),
            skills_home.exists(),
            state_root.exists(),
            codex_home.join("codex-manifest-installer.lock").exists(),
        ),
        (false, false, false, false, false)
    );
}

#[test]
fn compiled_binary_help_uses_successful_stdout() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_dotfiles-codex-installer"));
    command.env_clear().arg("--help");

    // Act
    let output = command.output().expect("run compiled installer help");
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 stdout");
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 stderr");

    // Assert
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        stdout,
        concat!(
            "Usage: dotfiles-codex-installer install [OPTIONS]\n",
            "\n",
            "Options:\n",
            "      --dry-run                      \n",
            "      --adopt-existing               \n",
            "      --agent-threads <auto|2..=32>  [default: auto]\n",
            "      --codex-home <PATH>            \n",
            "      --skills-home <PATH>           \n",
            "      --state-dir <PATH>             \n",
            "  -h, --help                         Print help\n",
        )
    );
    assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
}

#[test]
fn compiled_binary_missing_home_uses_failure_stderr() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_dotfiles-codex-installer"));
    command.env_clear();

    // Act
    let output = command
        .output()
        .expect("run compiled installer without HOME");
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 stdout");
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 stderr");

    // Assert
    assert_eq!(output.status.code(), Some(1));
    assert!(stdout.is_empty(), "unexpected stdout: {stdout}");
    assert_eq!(
        stderr,
        "✗  HOME must be set to resolve installer defaults\n"
    );
}

#[test]
fn compiled_binary_dry_run_conflict_uses_exit_two_stderr_without_mutation() {
    // Arrange
    let temporary = process_tempdir("compiled-dry-run-conflict");
    let source_root = source_fixture(temporary.path());
    let destination_root = temporary.path().join(concat!(
        "conflict-destination\n🍺 Install complete\rSTATUS ACTION\t",
        "\u{1b}\u{85}\\literal",
    ));
    let home = destination_root.join("home");
    let codex_home = destination_root.join("codex-home");
    let skills_home = destination_root.join("skills-home");
    let state_root = destination_root.join("xdg-state");
    let state_dir = state_root.join("installer");
    let conflict = codex_home.join("AGENTS.md");
    fs::create_dir_all(&codex_home).expect("create conflicting Codex home");
    fs::write(&conflict, b"local guidance\n").expect("write unmanaged conflict");
    assert!(conflict.is_absolute(), "conflict path must be absolute");

    let output = Command::new(env!("CARGO_BIN_EXE_dotfiles-codex-installer"))
        .env_clear()
        .env("HOME", &home)
        .env("CODEX_HOME", &codex_home)
        .env("XDG_STATE_HOME", &state_root)
        .env("TMPDIR", temporary.path())
        .env("PATH", "/usr/bin:/bin")
        .arg("--source-root")
        .arg(&source_root)
        .args([
            "install",
            "--dry-run",
            "--agent-threads",
            "6",
            "--codex-home",
        ])
        .arg(&codex_home)
        .arg("--skills-home")
        .arg(&skills_home)
        .arg("--state-dir")
        .arg(&state_dir)
        .output()
        .expect("run compiled installer dry-run conflict");

    // Act
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 stdout");
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 stderr");
    let escaped_paths = format!("{:?}", vec![&conflict]).replace('\\', "\\\\");

    // Assert
    assert_eq!(output.status.code(), Some(2));
    assert!(stdout.is_empty(), "unexpected stdout: {stdout}");
    assert_eq!(
        stderr,
        format!("✗  unmanaged destination conflicts: {escaped_paths}\n")
    );
    assert_eq!(stderr.lines().count(), 1);
    assert!(!stderr.contains('\r'));
    assert!(!stderr.contains('\t'));
    assert!(
        !stderr
            .lines()
            .any(|line| line.starts_with("STATUS  ACTION"))
    );
    assert!(!stderr.lines().any(|line| line.starts_with('🍺')));
    assert!(!stderr.contains('\u{1b}'));
    assert_eq!(
        (
            fs::read(&conflict).expect("read preserved unmanaged conflict"),
            home.exists(),
            skills_home.exists(),
            state_root.exists(),
            codex_home.join("codex-manifest-installer.lock").exists(),
        ),
        (b"local guidance\n".to_vec(), false, false, false, false)
    );
}
