use std::path::{Path, PathBuf};

use crate::plan::{AssetCategory, InstallPlan, PlanOperation};

/// The semantic mode represented by a successful installer operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationMode {
    InstallDryRun { max_threads: u8 },
    CompletedInstall,
    CompletedRestore,
}

/// An operation preserved from the installer plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReportOperation {
    Create,
    Replace,
    Remove,
    NoOp,
}

/// An asset category preserved from the installer plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationAssetCategory {
    Config,
    GlobalAgents,
    Skill,
    Agent,
    Manifest,
}

/// Presentation-neutral semantics for one planned destination.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationReportEntry {
    pub(crate) operation: ReportOperation,
    pub(crate) category: OperationAssetCategory,
    pub(crate) name: Option<String>,
    pub(crate) path: PathBuf,
}

impl OperationReportEntry {
    pub fn operation(&self) -> ReportOperation {
        self.operation
    }

    pub fn category(&self) -> OperationAssetCategory {
        self.category
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// A successful installer result without terminal presentation data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationReport {
    pub(crate) mode: OperationMode,
    pub(crate) entries: Vec<OperationReportEntry>,
}

impl OperationReport {
    pub fn mode(&self) -> OperationMode {
        self.mode
    }

    pub fn entries(&self) -> &[OperationReportEntry] {
        &self.entries
    }

    pub(crate) fn install_dry_run(plan: &InstallPlan) -> Self {
        Self::from_plan(
            OperationMode::InstallDryRun {
                max_threads: plan.max_threads,
            },
            plan,
        )
    }

    pub(crate) fn completed_install(plan: &InstallPlan) -> Self {
        Self::from_plan(OperationMode::CompletedInstall, plan)
    }

    pub(crate) fn completed_restore(plan: &InstallPlan) -> Self {
        Self::from_plan(OperationMode::CompletedRestore, plan)
    }

    fn from_plan(mode: OperationMode, plan: &InstallPlan) -> Self {
        let entries = plan
            .actions
            .iter()
            .map(|action| OperationReportEntry {
                operation: action.operation.into(),
                category: action.category.into(),
                name: action.name.clone(),
                path: plan.roots.resolve(&action.locator),
            })
            .collect();
        Self { mode, entries }
    }
}

impl From<PlanOperation> for ReportOperation {
    fn from(operation: PlanOperation) -> Self {
        match operation {
            PlanOperation::Create => Self::Create,
            PlanOperation::Replace => Self::Replace,
            PlanOperation::Remove => Self::Remove,
            PlanOperation::NoOp => Self::NoOp,
        }
    }
}

impl From<AssetCategory> for OperationAssetCategory {
    fn from(category: AssetCategory) -> Self {
        match category {
            AssetCategory::Config => Self::Config,
            AssetCategory::GlobalAgents => Self::GlobalAgents,
            AssetCategory::Skill => Self::Skill,
            AssetCategory::Agent => Self::Agent,
            AssetCategory::Manifest => Self::Manifest,
        }
    }
}
