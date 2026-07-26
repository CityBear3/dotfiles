use std::collections::BTreeMap;
use std::ffi::CString;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::symlink;

use crate::content::CapturedContent;
use crate::test_support::project_tempdir;

use super::inventory::SourceConfig;
use super::{SourceInventory, inventory};

#[test]
fn inventory_maps_only_declared_assets_in_deterministic_order() {
    // Arrange
    let temporary = project_tempdir("source-inventory");
    let source = temporary.path().join("codex");
    fs::create_dir_all(source.join("skills/zeta/nested")).expect("create zeta skill");
    fs::create_dir_all(source.join("skills/alpha")).expect("create alpha skill");
    fs::create_dir_all(source.join("agents")).expect("create agents");
    fs::write(source.join("config.toml"), b"model = \"repo\"\n").expect("write config");
    fs::write(source.join("AGENTS.global.md"), b"global").expect("write guidance");
    fs::write(source.join("README.md"), b"not managed").expect("write unrelated root file");
    fs::write(source.join("skills/zeta/SKILL.md"), b"zeta").expect("write zeta skill");
    fs::write(source.join("skills/zeta/nested/data.txt"), b"nested").expect("write nested");
    fs::write(source.join("skills/alpha/SKILL.md"), b"alpha").expect("write alpha skill");
    fs::write(source.join("agents/zeta.toml"), b"name = \"zeta\"\n").expect("write zeta agent");
    fs::write(source.join("agents/alpha.toml"), b"name = \"alpha\"\n").expect("write alpha agent");
    fs::write(source.join("agents/README.md"), b"not managed").expect("write unrelated agent");

    // Act
    let result = inventory(&source);

    // Assert
    assert_eq!(
        result,
        Ok(SourceInventory {
            config: SourceConfig {
                text: "model = \"repo\"\n".to_owned(),
                content: CapturedContent::file(b"model = \"repo\"\n".to_vec()),
            },
            global_agents: Some(CapturedContent::file(b"global".to_vec())),
            skills: BTreeMap::from([
                (
                    "alpha".to_owned(),
                    CapturedContent::directory(
                        Default::default(),
                        BTreeMap::from([("SKILL.md".into(), b"alpha".to_vec(),)]),
                    ),
                ),
                (
                    "zeta".to_owned(),
                    CapturedContent::directory(
                        ["nested".into()].into_iter().collect(),
                        BTreeMap::from([
                            ("SKILL.md".into(), b"zeta".to_vec()),
                            ("nested/data.txt".into(), b"nested".to_vec()),
                        ]),
                    ),
                ),
            ]),
            agents: BTreeMap::from([
                (
                    "alpha.toml".to_owned(),
                    CapturedContent::file(b"name = \"alpha\"\n".to_vec()),
                ),
                (
                    "zeta.toml".to_owned(),
                    CapturedContent::file(b"name = \"zeta\"\n".to_vec()),
                ),
            ]),
        })
    );
}

#[test]
fn inventory_excludes_system_skills_without_inspecting_them() {
    // Arrange
    let temporary = project_tempdir("source-system-excluded");
    let source = source_with_config(temporary.path());
    fs::create_dir_all(source.join("skills/.system")).expect("create system skills");
    let target = source.join("outside");
    fs::write(&target, b"outside").expect("write symlink target");
    symlink(&target, source.join("skills/.system/unsafe")).expect("create excluded symlink");

    // Act
    let result = inventory(&source);

    // Assert
    assert_eq!(result.expect("inventory").skills, BTreeMap::new());
}

#[test]
fn inventory_rejects_unsafe_managed_names() {
    // Arrange
    let temporary = project_tempdir("source-unsafe-name");
    let source = source_with_config(temporary.path());
    fs::create_dir_all(source.join("skills/Bad Skill")).expect("create unsafe skill");

    // Act
    let result = inventory(&source);

    // Assert
    assert!(result.is_err());
}

#[test]
fn inventory_rejects_an_agent_without_a_safe_name_before_toml_suffix() {
    // Arrange
    let temporary = project_tempdir("source-unsafe-agent-name");
    let source = source_with_config(temporary.path());
    fs::create_dir_all(source.join("agents")).expect("create agents");
    fs::write(source.join("agents/.toml"), b"name = \"missing\"\n").expect("write unsafe agent");

    // Act
    let result = inventory(&source);

    // Assert
    assert!(result.is_err());
}

#[test]
fn inventory_rejects_managed_symlinks() {
    // Arrange
    let temporary = project_tempdir("source-managed-symlink");
    let source = source_with_config(temporary.path());
    fs::create_dir_all(source.join("skills")).expect("create skills");
    let target = source.join("target");
    fs::create_dir(&target).expect("create symlink target");
    symlink(&target, source.join("skills/linked")).expect("create managed symlink");

    // Act
    let result = inventory(&source);

    // Assert
    assert!(result.is_err());
}

#[test]
fn inventory_rejects_special_managed_files() {
    // Arrange
    let temporary = project_tempdir("source-special-file");
    let source = source_with_config(temporary.path());
    fs::create_dir_all(source.join("agents")).expect("create agents");
    let fifo = source.join("agents/special.toml");
    let fifo_name = CString::new(fifo.as_os_str().as_bytes()).expect("fifo path");
    let result = unsafe { libc::mkfifo(fifo_name.as_ptr(), 0o600) };
    assert_eq!(result, 0, "create fifo");

    // Act
    let result = inventory(&source);

    // Assert
    assert!(result.is_err());
}

fn source_with_config(parent: &std::path::Path) -> std::path::PathBuf {
    let source = parent.join("codex");
    fs::create_dir(&source).expect("create source");
    fs::write(source.join("config.toml"), b"model = \"repo\"\n").expect("write config");
    source
}
