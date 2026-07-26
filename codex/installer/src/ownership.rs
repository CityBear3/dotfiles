use std::collections::BTreeSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::InstallerError;
use crate::content::{CapturedContent, ContentPayload, capture_optional};
use crate::source::{SourceInventory, validate_agent_name, validate_asset_name};

const MANIFEST_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OwnershipManifest {
    pub(crate) version: u32,
    pub(crate) global_agents: bool,
    pub(crate) skills: BTreeSet<String>,
    pub(crate) agents: BTreeSet<String>,
}

impl OwnershipManifest {
    pub(crate) fn new<'a>(
        global_agents: bool,
        skills: impl IntoIterator<Item = &'a str>,
        agents: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        Self {
            version: MANIFEST_VERSION,
            global_agents,
            skills: skills.into_iter().map(str::to_owned).collect(),
            agents: agents.into_iter().map(str::to_owned).collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ManifestState {
    Absent,
    Present {
        manifest: OwnershipManifest,
        content: CapturedContent,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaleAssets {
    pub(crate) global_agents: bool,
    pub(crate) skills: BTreeSet<String>,
    pub(crate) agents: BTreeSet<String>,
}

pub(crate) fn read_manifest(path: &Path) -> Result<ManifestState, InstallerError> {
    let Some(content) = capture_optional(path)? else {
        return Ok(ManifestState::Absent);
    };
    let ContentPayload::File(bytes) = &content.payload else {
        return Err(invalid_manifest(
            "ownership manifest must be an ordinary file",
        ));
    };
    let manifest: OwnershipManifest = serde_json::from_slice(bytes)
        .map_err(|error| invalid_manifest(format!("invalid ownership manifest: {error}")))?;
    validate_manifest(&manifest)?;
    Ok(ManifestState::Present { manifest, content })
}

pub(crate) fn desired_manifest(inventory: &SourceInventory) -> OwnershipManifest {
    OwnershipManifest {
        version: MANIFEST_VERSION,
        global_agents: inventory.global_agents.is_some(),
        skills: inventory.skills.keys().cloned().collect(),
        agents: inventory.agents.keys().cloned().collect(),
    }
}

pub(crate) fn manifest_content(
    manifest: &OwnershipManifest,
) -> Result<CapturedContent, InstallerError> {
    validate_manifest(manifest)?;
    let mut bytes = serde_json::to_vec_pretty(manifest)
        .map_err(|error| invalid_manifest(format!("serialize ownership manifest: {error}")))?;
    bytes.push(b'\n');
    Ok(CapturedContent::file(bytes))
}

pub(crate) fn stale_assets(prior: &OwnershipManifest, desired: &OwnershipManifest) -> StaleAssets {
    StaleAssets {
        global_agents: prior.global_agents && !desired.global_agents,
        skills: prior.skills.difference(&desired.skills).cloned().collect(),
        agents: prior.agents.difference(&desired.agents).cloned().collect(),
    }
}

pub(crate) fn validate_manifest(manifest: &OwnershipManifest) -> Result<(), InstallerError> {
    if manifest.version != MANIFEST_VERSION {
        return Err(invalid_manifest(format!(
            "unsupported ownership manifest version: {}",
            manifest.version
        )));
    }
    for skill in &manifest.skills {
        validate_asset_name(skill)
            .map_err(|_| invalid_manifest(format!("unsafe owned skill name: {skill:?}")))?;
        if skill == ".system" {
            return Err(invalid_manifest(".system cannot be installer-owned"));
        }
    }
    for agent in &manifest.agents {
        validate_agent_name(agent)
            .map_err(|_| invalid_manifest(format!("unsafe owned agent name: {agent:?}")))?;
    }
    Ok(())
}

fn invalid_manifest(message: impl Into<String>) -> InstallerError {
    InstallerError::InvalidManifest {
        message: message.into(),
    }
}

#[cfg(test)]
#[path = "ownership_tests.rs"]
mod tests;
