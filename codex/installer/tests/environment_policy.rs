mod support;

use std::fs;
use std::process::Command;

use support::{process_tempdir, source_fixture};

#[test]
fn detects_policy_drift_installs_idempotently_and_restores_prior_configuration() {
    // Arrange
    let temporary = process_tempdir("environment-policy-round-trip");
    let source_root = source_fixture(temporary.path());
    let policy = concat!(
        "\n[shell_environment_policy]\ninherit = \"all\"\n",
        "include_only = [\"PATH\",\"HOME\",\"HERDR_*\",\"GHOSTTY_*\",",
        "\"CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS\"]\n",
    );
    let source_config = source_root.join("config.toml");
    let mut managed = fs::read_to_string(&source_config).expect("read source fixture");
    managed.push_str(policy);
    fs::write(&source_config, managed).expect("declare managed policy");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    fs::create_dir(&codex_home).expect("create Codex destination");
    let config_path = codex_home.join("config.toml");
    let prior = concat!(
        "# preserve operator configuration\nmodel_verbosity = \"low\"\n",
        "[shell_environment_policy]\ninherit = \"core\"\n",
        "[shell_environment_policy.set]\n",
        "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS  =  \"1\" # keep override\n",
    );
    fs::write(&config_path, prior).expect("write prior configuration");
    let install = |extra: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_dotfiles-codex-installer"))
            .env_clear()
            .env("HOME", temporary.path().join("unused-home"))
            .env("TMPDIR", temporary.path())
            .env("PATH", "/usr/bin:/bin")
            .arg("--source-root")
            .arg(&source_root)
            .args(["install", "--agent-threads", "4", "--codex-home"])
            .arg(&codex_home)
            .arg("--skills-home")
            .arg(&skills_home)
            .arg("--state-dir")
            .arg(&state_dir)
            .args(extra)
            .output()
            .expect("run isolated installer")
    };

    // Act and assert: preview detects drift without changing destination or state.
    let preview = install(&["--dry-run"]);
    assert!(
        preview.status.success(),
        "{}",
        String::from_utf8_lossy(&preview.stderr)
    );
    assert!(String::from_utf8_lossy(&preview.stdout).contains("REPLACE"));
    assert_eq!(fs::read_to_string(&config_path).unwrap(), prior);
    assert!(!state_dir.exists());

    // Act and assert: install applies the declared values and retains local overrides.
    let applied = install(&[]);
    assert!(
        applied.status.success(),
        "{}",
        String::from_utf8_lossy(&applied.stderr)
    );
    let installed = fs::read_to_string(&config_path).expect("read installed configuration");
    let parsed: toml::Table = toml::from_str(&installed).expect("installed TOML");
    assert_eq!(
        parsed["shell_environment_policy"]["inherit"].as_str(),
        Some("all")
    );
    assert_eq!(
        parsed["shell_environment_policy"]["include_only"],
        toml::Value::Array(
            [
                "PATH",
                "HOME",
                "HERDR_*",
                "GHOSTTY_*",
                "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS"
            ]
            .into_iter()
            .map(|name| toml::Value::String(name.to_owned()))
            .collect()
        )
    );
    assert!(installed.contains("CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS  =  \"1\" # keep override\n"));
    let repeated = install(&[]);
    assert!(repeated.status.success());
    assert_eq!(fs::read_to_string(&config_path).unwrap(), installed);

    // Act and assert: a changed allowlist is detected without overwriting it in dry-run.
    let drifted = installed.replace("\"HERDR_*\"", "\"UNEXPECTED\"");
    assert_ne!(drifted, installed);
    fs::write(&config_path, &drifted).expect("introduce managed-policy drift");
    let preview = install(&["--dry-run"]);
    assert!(preview.status.success());
    assert!(String::from_utf8_lossy(&preview.stdout).contains("REPLACE"));
    assert_eq!(fs::read_to_string(&config_path).unwrap(), drifted);

    // Act and assert: the normal backup restores the original policy and local bytes.
    let restored = Command::new(env!("CARGO_BIN_EXE_dotfiles-codex-installer"))
        .env_clear()
        .env("HOME", temporary.path().join("unused-home"))
        .env("TMPDIR", temporary.path())
        .env("PATH", "/usr/bin:/bin")
        .arg("--source-root")
        .arg(&source_root)
        .args(["restore", "--state-dir"])
        .arg(&state_dir)
        .output()
        .expect("run isolated restore");
    assert!(
        restored.status.success(),
        "{}",
        String::from_utf8_lossy(&restored.stderr)
    );
    assert_eq!(fs::read_to_string(&config_path).unwrap(), prior);
}
