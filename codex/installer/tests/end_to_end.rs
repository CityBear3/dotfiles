mod support;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use support::{process_tempdir, source_fixture};

#[test]
fn workflow_bundle_replaces_owned_roles_and_preserves_unmanaged_assets() {
    // Arrange: exercise the real repository bundle and normal installer process.
    let temporary = process_tempdir("workflow-bundle-migration");
    let repository_bundle = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer is inside the Codex source bundle");
    let source_root = temporary.path().join("source-bundle");
    fs::create_dir(&source_root).expect("create isolated source snapshot");
    for name in ["config.toml", "AGENTS.global.md"] {
        fs::copy(repository_bundle.join(name), source_root.join(name))
            .expect("snapshot repository source file");
    }
    for category in ["agents", "skills"] {
        for (relative, bytes) in directory_files(&repository_bundle.join(category)) {
            let destination = source_root.join(category).join(relative);
            fs::create_dir_all(destination.parent().expect("snapshot file parent"))
                .expect("create source snapshot directory");
            fs::write(destination, bytes).expect("snapshot repository asset");
        }
    }
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    fs::create_dir_all(codex_home.join("agents")).expect("create isolated agents root");
    fs::create_dir_all(skills_home.join("unmanaged")).expect("create unmanaged skill");
    fs::create_dir_all(&state_dir).expect("create isolated installer state");
    let retired = [
        "implementer",
        "implementation-verifier",
        "code-reviewer",
        "code-quality-reviewer",
        "test-coverage-reviewer",
        "scope-reviewer",
        "code-architect",
        "adversarial-api-reviewer",
        "adversarial-performance-reviewer",
        "adversarial-robustness-reviewer",
        "adversarial-tests-reviewer",
        "adversarial-integrator",
        "review-integrator",
        "task-orchestrator",
    ];
    let owned_agents = retired.map(|name| format!("{name}.toml"));
    for name in &owned_agents {
        fs::write(codex_home.join("agents").join(name), b"old managed role\n")
            .expect("write old managed role");
    }
    let manifest = serde_json::json!({
        "version": 1,
        "global_agents": false,
        "skills": [],
        "agents": owned_agents,
    });
    fs::write(
        state_dir.join("manifest-v1.json"),
        serde_json::to_vec(&manifest).expect("serialize prior ownership"),
    )
    .expect("write prior ownership");
    let unmanaged_agent = codex_home.join("agents/unmanaged.toml");
    let unmanaged_skill = skills_home.join("unmanaged/SKILL.md");
    let system_sentinel = codex_home.join("skills/.system/sentinel");
    fs::create_dir_all(system_sentinel.parent().expect("system sentinel parent"))
        .expect("create isolated system directory");
    fs::write(&unmanaged_agent, b"unmanaged agent\n").expect("write unrelated agent");
    fs::write(&unmanaged_skill, b"unmanaged skill\n").expect("write unrelated skill");
    fs::write(&system_sentinel, b"system-owned\n").expect("write system sentinel");
    fs::write(
        codex_home.join("config.toml"),
        b"# operator-owned configuration\nmodel_verbosity = \"low\"\n",
    )
    .expect("write unmanaged configuration");

    // Act
    let output = Command::new(env!("CARGO_BIN_EXE_dotfiles-codex-installer"))
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
        .output()
        .expect("run real installer on isolated destinations");

    // Assert: installed consumers receive the complete new bundle, not stale roles.
    assert!(
        output.status.success(),
        "install failed: {}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let source_agents = directory_files(&source_root.join("agents"));
    let mut installed_agents = directory_files(&codex_home.join("agents"));
    installed_agents.remove(Path::new("unmanaged.toml"));
    assert_eq!(installed_agents, source_agents);
    let source_skills = directory_files(&source_root.join("skills"));
    let mut installed_skills = directory_files(&skills_home);
    installed_skills.remove(Path::new("unmanaged/SKILL.md"));
    assert_eq!(installed_skills, source_skills);
    assert!(
        installed_skills.contains_key(Path::new("execute-task/references/task-lead.md")),
        "independent roots need their installed shared role contract"
    );
    let bindings = installed_agents
        .iter()
        .map(|(path, bytes)| {
            let profile: toml::Table =
                toml::from_str(std::str::from_utf8(bytes).expect("installed profile is UTF-8"))
                    .expect("installed profile is TOML");
            let string = |key: &str| {
                profile
                    .get(key)
                    .and_then(toml::Value::as_str)
                    .expect("profile binding is a string")
                    .to_owned()
            };
            assert_eq!(
                path.file_stem().and_then(|stem| stem.to_str()),
                Some(string("name").as_str()),
                "runtime role identity must match its installed filename"
            );
            assert!(!string("developer_instructions").trim().is_empty());
            (
                string("name"),
                (
                    string("model"),
                    string("model_reasoning_effort"),
                    string("sandbox_mode"),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let expected_bindings = [
        (
            "verification-runner",
            "gpt-5.6-luna",
            "low",
            "workspace-write",
        ),
        ("focused-reviewer", "gpt-5.6-sol", "high", "read-only"),
        ("spec-reviewer", "gpt-5.6-sol", "high", "read-only"),
        (
            "implementation-quality-reviewer",
            "gpt-5.6-sol",
            "high",
            "read-only",
        ),
        ("risk-reviewer", "gpt-5.6-sol", "xhigh", "read-only"),
        ("finding-integrator", "gpt-5.6-sol", "high", "read-only"),
        (
            "design-alignment-reviewer",
            "gpt-5.6-sol",
            "xhigh",
            "read-only",
        ),
    ]
    .into_iter()
    .map(|(role, model, effort, sandbox)| {
        (
            role.to_owned(),
            (model.to_owned(), effort.to_owned(), sandbox.to_owned()),
        )
    })
    .collect::<BTreeMap<_, _>>();
    assert_eq!(bindings, expected_bindings);
    assert_eq!(
        fs::read(&unmanaged_agent).expect("read unrelated agent"),
        b"unmanaged agent\n"
    );
    assert_eq!(
        fs::read(&unmanaged_skill).expect("read unrelated skill"),
        b"unmanaged skill\n"
    );
    assert_eq!(
        fs::read(&system_sentinel).expect("read system sentinel"),
        b"system-owned\n"
    );
    let config =
        fs::read_to_string(codex_home.join("config.toml")).expect("read installed configuration");
    assert!(config.contains("# operator-owned configuration\nmodel_verbosity = \"low\"\n"));
    let config: toml::Table = toml::from_str(&config).expect("installed configuration is TOML");
    assert_eq!(config["agents"]["max_threads"].as_integer(), Some(4));
    assert_eq!(
        config["features"]["context_management"]["experimental_mode"].as_bool(),
        Some(true)
    );
}

fn directory_files(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, current: &Path, files: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(current).expect("read bundle directory") {
            let entry = entry.expect("read bundle entry");
            let path = entry.path();
            let kind = entry.file_type().expect("read bundle entry type");
            if kind.is_dir() {
                visit(root, &path, files);
            } else {
                assert!(kind.is_file(), "bundle contains a non-file entry: {path:?}");
                files.insert(
                    path.strip_prefix(root)
                        .expect("entry belongs to bundle")
                        .to_owned(),
                    fs::read(&path).expect("read bundle file"),
                );
            }
        }
    }
    let mut files = BTreeMap::new();
    visit(root, root, &mut files);
    files
}

#[test]
fn install_and_restore_round_trip_with_normal_binary() {
    // Arrange
    let temporary = process_tempdir("install-restore-round-trip");
    let source_root = source_fixture(temporary.path());
    let source_guidance =
        fs::read(source_root.join("AGENTS.global.md")).expect("read source guidance");
    let source_skill =
        fs::read(source_root.join("skills/fixture-skill/SKILL.md")).expect("read source skill");
    let source_agent =
        fs::read(source_root.join("agents/fixture-agent.toml")).expect("read source agent");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    for directory in [&codex_home, &skills_home, &state_dir] {
        fs::create_dir(directory).expect("create end-to-end root");
    }
    let unmanaged_config = concat!(
        "# preserve this unmanaged block byte-for-byte\n",
        "model_context_window = 123456\n",
        "statusline = [\"model\", \"context\"]\n",
    );
    let prior_config = concat!(
        "model = \"old-model\"\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "# preserve this unmanaged block byte-for-byte\n",
        "model_context_window = 123456\n",
        "statusline = [\"model\", \"context\"]\n",
        "\n",
        "[agents]\n",
        "max_threads = 2\n",
        "max_depth = 3\n",
        "\n",
        "[features]\n",
        "hooks  =  true # keep unrelated features\n",
        "\n",
        "[features.context_management]\n",
        "experimental_mode = false # keep context-management comment\n",
    );
    let prior_manifest = concat!(
        "{\n",
        "  \"version\": 1,\n",
        "  \"global_agents\": true,\n",
        "  \"skills\": [\n",
        "    \"stale-skill\"\n",
        "  ],\n",
        "  \"agents\": []\n",
        "}\n",
    );
    let prior_guidance = b"prior global guidance\n";
    let prior_stale_skill = b"prior stale skill\n";
    let unrelated_skill = skills_home.join("unrelated-skill/SKILL.md");
    let unrelated_agent = codex_home.join("agents/unrelated-agent.toml");
    let system_sentinel = codex_home.join("skills/.system/sentinel");
    fs::write(codex_home.join("config.toml"), prior_config).expect("write prior config");
    fs::write(codex_home.join("AGENTS.md"), prior_guidance).expect("write prior guidance");
    fs::create_dir(skills_home.join("stale-skill")).expect("create stale owned skill");
    fs::write(skills_home.join("stale-skill/SKILL.md"), prior_stale_skill)
        .expect("write stale owned skill");
    fs::create_dir(skills_home.join("unrelated-skill")).expect("create unrelated skill");
    fs::write(&unrelated_skill, b"unrelated skill\n").expect("write unrelated skill");
    fs::create_dir(codex_home.join("agents")).expect("create Codex agents directory");
    fs::write(&unrelated_agent, b"unrelated agent\n").expect("write unrelated agent");
    fs::create_dir_all(codex_home.join("skills/.system"))
        .expect("create Codex system skills directory");
    fs::write(&system_sentinel, b"system sentinel\n").expect("write system sentinel");
    fs::write(state_dir.join("manifest-v1.json"), prior_manifest)
        .expect("write prior ownership manifest");

    // Act
    let install = Command::new(env!("CARGO_BIN_EXE_dotfiles-codex-installer"))
        .env_clear()
        .env("HOME", temporary.path().join("home"))
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
        .output()
        .expect("run normal binary install");
    let installed_config = fs::read(codex_home.join("config.toml")).expect("read installed config");
    let installed_config_text =
        String::from_utf8(installed_config.clone()).expect("installed config is UTF-8");
    let installed_config_table =
        toml::from_str::<toml::Table>(&installed_config_text).expect("parse installed config");
    let installed_agents = installed_config_table
        .get("agents")
        .and_then(toml::Value::as_table)
        .expect("installed config has agents table");
    let installed_update_plan = installed_config_table
        .get("tools")
        .and_then(toml::Value::as_table)
        .and_then(|tools| tools.get("update_plan"))
        .and_then(toml::Value::as_table)
        .expect("installed config has tools.update_plan table");
    let installed_manifest =
        fs::read(state_dir.join("manifest-v1.json")).expect("read installed manifest");
    let stale_exists_after_install = skills_home.join("stale-skill").exists();
    let installed_guidance =
        fs::read(codex_home.join("AGENTS.md")).expect("read installed guidance");
    let installed_skill =
        fs::read(skills_home.join("fixture-skill/SKILL.md")).expect("read installed skill");
    let installed_agent =
        fs::read(codex_home.join("agents/fixture-agent.toml")).expect("read installed agent");
    let unrelated_after_install = (
        fs::read(&unrelated_skill).expect("read unrelated skill after install"),
        fs::read(&unrelated_agent).expect("read unrelated agent after install"),
        fs::read(&system_sentinel).expect("read system sentinel after install"),
    );
    let backups_dir = state_dir.join("backups");
    let latest_after_install =
        fs::read(backups_dir.join("latest")).expect("read latest marker after install");
    let selected_backup_id = std::str::from_utf8(&latest_after_install)
        .expect("latest marker is UTF-8")
        .strip_suffix('\n')
        .expect("latest marker ends with newline")
        .to_owned();
    let selected_backup = backups_dir.join(&selected_backup_id);
    let mut backup_directories_after_install = fs::read_dir(&backups_dir)
        .expect("read backups after install")
        .filter_map(|entry| {
            let entry = entry.expect("read backup entry after install");
            entry
                .file_type()
                .expect("read backup entry type after install")
                .is_dir()
                .then(|| entry.path())
        })
        .collect::<Vec<_>>();
    backup_directories_after_install.sort();
    let journal_after_install =
        fs::read(selected_backup.join("journal-v1.json")).expect("read selected backup journal");
    let payload_root = selected_backup.join("payload");
    let payload_after_install = (
        fs::read(payload_root.join("codex-home/config.toml")).expect("read backed up config"),
        fs::read(payload_root.join("codex-home/AGENTS.md")).expect("read backed up guidance"),
        fs::read(payload_root.join("skills-home/stale-skill/SKILL.md"))
            .expect("read backed up stale skill"),
        fs::read(payload_root.join("state-dir/manifest-v1.json")).expect("read backed up manifest"),
    );
    let wal_exists_after_install = state_dir.join("transaction/wal-v1.json").exists();
    let work_is_empty_after_install = fs::read_dir(state_dir.join("transaction/work"))
        .expect("read transaction work after install")
        .next()
        .is_none();
    let restore = Command::new(env!("CARGO_BIN_EXE_dotfiles-codex-installer"))
        .env_clear()
        .env("HOME", temporary.path().join("home"))
        .env("TMPDIR", temporary.path())
        .env("PATH", "/usr/bin:/bin")
        .arg("--source-root")
        .arg(&source_root)
        .args(["restore", "--state-dir"])
        .arg(&state_dir)
        .output()
        .expect("run normal binary restore");
    let restored_state = (
        fs::read(codex_home.join("config.toml")).expect("read restored config"),
        fs::read(codex_home.join("AGENTS.md")).expect("read restored guidance"),
        fs::read(skills_home.join("stale-skill/SKILL.md")).expect("read restored stale skill"),
        fs::read(state_dir.join("manifest-v1.json")).expect("read restored manifest"),
    );
    let unrelated_after_restore = (
        fs::read(&unrelated_skill).expect("read unrelated skill after restore"),
        fs::read(&unrelated_agent).expect("read unrelated agent after restore"),
        fs::read(&system_sentinel).expect("read system sentinel after restore"),
    );
    let latest_after_restore =
        fs::read(backups_dir.join("latest")).expect("read latest marker after restore");
    let journal_after_restore =
        fs::read(selected_backup.join("journal-v1.json")).expect("read journal after restore");
    let payload_after_restore = (
        fs::read(payload_root.join("codex-home/config.toml")).expect("reread backed up config"),
        fs::read(payload_root.join("codex-home/AGENTS.md")).expect("reread backed up guidance"),
        fs::read(payload_root.join("skills-home/stale-skill/SKILL.md"))
            .expect("reread backed up stale skill"),
        fs::read(payload_root.join("state-dir/manifest-v1.json"))
            .expect("reread backed up manifest"),
    );
    let mut backup_directories_after_restore = fs::read_dir(&backups_dir)
        .expect("read backups after restore")
        .filter_map(|entry| {
            let entry = entry.expect("read backup entry after restore");
            entry
                .file_type()
                .expect("read backup entry type after restore")
                .is_dir()
                .then(|| entry.path())
        })
        .collect::<Vec<_>>();
    backup_directories_after_restore.sort();
    let wal_exists_after_restore = state_dir.join("transaction/wal-v1.json").exists();
    let work_is_empty_after_restore = fs::read_dir(state_dir.join("transaction/work"))
        .expect("read transaction work after restore")
        .next()
        .is_none();
    let install_stdout = String::from_utf8(install.stdout.clone()).expect("UTF-8 install stdout");
    let install_stderr = String::from_utf8(install.stderr.clone()).expect("UTF-8 install stderr");
    let restore_stdout = String::from_utf8(restore.stdout.clone()).expect("UTF-8 restore stdout");
    let restore_stderr = String::from_utf8(restore.stderr.clone()).expect("UTF-8 restore stderr");

    // Assert
    assert!(install.status.success(), "unexpected install failure");
    assert!(
        install_stderr.is_empty(),
        "unexpected stderr: {install_stderr}"
    );
    assert!(install_stdout.starts_with("STATUS  ACTION"));
    assert!(install_stdout.contains("✓"));
    assert!(install_stdout.contains("REPLACE"));
    assert!(install_stdout.contains("REMOVE"));
    assert!(install_stdout.contains("skill/fixture-skill"));
    assert!(install_stdout.contains("agent/fixture-agent.toml"));
    assert!(install_stdout.contains("🍺 Install complete ·"));
    assert!(install_stdout.contains(" changed · "));
    assert!(install_stdout.contains(" unchanged\n"));
    assert!(!install_stdout.contains('\u{1b}'));
    assert_eq!(
        (
            installed_config_table
                .get("model")
                .and_then(toml::Value::as_str),
            installed_config_table
                .get("model_reasoning_effort")
                .and_then(toml::Value::as_str),
            installed_config_table
                .get("plan_mode_reasoning_effort")
                .and_then(toml::Value::as_str),
            installed_agents
                .get("max_threads")
                .and_then(toml::Value::as_integer),
            installed_agents
                .get("max_depth")
                .and_then(toml::Value::as_integer),
            installed_update_plan
                .get("enabled")
                .and_then(toml::Value::as_bool),
            installed_config_table
                .get("features")
                .and_then(|features| features.get("context_management"))
                .and_then(|context| context.get("experimental_mode"))
                .and_then(toml::Value::as_bool),
        ),
        (
            Some("fixture-model"),
            Some("medium"),
            Some("high"),
            Some(4),
            Some(2),
            Some(true),
            Some(true),
        )
    );
    assert!(
        installed_config_text.contains(unmanaged_config),
        "unmanaged bytes were not preserved:\n{installed_config_text}"
    );
    assert!(installed_config_text.contains("hooks  =  true # keep unrelated features\n"));
    assert!(
        installed_config_text
            .contains("experimental_mode = true # keep context-management comment\n")
    );
    assert_ne!(installed_manifest, prior_manifest.as_bytes());
    assert!(!stale_exists_after_install);
    assert_eq!(
        (installed_guidance, installed_skill, installed_agent),
        (source_guidance, source_skill, source_agent)
    );
    assert_eq!(
        unrelated_after_install,
        (
            b"unrelated skill\n".to_vec(),
            b"unrelated agent\n".to_vec(),
            b"system sentinel\n".to_vec(),
        )
    );
    assert_eq!(
        backup_directories_after_install,
        vec![selected_backup.clone()]
    );
    assert_eq!(
        payload_after_install,
        (
            prior_config.as_bytes().to_vec(),
            prior_guidance.to_vec(),
            prior_stale_skill.to_vec(),
            prior_manifest.as_bytes().to_vec(),
        )
    );
    assert!(!journal_after_install.is_empty());
    assert!(!wal_exists_after_install);
    assert!(work_is_empty_after_install);
    assert!(restore.status.success(), "unexpected restore failure");
    assert!(
        restore_stderr.is_empty(),
        "unexpected stderr: {restore_stderr}"
    );
    assert!(restore_stdout.starts_with("STATUS  ACTION"));
    assert!(restore_stdout.contains("✓"));
    assert!(restore_stdout.contains("REPLACE"));
    assert!(restore_stdout.contains("REMOVE"));
    assert!(restore_stdout.contains("skill/fixture-skill"));
    assert!(restore_stdout.contains("agent/fixture-agent.toml"));
    assert!(restore_stdout.contains("🍺 Restore complete ·"));
    assert!(restore_stdout.contains(" changed · "));
    assert!(restore_stdout.contains(" unchanged\n"));
    assert!(!restore_stdout.contains('\u{1b}'));
    assert_eq!(
        restored_state,
        (
            prior_config.as_bytes().to_vec(),
            prior_guidance.to_vec(),
            prior_stale_skill.to_vec(),
            prior_manifest.as_bytes().to_vec(),
        )
    );
    assert!(!skills_home.join("fixture-skill").exists());
    assert!(!codex_home.join("agents/fixture-agent.toml").exists());
    assert_eq!(unrelated_after_restore, unrelated_after_install);
    assert_eq!(latest_after_restore, latest_after_install);
    assert_eq!(journal_after_restore, journal_after_install);
    assert_eq!(payload_after_restore, payload_after_install);
    assert_eq!(
        backup_directories_after_restore,
        backup_directories_after_install
    );
    assert!(!wal_exists_after_restore);
    assert!(work_is_empty_after_restore);
}
