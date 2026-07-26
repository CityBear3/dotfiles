use std::path::{Path, PathBuf};

use crate::InstallerError;
use crate::backup::Backup;
use crate::content::capture_optional;
use crate::ownership::{ManifestState, OwnershipManifest, read_manifest};
use crate::path::{InstallRoots, Locator, RootId, validate_destination_ancestors};

use super::{AssetCategory, InstallPlan, PlanAction, PlanOperation};

pub(crate) fn build_restore_plan(
    source_root: &Path,
    backup: &Backup,
) -> Result<InstallPlan, InstallerError> {
    let roots = InstallRoots::normalize(
        source_root,
        &backup.journal.roots.codex_home,
        &backup.journal.roots.skills_home,
        &backup.journal.roots.state_dir,
    )?;
    let current_ownership = current_ownership(&roots)?;
    let mut locators = backup
        .journal
        .entries
        .iter()
        .map(|entry| entry.locator.clone())
        .collect::<Vec<_>>();
    if let Some(backup_ownership) = &backup.journal.ownership {
        extend_owned_locators(&mut locators, backup_ownership)?;
    }
    extend_owned_locators(&mut locators, &current_ownership)?;
    let mut actions = Vec::with_capacity(locators.len());
    let mut conflicts = Vec::new();
    for locator in &locators {
        match plan_backup_entry(&roots, backup, &current_ownership, locator)? {
            PlannedEntry::Action(action) => actions.push(action),
            PlannedEntry::Conflict(path) => conflicts.push(path),
        }
    }
    if !conflicts.is_empty() {
        return Err(InstallerError::UnmanagedConflict { paths: conflicts });
    }
    actions.sort_by_key(|action| category_order(action.category));

    Ok(InstallPlan {
        roots,
        max_threads: 0,
        actions,
    })
}

fn plan_backup_entry(
    roots: &InstallRoots,
    backup: &Backup,
    current_ownership: &OwnershipManifest,
    locator: &Locator,
) -> Result<PlannedEntry, InstallerError> {
    let (category, name) = describe(locator)?;
    let root = match locator.root {
        RootId::CodexHome => &roots.codex_home,
        RootId::SkillsHome => &roots.skills_home,
        RootId::StateDir => &roots.state_dir,
    };
    validate_destination_ancestors(root, &locator.relative)?;
    let existing = capture_optional(&roots.resolve(locator))?;
    let desired = if backup
        .journal
        .entries
        .iter()
        .any(|entry| entry.locator == *locator)
    {
        backup.content(locator)?.cloned()
    } else {
        None
    };
    if existing.is_some() && !is_owned(current_ownership, category, name.as_deref()) {
        return Ok(PlannedEntry::Conflict(roots.resolve(locator)));
    }
    let operation = match (&existing, &desired) {
        (None, Some(_)) => PlanOperation::Create,
        (Some(existing), Some(desired)) if existing == desired => PlanOperation::NoOp,
        (Some(_), Some(_)) => PlanOperation::Replace,
        (Some(_), None) => PlanOperation::Remove,
        (None, None) => PlanOperation::NoOp,
    };
    Ok(PlannedEntry::Action(PlanAction {
        operation,
        category,
        name,
        locator: locator.clone(),
        desired,
    }))
}

enum PlannedEntry {
    Action(PlanAction),
    Conflict(PathBuf),
}

fn is_owned(ownership: &OwnershipManifest, category: AssetCategory, name: Option<&str>) -> bool {
    match category {
        AssetCategory::Config | AssetCategory::Manifest => true,
        AssetCategory::GlobalAgents => ownership.global_agents,
        AssetCategory::Skill => name.is_some_and(|name| ownership.skills.contains(name)),
        AssetCategory::Agent => name.is_some_and(|name| ownership.agents.contains(name)),
    }
}

fn current_ownership(roots: &InstallRoots) -> Result<OwnershipManifest, InstallerError> {
    let manifest = Locator::new(RootId::StateDir, "manifest-v1.json")?;
    validate_destination_ancestors(&roots.state_dir, &manifest.relative)?;
    match read_manifest(&roots.resolve(&manifest))? {
        ManifestState::Absent => Ok(OwnershipManifest::new(false, [], [])),
        ManifestState::Present { manifest, .. } => Ok(manifest),
    }
}

fn extend_owned_locators(
    locators: &mut Vec<Locator>,
    ownership: &OwnershipManifest,
) -> Result<(), InstallerError> {
    if ownership.global_agents {
        push_unique(locators, Locator::new(RootId::CodexHome, "AGENTS.md")?);
    }
    for skill in &ownership.skills {
        push_unique(
            locators,
            Locator::new(RootId::SkillsHome, PathBuf::from(skill))?,
        );
    }
    for agent in &ownership.agents {
        push_unique(
            locators,
            Locator::new(RootId::CodexHome, PathBuf::from("agents").join(agent))?,
        );
    }
    Ok(())
}

fn push_unique(locators: &mut Vec<Locator>, locator: Locator) {
    if !locators.contains(&locator) {
        locators.push(locator);
    }
}

fn describe(locator: &Locator) -> Result<(AssetCategory, Option<String>), InstallerError> {
    let path = &locator.relative;
    match locator.root {
        RootId::CodexHome if path == Path::new("config.toml") => Ok((AssetCategory::Config, None)),
        RootId::CodexHome if path == Path::new("AGENTS.md") => {
            Ok((AssetCategory::GlobalAgents, None))
        }
        RootId::CodexHome if path.components().count() == 2 => Ok((
            AssetCategory::Agent,
            path.file_name()
                .and_then(|name| name.to_str())
                .map(str::to_owned),
        )),
        RootId::SkillsHome => Ok((AssetCategory::Skill, path.to_str().map(str::to_owned))),
        RootId::StateDir if path == Path::new("manifest-v1.json") => {
            Ok((AssetCategory::Manifest, None))
        }
        _ => Err(InstallerError::InvalidBackup {
            message: format!("backup locator is not restorable: {:?}", path),
        }),
    }
}

fn category_order(category: AssetCategory) -> u8 {
    match category {
        AssetCategory::Config => 0,
        AssetCategory::GlobalAgents => 1,
        AssetCategory::Skill => 2,
        AssetCategory::Agent => 3,
        AssetCategory::Manifest => 4,
    }
}
