use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

use crate::InstallerError;
use crate::content::{CapturedContent, ContentPayload, capture_optional};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SourceConfig {
    pub(crate) text: String,
    pub(crate) content: CapturedContent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SourceInventory {
    pub(crate) config: SourceConfig,
    pub(crate) global_agents: Option<CapturedContent>,
    pub(crate) skills: BTreeMap<String, CapturedContent>,
    pub(crate) agents: BTreeMap<String, CapturedContent>,
}

pub(crate) fn inventory(source_root: &Path) -> Result<SourceInventory, InstallerError> {
    let config_content = required_file(&source_root.join("config.toml"), "config.toml")?;
    let config_text = std::str::from_utf8(
        config_content
            .file_bytes()
            .expect("required_file guarantees file content"),
    )
    .map_err(|error| invalid_inventory(format!("config.toml is not UTF-8: {error}")))?
    .to_owned();
    let global_agents = optional_file(&source_root.join("AGENTS.global.md"), "AGENTS.global.md")?;
    let skills = inventory_skills(&source_root.join("skills"))?;
    let agents = inventory_agents(&source_root.join("agents"))?;

    Ok(SourceInventory {
        config: SourceConfig {
            text: config_text,
            content: config_content,
        },
        global_agents,
        skills,
        agents,
    })
}

pub(crate) fn validate_asset_name(name: &str) -> Result<(), InstallerError> {
    if name.is_empty()
        || matches!(name, "." | "..")
        || !name.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
        })
    {
        return Err(invalid_inventory(format!(
            "unsafe managed asset name: {name:?}"
        )));
    }
    Ok(())
}

pub(crate) fn validate_agent_name(name: &str) -> Result<(), InstallerError> {
    let stem = name
        .strip_suffix(".toml")
        .ok_or_else(|| invalid_inventory(format!("managed agent lacks .toml suffix: {name:?}")))?;
    validate_asset_name(stem)
}

fn inventory_skills(path: &Path) -> Result<BTreeMap<String, CapturedContent>, InstallerError> {
    let mut skills = BTreeMap::new();
    for entry in optional_directory_entries(path, "skills")? {
        let name = entry_name(&entry)?;
        if name == ".system" {
            continue;
        }
        validate_asset_name(&name)?;
        let content = capture_optional(&entry.path())?.ok_or_else(|| {
            invalid_inventory(format!("skill disappeared during capture: {name}"))
        })?;
        if !matches!(content.payload, ContentPayload::Directory { .. }) {
            return Err(invalid_inventory(format!(
                "managed skill must be a directory: {name}"
            )));
        }
        skills.insert(name, content);
    }
    Ok(skills)
}

fn inventory_agents(path: &Path) -> Result<BTreeMap<String, CapturedContent>, InstallerError> {
    let mut agents = BTreeMap::new();
    for entry in optional_directory_entries(path, "agents")? {
        let name = entry_name(&entry)?;
        if !name.ends_with(".toml") {
            continue;
        }
        validate_agent_name(&name)?;
        let content = capture_optional(&entry.path())?.ok_or_else(|| {
            invalid_inventory(format!("agent disappeared during capture: {name}"))
        })?;
        if !matches!(content.payload, ContentPayload::File(_)) {
            return Err(invalid_inventory(format!(
                "managed agent must be an ordinary file: {name}"
            )));
        }
        agents.insert(name, content);
    }
    Ok(agents)
}

fn required_file(path: &Path, label: &str) -> Result<CapturedContent, InstallerError> {
    optional_file(path, label)?
        .ok_or_else(|| invalid_inventory(format!("required source asset is missing: {label}")))
}

fn optional_file(path: &Path, label: &str) -> Result<Option<CapturedContent>, InstallerError> {
    let content = capture_optional(path)?;
    if content
        .as_ref()
        .is_some_and(|content| !matches!(content.payload, ContentPayload::File(_)))
    {
        return Err(invalid_inventory(format!(
            "source asset must be an ordinary file: {label}"
        )));
    }
    Ok(content)
}

fn optional_directory_entries(
    path: &Path,
    label: &str,
) -> Result<Vec<fs::DirEntry>, InstallerError> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(filesystem_error("inspect source directory", path, error)),
    };
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(invalid_inventory(format!(
            "source {label} must be an ordinary directory"
        )));
    }
    let mut entries = fs::read_dir(path)
        .map_err(|error| filesystem_error("read source directory", path, error))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| filesystem_error("read source entry", path, error))?;
    entries.sort_by_key(fs::DirEntry::file_name);
    Ok(entries)
}

fn entry_name(entry: &fs::DirEntry) -> Result<String, InstallerError> {
    entry
        .file_name()
        .into_string()
        .map_err(|_| invalid_inventory("managed source name is not UTF-8"))
}

fn invalid_inventory(message: impl Into<String>) -> InstallerError {
    InstallerError::InvalidInventory {
        message: message.into(),
    }
}

fn filesystem_error(operation: &str, path: &Path, error: io::Error) -> InstallerError {
    InstallerError::Filesystem {
        message: format!("{operation} {}: {error}", path.display()),
    }
}
