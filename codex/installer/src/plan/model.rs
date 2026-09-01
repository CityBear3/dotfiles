use std::path::PathBuf;

use crate::content::CapturedContent;
use crate::path::{InstallRoots, Locator};
use crate::resources::MachineResources;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PlanOperation {
    Create,
    Replace,
    Remove,
    NoOp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AssetCategory {
    Config,
    GlobalAgents,
    Skill,
    Agent,
    Manifest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlanAction {
    pub(crate) operation: PlanOperation,
    pub(crate) category: AssetCategory,
    pub(crate) name: Option<String>,
    pub(crate) locator: Locator,
    pub(crate) desired: Option<CapturedContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct InstallPlan {
    pub(crate) roots: InstallRoots,
    pub(crate) max_threads: u8,
    pub(crate) actions: Vec<PlanAction>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct InstallPlanRequest {
    pub(crate) source_root: PathBuf,
    pub(crate) codex_home: PathBuf,
    pub(crate) skills_home: PathBuf,
    pub(crate) state_dir: PathBuf,
    pub(crate) adopt_existing: bool,
    pub(crate) requested_threads: String,
    pub(crate) resources: MachineResources,
}
