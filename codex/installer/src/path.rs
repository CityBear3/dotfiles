use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::InstallerError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub(crate) enum RootId {
    CodexHome,
    SkillsHome,
    StateDir,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct Locator {
    pub(crate) root: RootId,
    pub(crate) relative: PathBuf,
}

impl<'de> Deserialize<'de> for Locator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireLocator {
            root: RootId,
            relative: PathBuf,
        }

        let wire = WireLocator::deserialize(deserializer)?;
        Self::new(wire.root, wire.relative).map_err(serde::de::Error::custom)
    }
}

impl Locator {
    pub(crate) fn new(root: RootId, relative: impl Into<PathBuf>) -> Result<Self, InstallerError> {
        let relative = relative.into();
        validate_relative(&relative)?;
        Ok(Self { root, relative })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct InstallRoots {
    pub(crate) source_root: PathBuf,
    pub(crate) codex_home: PathBuf,
    pub(crate) skills_home: PathBuf,
    pub(crate) state_dir: PathBuf,
}

impl InstallRoots {
    pub(crate) fn normalize(
        source_root: &Path,
        codex_home: &Path,
        skills_home: &Path,
        state_dir: &Path,
    ) -> Result<Self, InstallerError> {
        let roots = Self {
            source_root: normalize_directory(source_root, true)?,
            codex_home: normalize_directory(codex_home, false)?,
            skills_home: normalize_directory(skills_home, false)?,
            state_dir: normalize_directory(state_dir, false)?,
        };
        ensure_non_overlapping(&roots.source_root, &roots.codex_home)?;
        ensure_non_overlapping(&roots.source_root, &roots.skills_home)?;
        ensure_non_overlapping(&roots.source_root, &roots.state_dir)?;
        ensure_non_overlapping(&roots.codex_home, &roots.skills_home)?;
        ensure_non_overlapping(&roots.codex_home, &roots.state_dir)?;
        ensure_non_overlapping(&roots.skills_home, &roots.state_dir)?;
        Ok(roots)
    }

    pub(crate) fn resolve(&self, locator: &Locator) -> PathBuf {
        let root = match locator.root {
            RootId::CodexHome => &self.codex_home,
            RootId::SkillsHome => &self.skills_home,
            RootId::StateDir => &self.state_dir,
        };
        root.join(&locator.relative)
    }
}

pub(crate) fn validate_relative(relative: &Path) -> Result<(), InstallerError> {
    let text = relative
        .to_str()
        .ok_or_else(|| unsafe_path(relative, "path is not UTF-8"))?;
    if text.is_empty() {
        return Err(unsafe_path(relative, "path is empty"));
    }
    for component in text.split(std::path::MAIN_SEPARATOR) {
        if component.is_empty() {
            return Err(unsafe_path(relative, "path contains an empty component"));
        }
        if matches!(component, "." | "..") {
            return Err(unsafe_path(relative, "path contains a dot component"));
        }
    }
    if relative
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(unsafe_path(
            relative,
            "path must contain only normal relative components",
        ));
    }
    Ok(())
}

pub(crate) fn validate_destination_ancestors(
    root: &Path,
    relative: &Path,
) -> Result<(), InstallerError> {
    validate_relative(relative)?;
    let mut current = root.to_path_buf();
    let components = relative.components().collect::<Vec<_>>();
    for component in components.iter().take(components.len().saturating_sub(1)) {
        current.push(component.as_os_str());
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(unsafe_path(&current, "symlink ancestor is not allowed"));
            }
            Ok(metadata) if !metadata.is_dir() => {
                return Err(unsafe_path(
                    &current,
                    "destination ancestor is not an ordinary directory",
                ));
            }
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => break,
            Err(error) => return Err(filesystem_error("inspect destination", &current, error)),
        }
    }
    Ok(())
}

pub(crate) fn normalize_directory(
    path: &Path,
    must_exist: bool,
) -> Result<PathBuf, InstallerError> {
    if !path.is_absolute() {
        return Err(unsafe_path(path, "root must be absolute"));
    }
    path.to_str()
        .ok_or_else(|| unsafe_path(path, "root is not UTF-8"))?;

    let mut existing = PathBuf::new();
    let mut missing = Vec::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => existing.push(prefix.as_os_str()),
            Component::RootDir => existing.push(component.as_os_str()),
            Component::Normal(name) if missing.is_empty() => {
                let candidate = existing.join(name);
                match fs::symlink_metadata(&candidate) {
                    Ok(metadata) if metadata.file_type().is_symlink() => {
                        return Err(unsafe_path(&candidate, "symlink ancestor is not allowed"));
                    }
                    Ok(metadata) if !metadata.is_dir() => {
                        return Err(unsafe_path(
                            &candidate,
                            "root ancestor is not an ordinary directory",
                        ));
                    }
                    Ok(_) => existing = candidate,
                    Err(error) if error.kind() == io::ErrorKind::NotFound => {
                        missing.push(name.to_os_string());
                    }
                    Err(error) => {
                        return Err(filesystem_error("inspect root", &candidate, error));
                    }
                }
            }
            Component::Normal(name) => missing.push(name.to_os_string()),
            Component::CurDir | Component::ParentDir => {
                return Err(unsafe_path(path, "root contains a non-normal component"));
            }
        }
    }

    if must_exist && !missing.is_empty() {
        return Err(unsafe_path(path, "required directory does not exist"));
    }
    let mut normalized = existing
        .canonicalize()
        .map_err(|error| filesystem_error("canonicalize root", &existing, error))?;
    for component in missing {
        normalized.push(component);
    }
    Ok(normalized)
}

fn ensure_non_overlapping(first: &Path, second: &Path) -> Result<(), InstallerError> {
    if first.starts_with(second) || second.starts_with(first) {
        return Err(InstallerError::UnsafePath {
            path: first.to_path_buf(),
            message: format!("root overlaps {}", second.display()),
        });
    }
    Ok(())
}

fn unsafe_path(path: &Path, message: impl Into<String>) -> InstallerError {
    InstallerError::UnsafePath {
        path: path.to_path_buf(),
        message: message.into(),
    }
}

fn filesystem_error(operation: &str, path: &Path, error: io::Error) -> InstallerError {
    InstallerError::Filesystem {
        message: format!("{operation} {}: {error}", path.display()),
    }
}

#[cfg(test)]
#[path = "path_tests.rs"]
mod tests;
