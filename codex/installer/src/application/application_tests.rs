use std::fs;

use crate::command::{InstallCommand, InstallerCommand};
use crate::resources::MachineResources;
use crate::test_support::project_tempdir;

use super::install::{ApplicationContext, execute_with_context};

const MANAGED_CONFIG: &str = concat!(
    "model = \"gpt-5.6\"\n",
    "model_reasoning_effort = \"xhigh\"\n",
    "plan_mode_reasoning_effort = \"xhigh\"\n",
    "\n",
    "[agents]\n",
    "max_threads = 6\n",
    "max_depth = 1\n",
);

#[test]
fn dry_run_creates_no_destination_or_state() {
    // Arrange
    let temporary = project_tempdir("application-dry-run");
    let source_root = temporary.path().join("source");
    let codex_home = temporary.path().join("codex-home");
    let skills_home = temporary.path().join("skills-home");
    let state_dir = temporary.path().join("state");
    fs::create_dir(&source_root).expect("create source");
    fs::write(source_root.join("config.toml"), MANAGED_CONFIG).expect("write config");
    fs::create_dir_all(source_root.join("skills/review")).expect("create source skill");
    fs::write(source_root.join("skills/review/SKILL.md"), b"review").expect("write source skill");
    let command = InstallerCommand::Install(InstallCommand {
        dry_run: true,
        adopt_existing: false,
        agent_threads: "6".to_owned(),
        codex_home: codex_home.clone(),
        skills_home: skills_home.clone(),
        state_dir: state_dir.clone(),
    });
    let context = ApplicationContext {
        source_root,
        resources: MachineResources {
            logical_cpus: 1,
            memory_bytes: 0,
        },
    };

    // Act
    let result = execute_with_context(command, context);

    // Assert
    let output = result.expect("dry-run succeeds");
    assert!(output.contains("CREATE config"));
    assert!(output.contains("CREATE skill review"));
    assert!(output.contains("CREATE manifest"));
    assert_eq!(
        (
            codex_home.exists(),
            skills_home.exists(),
            state_dir.exists(),
            codex_home.join("codex-manifest-installer.lock").exists(),
        ),
        (false, false, false, false)
    );
}
