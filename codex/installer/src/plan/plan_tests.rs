use std::fs;
use std::path::{Path, PathBuf};

use crate::InstallerError;
use crate::content::CapturedContent;
use crate::ownership::{OwnershipManifest, manifest_content};
use crate::resources::MachineResources;
use crate::test_support::project_tempdir;

use super::{
    AssetCategory, InstallPlan, InstallPlanRequest, PlanOperation, plan_install, render_dry_run,
};

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
fn plan_merges_only_the_five_managed_configuration_keys() {
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
        "[agents]\n",
        "max_threads = 6\n",
        "max_depth = 1\n",
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
fn manifest_action_is_always_last_and_dry_run_output_is_deterministic() {
    // Arrange
    let temporary = project_tempdir("plan-manifest-last");
    let fixture = Fixture::new(temporary.path());
    fixture.add_skill("zeta", b"zeta");
    fixture.add_skill("alpha", b"alpha");

    // Act
    let plan = plan_install(fixture.request(false)).expect("ordered plan");
    let first = render_dry_run(&plan);
    let second = render_dry_run(&plan);

    // Assert
    assert_eq!(
        plan.actions.last().map(|action| action.category),
        Some(AssetCategory::Manifest)
    );
    assert_eq!(first, second);
    assert!(
        first
            .lines()
            .last()
            .is_some_and(|line| line.contains("manifest"))
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
