use std::fs;
use std::path::{Path, PathBuf};

use crate::InstallerError;
use crate::backup::{BackupRequest, BackupRoots, BackupStore};
use crate::content::CapturedContent;
use crate::ownership::{OwnershipManifest, manifest_content};
use crate::path::{Locator, RootId};
use crate::platform::macos::MacOsPlatform;
use crate::resources::MachineResources;
use crate::test_support::project_tempdir;

use super::{
    AssetCategory, InstallPlan, InstallPlanRequest, PlanOperation, build_restore_plan, plan_install,
};

const MANAGED_CONFIG: &str = concat!(
    "model = \"gpt-5.6\"\n",
    "model_reasoning_effort = \"xhigh\"\n",
    "plan_mode_reasoning_effort = \"xhigh\"\n",
    "\n",
    "[agents]\n",
    "max_threads = 6\n",
    "max_depth = 2\n",
    "\n",
    "[tools.update_plan]\n",
    "enabled = true\n",
);

#[test]
fn plan_classifies_absent_destinations_as_create() {
    // Arrange
    let temporary = project_tempdir("plan-create");
    let fixture = Fixture::new(temporary.path());

    // Act
    let result = plan_install(fixture.request(false));

    // Assert
    let plan = result.expect("create plan");
    assert_eq!(
        plan.actions
            .iter()
            .map(|action| (action.operation, action.category))
            .collect::<Vec<_>>(),
        vec![
            (PlanOperation::Create, AssetCategory::Config),
            (PlanOperation::Create, AssetCategory::Manifest),
        ]
    );
}

#[test]
fn plan_classifies_owned_changed_destination_as_replace() {
    // Arrange
    let temporary = project_tempdir("plan-replace");
    let fixture = Fixture::new(temporary.path());
    fixture.add_skill("review", b"desired");
    fs::create_dir_all(fixture.skills_home.join("review")).expect("create installed skill");
    fs::write(fixture.skills_home.join("review/SKILL.md"), b"existing")
        .expect("write installed skill");
    fixture.write_manifest(OwnershipManifest::new(false, ["review"], []));

    // Act
    let result = plan_install(fixture.request(false));

    // Assert
    assert_eq!(
        action_for(
            result.expect("replace plan"),
            AssetCategory::Skill,
            Some("review")
        )
        .operation,
        PlanOperation::Replace
    );
}

#[test]
fn plan_replaces_the_owned_task_orchestrator_profile_on_update() {
    // Arrange
    let temporary = project_tempdir("plan-update-task-orchestrator");
    let fixture = Fixture::new(temporary.path());
    fixture.add_agent("task-orchestrator.toml", b"name = \"task-orchestrator\"\n");
    fs::create_dir_all(fixture.codex_home.join("agents")).expect("create installed agents");
    fs::write(
        fixture.codex_home.join("agents/task-orchestrator.toml"),
        b"name = \"old-task-orchestrator\"\n",
    )
    .expect("write installed Task orchestrator");
    fixture.write_manifest(OwnershipManifest::new(
        false,
        [],
        ["task-orchestrator.toml"],
    ));

    // Act
    let result = plan_install(fixture.request(false));

    // Assert
    let action = action_for(
        result.expect("Task orchestrator update plan"),
        AssetCategory::Agent,
        Some("task-orchestrator.toml"),
    );
    assert_eq!(
        (action.operation, action.desired),
        (
            PlanOperation::Replace,
            Some(CapturedContent::file(
                b"name = \"task-orchestrator\"\n".to_vec(),
            )),
        )
    );
}

#[test]
fn plan_removes_only_stale_manifest_owned_assets() {
    // Arrange
    let temporary = project_tempdir("plan-remove");
    let fixture = Fixture::new(temporary.path());
    fs::create_dir_all(fixture.skills_home.join("owned-stale")).expect("create owned skill");
    fs::write(fixture.skills_home.join("owned-stale/SKILL.md"), b"stale")
        .expect("write owned skill");
    fs::create_dir_all(fixture.skills_home.join("external")).expect("create external skill");
    fs::write(fixture.skills_home.join("external/SKILL.md"), b"external")
        .expect("write external skill");
    fixture.write_manifest(OwnershipManifest::new(false, ["owned-stale"], []));

    // Act
    let result = plan_install(fixture.request(false));

    // Assert
    let plan = result.expect("remove plan");
    assert_eq!(
        action_for(plan.clone(), AssetCategory::Skill, Some("owned-stale")).operation,
        PlanOperation::Remove
    );
    assert!(
        plan.actions
            .iter()
            .all(|action| action.name.as_deref() != Some("external"))
    );
}

#[test]
fn plan_classifies_identical_owned_destinations_as_no_op() {
    // Arrange
    let temporary = project_tempdir("plan-no-op");
    let fixture = Fixture::new(temporary.path());
    fs::create_dir_all(&fixture.codex_home).expect("create Codex home");
    fs::write(fixture.codex_home.join("config.toml"), MANAGED_CONFIG).expect("write config");
    fixture.write_manifest(OwnershipManifest::new(false, [], []));

    // Act
    let result = plan_install(fixture.request(false));

    // Assert
    let plan = result.expect("no-op plan");
    assert_eq!(
        plan.actions
            .iter()
            .map(|action| (action.operation, action.category))
            .collect::<Vec<_>>(),
        vec![
            (PlanOperation::NoOp, AssetCategory::Config),
            (PlanOperation::NoOp, AssetCategory::Manifest),
        ]
    );
}

#[test]
fn plan_rejects_unmanaged_existing_assets_even_when_content_matches() {
    // Arrange
    let temporary = project_tempdir("plan-conflict");
    let fixture = Fixture::new(temporary.path());
    fixture.add_skill("review", b"same");
    fs::create_dir_all(fixture.skills_home.join("review")).expect("create installed skill");
    fs::write(fixture.skills_home.join("review/SKILL.md"), b"same").expect("write installed skill");

    // Act
    let result = plan_install(fixture.request(false));

    // Assert
    assert!(matches!(
        result,
        Err(InstallerError::UnmanagedConflict { .. })
    ));
}

#[test]
fn plan_adopts_an_existing_asset_only_when_requested() {
    // Arrange
    let temporary = project_tempdir("plan-adopt");
    let fixture = Fixture::new(temporary.path());
    fixture.add_skill("review", b"desired");
    fs::create_dir_all(fixture.skills_home.join("review")).expect("create installed skill");
    fs::write(fixture.skills_home.join("review/SKILL.md"), b"existing")
        .expect("write installed skill");

    // Act
    let result = plan_install(fixture.request(true));

    // Assert
    assert_eq!(
        action_for(
            result.expect("adoption plan"),
            AssetCategory::Skill,
            Some("review")
        )
        .operation,
        PlanOperation::Replace
    );
}

#[test]
fn plan_merges_only_the_six_managed_configuration_keys() {
    // Arrange
    let temporary = project_tempdir("plan-config-merge");
    let fixture = Fixture::new(temporary.path());
    fs::create_dir_all(&fixture.codex_home).expect("create Codex home");
    let existing = concat!(
        "model = \"old\"\n",
        "model_reasoning_effort = \"medium\"\n",
        "plan_mode_reasoning_effort = \"high\"\n",
        "approval_policy = \"on-request\"\n",
        "\n",
        "[tools.update_plan]\n",
        "enabled = false # local update-plan context\n",
        "\n",
        "[agents]\n",
        "max_threads = 2\n",
        "max_depth = 3\n",
        "custom = true\n",
    );
    fs::write(fixture.codex_home.join("config.toml"), existing).expect("write config");
    let expected = concat!(
        "model = \"gpt-5.6\"\n",
        "model_reasoning_effort = \"xhigh\"\n",
        "plan_mode_reasoning_effort = \"xhigh\"\n",
        "approval_policy = \"on-request\"\n",
        "\n",
        "[tools.update_plan]\n",
        "enabled = true # local update-plan context\n",
        "\n",
        "[agents]\n",
        "max_threads = 6\n",
        "max_depth = 2\n",
        "custom = true\n",
    );

    // Act
    let result = plan_install(fixture.request(false));

    // Assert
    assert_eq!(
        action_for(result.expect("merged plan"), AssetCategory::Config, None).desired,
        Some(CapturedContent::file(expected.as_bytes().to_vec()))
    );
}

#[test]
fn manifest_action_is_always_last_in_deterministic_plan_order() {
    // Arrange
    let temporary = project_tempdir("plan-manifest-last");
    let fixture = Fixture::new(temporary.path());
    fixture.add_skill("zeta", b"zeta");
    fixture.add_skill("alpha", b"alpha");

    // Act
    let plan = plan_install(fixture.request(false)).expect("ordered plan");
    // Assert
    assert_eq!(
        plan.actions.last().map(|action| action.category),
        Some(AssetCategory::Manifest)
    );
    assert_eq!(
        plan.actions
            .iter()
            .map(|action| (action.category, action.name.as_deref()))
            .collect::<Vec<_>>(),
        vec![
            (AssetCategory::Config, None),
            (AssetCategory::Skill, Some("alpha")),
            (AssetCategory::Skill, Some("zeta")),
            (AssetCategory::Manifest, None),
        ]
    );
}

#[test]
fn plan_rejects_overlapping_normalized_roots_before_inventory() {
    // Arrange
    let temporary = project_tempdir("plan-root-validation");
    let fixture = Fixture::new(temporary.path());
    let mut request = fixture.request(false);
    request.skills_home = request.codex_home.join("skills");

    // Act
    let result = plan_install(request);

    // Assert
    assert!(matches!(result, Err(InstallerError::UnsafePath { .. })));
}

#[test]
fn restore_plan_replaces_changed_backup_asset_and_keeps_manifest_last() {
    // Arrange
    let temporary = project_tempdir("restore-plan-replace");
    let fixture = Fixture::new(temporary.path());
    for directory in [
        &fixture.codex_home,
        &fixture.skills_home,
        &fixture.state_dir,
    ] {
        fs::create_dir_all(directory).expect("create restore root");
    }
    let skill = fixture.skills_home.join("review");
    fs::create_dir(&skill).expect("create generation A skill");
    fs::write(skill.join("SKILL.md"), b"generation-a").expect("write generation A skill");
    let ownership = OwnershipManifest::new(false, ["review"], []);
    fixture.write_manifest(ownership.clone());
    let roots = BackupRoots {
        codex_home: fixture.codex_home.clone(),
        skills_home: fixture.skills_home.clone(),
        state_dir: fixture.state_dir.clone(),
    };
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &fixture.state_dir);
    store
        .publish_current(BackupRequest {
            backup_id: "backup-a".to_owned(),
            roots,
            ownership: Some(ownership),
            locators: vec![
                Locator::new(RootId::SkillsHome, "review").expect("skill locator"),
                Locator::new(RootId::StateDir, "manifest-v1.json").expect("manifest locator"),
            ],
        })
        .expect("publish generation A");
    store
        .select_latest("backup-a")
        .expect("select generation A");
    fs::write(skill.join("SKILL.md"), b"generation-b").expect("write generation B skill");
    let backup = store
        .load_latest()
        .expect("load selected backup")
        .expect("selected backup");

    // Act
    let result = build_restore_plan(&fixture.source_root, &backup);

    // Assert
    let plan = result.expect("restore plan");
    assert_eq!(
        plan.actions
            .iter()
            .map(|action| (action.operation, action.category))
            .collect::<Vec<_>>(),
        vec![
            (PlanOperation::Replace, AssetCategory::Skill),
            (PlanOperation::NoOp, AssetCategory::Manifest),
        ]
    );
    assert_eq!(
        plan.actions[0].desired,
        Some(CapturedContent::directory(
            std::collections::BTreeSet::new(),
            [(PathBuf::from("SKILL.md"), b"generation-a".to_vec(),)]
                .into_iter()
                .collect(),
        ))
    );
    assert_eq!(
        plan.actions.last().map(|action| action.category),
        Some(AssetCategory::Manifest)
    );
}

#[test]
fn restore_plan_creates_backup_asset_missing_from_live_state() {
    // Arrange
    let temporary = project_tempdir("restore-plan-create");
    let fixture = Fixture::new(temporary.path());
    for directory in [
        &fixture.codex_home,
        &fixture.skills_home,
        &fixture.state_dir,
    ] {
        fs::create_dir_all(directory).expect("create restore root");
    }
    let skill = fixture.skills_home.join("review");
    fs::create_dir(&skill).expect("create generation A skill");
    fs::write(skill.join("SKILL.md"), b"generation-a").expect("write generation A skill");
    let ownership = OwnershipManifest::new(false, ["review"], []);
    fixture.write_manifest(ownership.clone());
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &fixture.state_dir);
    store
        .publish_current(BackupRequest {
            backup_id: "backup-a".to_owned(),
            roots: BackupRoots {
                codex_home: fixture.codex_home.clone(),
                skills_home: fixture.skills_home.clone(),
                state_dir: fixture.state_dir.clone(),
            },
            ownership: Some(ownership),
            locators: vec![
                Locator::new(RootId::SkillsHome, "review").expect("skill locator"),
                Locator::new(RootId::StateDir, "manifest-v1.json").expect("manifest locator"),
            ],
        })
        .expect("publish generation A");
    store
        .select_latest("backup-a")
        .expect("select generation A");
    fs::remove_dir_all(&skill).expect("remove skill from live state");
    fixture.write_manifest(OwnershipManifest::new(false, [], []));
    let backup = store
        .load_latest()
        .expect("load selected backup")
        .expect("selected backup");

    // Act
    let result = build_restore_plan(&fixture.source_root, &backup);

    // Assert
    let plan = result.expect("restore plan");
    assert_eq!(
        action_for(plan, AssetCategory::Skill, Some("review")).operation,
        PlanOperation::Create
    );
}

#[test]
fn restore_plan_removes_only_live_owned_assets_absent_from_backup_without_promotion() {
    // Arrange
    let temporary = project_tempdir("restore-plan-remove");
    let fixture = Fixture::new(temporary.path());
    for directory in [
        &fixture.codex_home,
        &fixture.skills_home,
        &fixture.state_dir,
    ] {
        fs::create_dir_all(directory).expect("create restore root");
    }
    let generation_a = OwnershipManifest::new(false, [], []);
    fixture.write_manifest(generation_a.clone());
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &fixture.state_dir);
    store
        .publish_current(BackupRequest {
            backup_id: "backup-a".to_owned(),
            roots: BackupRoots {
                codex_home: fixture.codex_home.clone(),
                skills_home: fixture.skills_home.clone(),
                state_dir: fixture.state_dir.clone(),
            },
            ownership: Some(generation_a),
            locators: vec![
                Locator::new(RootId::StateDir, "manifest-v1.json").expect("manifest locator"),
            ],
        })
        .expect("publish generation A");
    store
        .select_latest("backup-a")
        .expect("select generation A");
    let live_only = fixture.skills_home.join("live-only");
    fs::create_dir(&live_only).expect("create live-only skill");
    fs::write(live_only.join("SKILL.md"), b"generation-b").expect("write live-only skill");
    let external = fixture.skills_home.join("external");
    fs::create_dir(&external).expect("create manifest-external skill");
    fs::write(external.join("SKILL.md"), b"external").expect("write manifest-external skill");
    let system = fixture.skills_home.join(".system");
    fs::create_dir(&system).expect("create system skill");
    fs::write(system.join("SKILL.md"), b"system").expect("write system skill");
    fixture.write_manifest(OwnershipManifest::new(false, ["live-only"], []));
    let backup = store
        .load_latest()
        .expect("load selected backup")
        .expect("selected backup");

    // Act
    let result = build_restore_plan(&fixture.source_root, &backup);

    // Assert
    let plan = result.expect("restore plan");
    let removal = action_for(plan.clone(), AssetCategory::Skill, Some("live-only"));
    assert_eq!(
        (removal.operation, removal.desired),
        (PlanOperation::Remove, None)
    );
    assert_eq!(
        plan.actions.last().map(|action| action.category),
        Some(AssetCategory::Manifest)
    );
    assert!(
        plan.actions
            .iter()
            .all(|action| !matches!(action.name.as_deref(), Some("external" | ".system")))
    );
    assert_eq!(
        fs::read(fixture.state_dir.join("backups/latest")).expect("read latest marker"),
        b"backup-a\n"
    );
    assert!(!fixture.state_dir.join("backups/backup-b").exists());
}

#[test]
fn restore_plan_rejects_replacing_a_live_asset_not_owned_by_current_manifest() {
    // Arrange
    let temporary = project_tempdir("restore-plan-conflict");
    let fixture = Fixture::new(temporary.path());
    for directory in [
        &fixture.codex_home,
        &fixture.skills_home,
        &fixture.state_dir,
    ] {
        fs::create_dir_all(directory).expect("create restore root");
    }
    let skill = fixture.skills_home.join("review");
    fs::create_dir(&skill).expect("create generation A skill");
    fs::write(skill.join("SKILL.md"), b"generation-a").expect("write generation A skill");
    let generation_a = OwnershipManifest::new(false, ["review"], []);
    fixture.write_manifest(generation_a.clone());
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &fixture.state_dir);
    store
        .publish_current(BackupRequest {
            backup_id: "backup-a".to_owned(),
            roots: BackupRoots {
                codex_home: fixture.codex_home.clone(),
                skills_home: fixture.skills_home.clone(),
                state_dir: fixture.state_dir.clone(),
            },
            ownership: Some(generation_a),
            locators: vec![
                Locator::new(RootId::SkillsHome, "review").expect("skill locator"),
                Locator::new(RootId::StateDir, "manifest-v1.json").expect("manifest locator"),
            ],
        })
        .expect("publish generation A");
    store
        .select_latest("backup-a")
        .expect("select generation A");
    fs::write(skill.join("SKILL.md"), b"external").expect("write unmanaged live skill");
    fixture.write_manifest(OwnershipManifest::new(false, [], []));
    let backup = store
        .load_latest()
        .expect("load selected backup")
        .expect("selected backup");

    // Act
    let result = build_restore_plan(&fixture.source_root, &backup);

    // Assert
    assert_eq!(
        result,
        Err(InstallerError::UnmanagedConflict { paths: vec![skill] })
    );
}

#[cfg(unix)]
#[test]
fn restore_plan_rejects_an_unsafe_live_destination_during_preflight() {
    // Arrange
    use std::os::unix::fs::symlink;

    let temporary = project_tempdir("restore-plan-unsafe-destination");
    let fixture = Fixture::new(temporary.path());
    for directory in [
        &fixture.codex_home,
        &fixture.skills_home,
        &fixture.state_dir,
    ] {
        fs::create_dir_all(directory).expect("create restore root");
    }
    let skill = fixture.skills_home.join("review");
    fs::create_dir(&skill).expect("create generation A skill");
    fs::write(skill.join("SKILL.md"), b"generation-a").expect("write generation A skill");
    let generation_a = OwnershipManifest::new(false, ["review"], []);
    fixture.write_manifest(generation_a.clone());
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &fixture.state_dir);
    store
        .publish_current(BackupRequest {
            backup_id: "backup-a".to_owned(),
            roots: BackupRoots {
                codex_home: fixture.codex_home.clone(),
                skills_home: fixture.skills_home.clone(),
                state_dir: fixture.state_dir.clone(),
            },
            ownership: Some(generation_a),
            locators: vec![
                Locator::new(RootId::SkillsHome, "review").expect("skill locator"),
                Locator::new(RootId::StateDir, "manifest-v1.json").expect("manifest locator"),
            ],
        })
        .expect("publish generation A");
    store
        .select_latest("backup-a")
        .expect("select generation A");
    fs::remove_dir_all(&skill).expect("remove generation A skill");
    let external = temporary.path().join("external");
    fs::create_dir(&external).expect("create external directory");
    symlink(&external, &skill).expect("create unsafe destination symlink");
    let backup = store
        .load_latest()
        .expect("load selected backup")
        .expect("selected backup");

    // Act
    let result = build_restore_plan(&fixture.source_root, &backup);

    // Assert
    assert!(matches!(
        result,
        Err(InstallerError::UnsafePath { path, .. }) if path == skill
    ));
}

#[test]
fn restore_plan_uses_selected_latest_backup_not_newer_unselected_generation() {
    // Arrange
    let temporary = project_tempdir("restore-plan-latest-only");
    let fixture = Fixture::new(temporary.path());
    for directory in [
        &fixture.codex_home,
        &fixture.skills_home,
        &fixture.state_dir,
    ] {
        fs::create_dir_all(directory).expect("create restore root");
    }
    let config = fixture.codex_home.join("config.toml");
    fs::write(&config, b"generation-a").expect("write generation A config");
    let roots = BackupRoots {
        codex_home: fixture.codex_home.clone(),
        skills_home: fixture.skills_home.clone(),
        state_dir: fixture.state_dir.clone(),
    };
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &fixture.state_dir);
    store
        .publish_current(BackupRequest {
            backup_id: "backup-a".to_owned(),
            roots: roots.clone(),
            ownership: None,
            locators: vec![Locator::new(RootId::CodexHome, "config.toml").expect("config locator")],
        })
        .expect("publish generation A");
    store
        .select_latest("backup-a")
        .expect("select generation A");
    fs::write(&config, b"generation-b").expect("write generation B config");
    store
        .publish_current(BackupRequest {
            backup_id: "backup-b".to_owned(),
            roots,
            ownership: None,
            locators: vec![Locator::new(RootId::CodexHome, "config.toml").expect("config locator")],
        })
        .expect("publish unselected generation B");
    let backup = store
        .load_latest()
        .expect("load selected backup")
        .expect("selected backup");

    // Act
    let result = build_restore_plan(&fixture.source_root, &backup);

    // Assert
    let config_action = action_for(result.expect("restore plan"), AssetCategory::Config, None);
    assert_eq!(config_action.operation, PlanOperation::Replace);
    assert_eq!(
        config_action.desired,
        Some(CapturedContent::file(b"generation-a".to_vec()))
    );
    assert_eq!(
        fs::read(fixture.state_dir.join("backups/latest")).expect("read latest marker"),
        b"backup-a\n"
    );
}

#[test]
fn restore_planning_rejects_corrupt_latest_backup_before_building_a_plan() {
    // Arrange
    let temporary = project_tempdir("restore-plan-corrupt-backup");
    let fixture = Fixture::new(temporary.path());
    for directory in [
        &fixture.codex_home,
        &fixture.skills_home,
        &fixture.state_dir,
    ] {
        fs::create_dir_all(directory).expect("create restore root");
    }
    let config = fixture.codex_home.join("config.toml");
    fs::write(&config, b"generation-a").expect("write generation A config");
    let platform = MacOsPlatform::new();
    let store = BackupStore::new(&platform, &fixture.state_dir);
    let backup = store
        .publish_current(BackupRequest {
            backup_id: "backup-a".to_owned(),
            roots: BackupRoots {
                codex_home: fixture.codex_home.clone(),
                skills_home: fixture.skills_home.clone(),
                state_dir: fixture.state_dir.clone(),
            },
            ownership: None,
            locators: vec![Locator::new(RootId::CodexHome, "config.toml").expect("config locator")],
        })
        .expect("publish generation A");
    store
        .select_latest("backup-a")
        .expect("select generation A");
    fs::write(
        backup.directory.join("payload/codex-home/config.toml"),
        b"corrupt",
    )
    .expect("corrupt selected backup");
    let marker_before =
        fs::read(fixture.state_dir.join("backups/latest")).expect("read latest marker");

    // Act
    let result = store.load_latest();

    // Assert
    assert_eq!(
        result,
        Err(InstallerError::InvalidBackup {
            message: "backup payload fingerprint does not match journal".to_owned(),
        })
    );
    assert_eq!(
        fs::read(fixture.state_dir.join("backups/latest")).expect("reread latest marker"),
        marker_before
    );
    assert_eq!(fs::read(config).expect("read live config"), b"generation-a");
}

#[derive(Clone)]
struct Fixture {
    source_root: PathBuf,
    codex_home: PathBuf,
    skills_home: PathBuf,
    state_dir: PathBuf,
}

impl Fixture {
    fn new(parent: &Path) -> Self {
        let fixture = Self {
            source_root: parent.join("source"),
            codex_home: parent.join("codex-home"),
            skills_home: parent.join("skills-home"),
            state_dir: parent.join("state"),
        };
        fs::create_dir(&fixture.source_root).expect("create source");
        fs::write(fixture.source_root.join("config.toml"), MANAGED_CONFIG)
            .expect("write managed config");
        fixture
    }

    fn request(&self, adopt_existing: bool) -> InstallPlanRequest {
        InstallPlanRequest {
            source_root: self.source_root.clone(),
            codex_home: self.codex_home.clone(),
            skills_home: self.skills_home.clone(),
            state_dir: self.state_dir.clone(),
            adopt_existing,
            requested_threads: "6".to_owned(),
            resources: MachineResources {
                logical_cpus: 1,
                memory_bytes: 0,
            },
        }
    }

    fn add_skill(&self, name: &str, bytes: &[u8]) {
        let skill = self.source_root.join("skills").join(name);
        fs::create_dir_all(&skill).expect("create source skill");
        fs::write(skill.join("SKILL.md"), bytes).expect("write source skill");
    }

    fn add_agent(&self, name: &str, bytes: &[u8]) {
        let agents = self.source_root.join("agents");
        fs::create_dir_all(&agents).expect("create source agents");
        fs::write(agents.join(name), bytes).expect("write source agent");
    }

    fn write_manifest(&self, manifest: OwnershipManifest) {
        fs::create_dir_all(&self.state_dir).expect("create state");
        let content = manifest_content(&manifest).expect("serialize manifest");
        fs::write(
            self.state_dir.join("manifest-v1.json"),
            content.file_bytes().expect("manifest file"),
        )
        .expect("write manifest");
    }
}

fn action_for(plan: InstallPlan, category: AssetCategory, name: Option<&str>) -> super::PlanAction {
    plan.actions
        .into_iter()
        .find(|action| action.category == category && action.name.as_deref() == name)
        .expect("planned action")
}
