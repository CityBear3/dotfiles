use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::InstallerError;
use crate::config_merge::merge_config;
use crate::content::{CapturedContent, ContentPayload, capture_optional};
use crate::ownership::{
    ManifestState, OwnershipManifest, desired_manifest, manifest_content, read_manifest,
    stale_assets,
};
use crate::path::{InstallRoots, Locator, RootId, validate_destination_ancestors};
use crate::resources::select_max_threads;
use crate::source::inventory;

use super::{AssetCategory, InstallPlan, InstallPlanRequest, PlanAction, PlanOperation};

pub(crate) fn plan_install(request: InstallPlanRequest) -> Result<InstallPlan, InstallerError> {
    let roots = InstallRoots::normalize(
        &request.source_root,
        &request.codex_home,
        &request.skills_home,
        &request.state_dir,
    )?;
    let inventory = inventory(&roots.source_root)?;
    let max_threads = select_max_threads(request.resources, &request.requested_threads)?;
    debug_assert_eq!(
        inventory.config.content.file_bytes(),
        Some(inventory.config.text.as_bytes())
    );

    let manifest_locator = Locator::new(RootId::StateDir, "manifest-v1.json")?;
    validate_destination_ancestors(&roots.state_dir, &manifest_locator.relative)?;
    let prior_state = read_manifest(&roots.resolve(&manifest_locator))?;
    let (prior, prior_content) = match prior_state {
        ManifestState::Absent => (OwnershipManifest::new(false, [], []), None),
        ManifestState::Present { manifest, content } => (manifest, Some(content)),
    };
    let desired_manifest = desired_manifest(&inventory);
    let stale = stale_assets(&prior, &desired_manifest);

    let mut actions = Vec::new();
    let mut conflicts = Vec::new();

    plan_config(&roots, &inventory.config.text, max_threads, &mut actions)?;

    if inventory.global_agents.is_some() || stale.global_agents {
        plan_optional(
            &roots,
            Locator::new(RootId::CodexHome, "AGENTS.md")?,
            AssetCategory::GlobalAgents,
            None,
            inventory.global_agents.clone(),
            prior.global_agents,
            request.adopt_existing,
            &mut actions,
            &mut conflicts,
        )?;
    }

    let skill_names = inventory
        .skills
        .keys()
        .cloned()
        .chain(stale.skills)
        .collect::<BTreeSet<_>>();
    for name in skill_names {
        plan_optional(
            &roots,
            Locator::new(RootId::SkillsHome, &name)?,
            AssetCategory::Skill,
            Some(name.clone()),
            inventory.skills.get(&name).cloned(),
            prior.skills.contains(&name),
            request.adopt_existing,
            &mut actions,
            &mut conflicts,
        )?;
    }

    let agent_names = inventory
        .agents
        .keys()
        .cloned()
        .chain(stale.agents)
        .collect::<BTreeSet<_>>();
    for name in agent_names {
        plan_optional(
            &roots,
            Locator::new(RootId::CodexHome, PathBuf::from("agents").join(&name))?,
            AssetCategory::Agent,
            Some(name.clone()),
            inventory.agents.get(&name).cloned(),
            prior.agents.contains(&name),
            request.adopt_existing,
            &mut actions,
            &mut conflicts,
        )?;
    }

    if !conflicts.is_empty() {
        return Err(InstallerError::UnmanagedConflict { paths: conflicts });
    }

    let desired_manifest_content = manifest_content(&desired_manifest)?;
    actions.push(PlanAction {
        operation: classify_managed(prior_content.as_ref(), &desired_manifest_content),
        category: AssetCategory::Manifest,
        name: None,
        locator: manifest_locator,
        desired: Some(desired_manifest_content),
    });

    Ok(InstallPlan {
        roots,
        max_threads,
        actions,
    })
}

fn plan_config(
    roots: &InstallRoots,
    managed_text: &str,
    max_threads: u8,
    actions: &mut Vec<PlanAction>,
) -> Result<(), InstallerError> {
    let locator = Locator::new(RootId::CodexHome, "config.toml")?;
    let existing = capture_destination(roots, &locator)?;
    let existing_text = match &existing {
        None => "",
        Some(content) => file_text(content, "destination config.toml")?,
    };
    let desired =
        CapturedContent::file(merge_config(existing_text, managed_text, max_threads)?.into_bytes());
    actions.push(PlanAction {
        operation: classify_managed(existing.as_ref(), &desired),
        category: AssetCategory::Config,
        name: None,
        locator,
        desired: Some(desired),
    });
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn plan_optional(
    roots: &InstallRoots,
    locator: Locator,
    category: AssetCategory,
    name: Option<String>,
    desired: Option<CapturedContent>,
    owned: bool,
    adopt_existing: bool,
    actions: &mut Vec<PlanAction>,
    conflicts: &mut Vec<PathBuf>,
) -> Result<(), InstallerError> {
    let existing = capture_destination(roots, &locator)?;
    let operation = match (&desired, &existing) {
        (Some(desired), Some(existing)) if !owned && !adopt_existing => {
            conflicts.push(roots.resolve(&locator));
            return Ok(());
        }
        (Some(desired), Some(existing)) if desired == existing => PlanOperation::NoOp,
        (Some(_), Some(_)) => PlanOperation::Replace,
        (Some(_), None) => PlanOperation::Create,
        (None, Some(_)) if owned => PlanOperation::Remove,
        (None, None) | (None, Some(_)) => return Ok(()),
    };
    actions.push(PlanAction {
        operation,
        category,
        name,
        locator,
        desired,
    });
    Ok(())
}

fn capture_destination(
    roots: &InstallRoots,
    locator: &Locator,
) -> Result<Option<CapturedContent>, InstallerError> {
    let root = root_for(roots, locator.root);
    validate_destination_ancestors(root, &locator.relative)?;
    capture_optional(&roots.resolve(locator))
}

fn root_for(roots: &InstallRoots, root: RootId) -> &Path {
    match root {
        RootId::CodexHome => &roots.codex_home,
        RootId::SkillsHome => &roots.skills_home,
        RootId::StateDir => &roots.state_dir,
    }
}

fn classify_managed(
    existing: Option<&CapturedContent>,
    desired: &CapturedContent,
) -> PlanOperation {
    match existing {
        None => PlanOperation::Create,
        Some(existing) if existing == desired => PlanOperation::NoOp,
        Some(_) => PlanOperation::Replace,
    }
}

fn file_text<'a>(content: &'a CapturedContent, label: &str) -> Result<&'a str, InstallerError> {
    let ContentPayload::File(bytes) = &content.payload else {
        return Err(InstallerError::InvalidConfiguration {
            message: format!("{label} must be an ordinary file"),
        });
    };
    std::str::from_utf8(bytes).map_err(|error| InstallerError::InvalidConfiguration {
        message: format!("{label} is not UTF-8: {error}"),
    })
}
