use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn process_tempdir(test_name: &str) -> tempfile::TempDir {
    let parent = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("process");
    std::fs::create_dir_all(&parent).expect("create project-local process-test root");
    tempfile::Builder::new()
        .prefix(test_name)
        .tempdir_in(parent)
        .expect("create process-test directory")
}

pub(crate) fn source_fixture(parent: &Path) -> PathBuf {
    let source = parent.join("source-fixture");
    fs::create_dir_all(source.join("skills/fixture-skill")).expect("create fixture skill");
    fs::create_dir_all(source.join("agents")).expect("create fixture agents");
    fs::write(
        source.join("config.toml"),
        concat!(
            "model = \"fixture-model\"\n",
            "model_reasoning_effort = \"medium\"\n",
            "plan_mode_reasoning_effort = \"high\"\n",
            "\n",
            "[agents]\n",
            "max_threads = 2\n",
            "max_depth = 2\n",
        ),
    )
    .expect("write fixture config");
    fs::write(source.join("AGENTS.global.md"), b"fixture guidance\n")
        .expect("write fixture guidance");
    fs::write(
        source.join("skills/fixture-skill/SKILL.md"),
        b"fixture skill\n",
    )
    .expect("write fixture skill");
    fs::write(
        source.join("agents/fixture-agent.toml"),
        b"name = \"fixture-agent\"\n",
    )
    .expect("write fixture agent");
    source
}
