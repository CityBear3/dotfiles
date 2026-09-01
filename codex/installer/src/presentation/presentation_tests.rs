use std::path::PathBuf;

use crate::{
    OperationAssetCategory, OperationMode, OperationReport, OperationReportEntry, ReportOperation,
};

use super::render_legacy;

#[test]
fn legacy_dry_run_text_covers_all_operations_and_categories_in_report_order() {
    // Arrange
    let report = OperationReport {
        mode: OperationMode::InstallDryRun { max_threads: 6 },
        entries: vec![
            OperationReportEntry {
                operation: ReportOperation::Create,
                category: OperationAssetCategory::Config,
                name: None,
                path: PathBuf::from("/absolute/codex/config.toml"),
            },
            OperationReportEntry {
                operation: ReportOperation::Replace,
                category: OperationAssetCategory::GlobalAgents,
                name: None,
                path: PathBuf::from("/absolute/codex/AGENTS.md"),
            },
            OperationReportEntry {
                operation: ReportOperation::Remove,
                category: OperationAssetCategory::Skill,
                name: Some("review".to_owned()),
                path: PathBuf::from("/absolute/skills/review"),
            },
            OperationReportEntry {
                operation: ReportOperation::NoOp,
                category: OperationAssetCategory::Agent,
                name: Some("task-orchestrator.toml".to_owned()),
                path: PathBuf::from("/absolute/codex/agents/task-orchestrator.toml"),
            },
            OperationReportEntry {
                operation: ReportOperation::Create,
                category: OperationAssetCategory::Manifest,
                name: None,
                path: PathBuf::from("/absolute/state/manifest-v1.json"),
            },
        ],
    };

    // Act
    let first = render_legacy(&report);
    let second = render_legacy(&report);

    // Assert
    assert_eq!(
        first,
        concat!(
            "dry-run: max_threads=6\n",
            "CREATE config /absolute/codex/config.toml\n",
            "REPLACE global-agents /absolute/codex/AGENTS.md\n",
            "REMOVE skill review /absolute/skills/review\n",
            "NO-OP agent task-orchestrator.toml ",
            "/absolute/codex/agents/task-orchestrator.toml\n",
            "CREATE manifest /absolute/state/manifest-v1.json\n",
        )
    );
    assert_eq!(second, first);
}

#[test]
fn legacy_completed_install_text_ignores_no_op_entries() {
    // Arrange
    let report = OperationReport {
        mode: OperationMode::CompletedInstall,
        entries: vec![OperationReportEntry {
            operation: ReportOperation::NoOp,
            category: OperationAssetCategory::Config,
            name: None,
            path: PathBuf::from("/absolute/codex/config.toml"),
        }],
    };

    // Act
    let output = render_legacy(&report);

    // Assert
    assert_eq!(output, "install complete\n");
}

#[test]
fn legacy_completed_restore_text_ignores_no_op_entries() {
    // Arrange
    let report = OperationReport {
        mode: OperationMode::CompletedRestore,
        entries: vec![OperationReportEntry {
            operation: ReportOperation::NoOp,
            category: OperationAssetCategory::Manifest,
            name: None,
            path: PathBuf::from("/absolute/state/manifest-v1.json"),
        }],
    };

    // Act
    let output = render_legacy(&report);

    // Assert
    assert_eq!(output, "restore complete\n");
}
