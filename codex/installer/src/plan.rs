mod install;
mod model;
#[cfg(target_os = "macos")]
mod restore;

pub(crate) use install::{plan_install, render_dry_run};
pub(crate) use model::{AssetCategory, InstallPlan, InstallPlanRequest, PlanAction, PlanOperation};
#[cfg(target_os = "macos")]
pub(crate) use restore::build_restore_plan;

#[cfg(all(test, target_os = "macos"))]
#[path = "plan/plan_tests.rs"]
mod tests;
