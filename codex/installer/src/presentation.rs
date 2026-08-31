use crate::{OperationAssetCategory, OperationMode, OperationReport, ReportOperation};

/// Preserve the pre-feature CLI text while presentation Tasks replace it.
pub fn render_legacy(report: &OperationReport) -> String {
    match report.mode() {
        OperationMode::InstallDryRun { max_threads } => {
            let mut output = format!("dry-run: max_threads={max_threads}\n");
            for entry in report.entries() {
                let name = entry
                    .name()
                    .map(|name| format!(" {name}"))
                    .unwrap_or_default();
                output.push_str(&format!(
                    "{} {}{name} {}\n",
                    operation_label(entry.operation()),
                    category_label(entry.category()),
                    entry.path().display()
                ));
            }
            output
        }
        OperationMode::CompletedInstall => "install complete\n".to_owned(),
        OperationMode::CompletedRestore => "restore complete\n".to_owned(),
    }
}

fn operation_label(operation: ReportOperation) -> &'static str {
    match operation {
        ReportOperation::Create => "CREATE",
        ReportOperation::Replace => "REPLACE",
        ReportOperation::Remove => "REMOVE",
        ReportOperation::NoOp => "NO-OP",
    }
}

fn category_label(category: OperationAssetCategory) -> &'static str {
    match category {
        OperationAssetCategory::Config => "config",
        OperationAssetCategory::GlobalAgents => "global-agents",
        OperationAssetCategory::Skill => "skill",
        OperationAssetCategory::Agent => "agent",
        OperationAssetCategory::Manifest => "manifest",
    }
}

#[cfg(test)]
#[path = "presentation/presentation_tests.rs"]
mod tests;
