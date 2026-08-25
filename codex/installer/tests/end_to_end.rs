mod support;

use std::fs;
use std::path::Path;
use std::process::Command;

use support::process_tempdir;

#[test]
fn managed_bundle_declares_depth_two_and_bounded_orchestration_profiles() {
    // Arrange
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let config_text = fs::read_to_string(source_root.join("config.toml"))
        .expect("read managed Codex configuration");
    let config =
        toml::from_str::<toml::Table>(&config_text).expect("parse managed Codex configuration");
    let agents_config = config
        .get("agents")
        .and_then(toml::Value::as_table)
        .expect("managed configuration has agents table");
    let mut profiles = fs::read_dir(source_root.join("agents"))
        .expect("read managed agent directory")
        .filter_map(|entry| {
            let entry = entry.expect("read managed agent entry");
            if !entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "toml")
            {
                return None;
            }
            let name = entry
                .file_name()
                .into_string()
                .expect("managed agent name is UTF-8");
            let text = fs::read_to_string(entry.path()).expect("read managed agent profile");
            let profile = toml::from_str::<toml::Table>(&text)
                .unwrap_or_else(|error| panic!("parse managed agent {name}: {error}"));
            Some((name, profile))
        })
        .collect::<Vec<_>>();
    profiles.sort_by(|left, right| left.0.cmp(&right.0));

    // Act
    let task_orchestrator = profiles
        .iter()
        .find(|(name, _)| name == "task-orchestrator.toml")
        .map(|(_, profile)| profile)
        .expect("managed Task orchestrator profile");
    let instructions = task_orchestrator
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("Task orchestrator developer instructions");
    let review_integrator = profiles
        .iter()
        .find(|(name, _)| name == "review-integrator.toml")
        .map(|(_, profile)| profile)
        .expect("managed review integrator profile");
    let review_integrator_instructions = review_integrator
        .get("developer_instructions")
        .and_then(toml::Value::as_str)
        .expect("review integrator developer instructions");

    // Assert
    assert_eq!(
        (
            agents_config
                .get("max_threads")
                .and_then(toml::Value::as_integer),
            agents_config
                .get("max_depth")
                .and_then(toml::Value::as_integer),
            task_orchestrator.get("name").and_then(toml::Value::as_str),
            task_orchestrator
                .get("sandbox_mode")
                .and_then(toml::Value::as_str),
        ),
        (
            Some(6),
            Some(2),
            Some("task-orchestrator"),
            Some("read-only")
        )
    );
    for required in [
        "exactly one planned Task Contract",
        "Own only that Task's execute-task loop",
        "Do not edit or commit Task source",
        "root-granted lease",
        "spawn only the implementer, verifier, reviewer, adversarial-integrator, or review-integrator leaves",
        "never grant, expand, or infer your own lease",
        "require findings-only integration followed by receiving-code-review",
        "confirmed Design Escalation",
        "Candidate",
        "Accepted",
        "BLOCKED",
        "Escalate",
        "Each of these statuses ends the current turn",
        "Do not choose another Task, alter Review policy, release dependencies, decide Feature acceptance",
        "publish or merge work",
        "clean or remove a workspace",
    ] {
        assert!(
            instructions.contains(required),
            "Task orchestrator instructions omit {required:?}"
        );
    }
    assert_eq!(
        (
            review_integrator.get("name").and_then(toml::Value::as_str),
            review_integrator.get("model").and_then(toml::Value::as_str),
            review_integrator
                .get("model_reasoning_effort")
                .and_then(toml::Value::as_str),
            review_integrator
                .get("sandbox_mode")
                .and_then(toml::Value::as_str),
        ),
        (
            Some("review-integrator"),
            Some("gpt-5.6-sol"),
            Some("xhigh"),
            Some("read-only")
        )
    );
    for required in [
        "do not spawn descendants",
        "Do not rely on conversation memory",
        "introduced, worsened, merely exposed, or did not cause it",
        "proposed remedy is necessary, proportionate, and inside current authority",
        "Design Doc is missing, contradictory, or materially ambiguous",
        "non-blocking concern",
        "Do not classify final workflow action as Fix, Push back, or Escalate",
    ] {
        assert!(
            review_integrator_instructions.contains(required),
            "review integrator instructions omit {required:?}"
        );
    }
    for (name, profile) in profiles
        .iter()
        .filter(|(name, _)| name != "task-orchestrator.toml")
    {
        let leaf_instructions = profile
            .get("developer_instructions")
            .and_then(toml::Value::as_str)
            .unwrap_or_else(|| panic!("managed leaf {name} has developer instructions"));
        assert!(
            leaf_instructions.contains("do not spawn descendants"),
            "managed leaf {name} must prohibit descendant spawning"
        );
    }
}

#[test]
fn install_and_restore_round_trip_with_normal_binary() {
    // Arrange
    let temporary = process_tempdir("install-restore-round-trip");
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("installer crate must be nested under the Codex source root");
    let source_guidance =
        fs::read(source_root.join("AGENTS.global.md")).expect("read source guidance");
    let source_skill = fs::read(source_root.join("skills/agent-teams-driven-development/SKILL.md"))
        .expect("read representative source skill");
    let source_agent = fs::read(source_root.join("agents/code-reviewer.toml"))
        .expect("read representative source agent");
    let source_task_orchestrator = fs::read(source_root.join("agents/task-orchestrator.toml"))
        .expect("read source Task orchestrator");
    let source_review_integrator = fs::read(source_root.join("agents/review-integrator.toml"))
        .expect("read source review integrator");
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
    let installed_manifest =
        fs::read(state_dir.join("manifest-v1.json")).expect("read installed manifest");
    let stale_exists_after_install = skills_home.join("stale-skill").exists();
    let installed_guidance =
        fs::read(codex_home.join("AGENTS.md")).expect("read installed guidance");
    let installed_skill = fs::read(skills_home.join("agent-teams-driven-development/SKILL.md"))
        .expect("read installed representative skill");
    let installed_agent = fs::read(codex_home.join("agents/code-reviewer.toml"))
        .expect("read installed representative agent");
    let installed_task_orchestrator = fs::read(codex_home.join("agents/task-orchestrator.toml"))
        .expect("read installed Task orchestrator");
    let installed_review_integrator = fs::read(codex_home.join("agents/review-integrator.toml"))
        .expect("read installed review integrator");
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

    // Assert
    assert_eq!(
        (
            install.status.success(),
            String::from_utf8(install.stdout).expect("UTF-8 install stdout"),
            String::from_utf8(install.stderr).expect("UTF-8 install stderr"),
        ),
        (true, "install complete\n".to_owned(), String::new())
    );
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
        ),
        (
            Some("gpt-5.6-sol"),
            Some("xhigh"),
            Some("xhigh"),
            Some(4),
            Some(2),
        )
    );
    assert!(
        installed_config_text.contains(unmanaged_config),
        "unmanaged bytes were not preserved:\n{installed_config_text}"
    );
    assert_ne!(installed_manifest, prior_manifest.as_bytes());
    assert!(!stale_exists_after_install);
    assert_eq!(
        (
            installed_guidance,
            installed_skill,
            installed_agent,
            installed_task_orchestrator,
            installed_review_integrator,
        ),
        (
            source_guidance,
            source_skill,
            source_agent,
            source_task_orchestrator,
            source_review_integrator,
        )
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
    assert_eq!(
        (
            restore.status.success(),
            String::from_utf8(restore.stdout).expect("UTF-8 restore stdout"),
            String::from_utf8(restore.stderr).expect("UTF-8 restore stderr"),
        ),
        (true, "restore complete\n".to_owned(), String::new())
    );
    assert_eq!(
        restored_state,
        (
            prior_config.as_bytes().to_vec(),
            prior_guidance.to_vec(),
            prior_stale_skill.to_vec(),
            prior_manifest.as_bytes().to_vec(),
        )
    );
    assert!(!skills_home.join("agent-teams-driven-development").exists());
    assert!(!codex_home.join("agents/code-reviewer.toml").exists());
    assert!(!codex_home.join("agents/task-orchestrator.toml").exists());
    assert!(!codex_home.join("agents/review-integrator.toml").exists());
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
