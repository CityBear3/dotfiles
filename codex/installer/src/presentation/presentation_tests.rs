use std::path::PathBuf;

use crate::{
    OperationAssetCategory, OperationMode, OperationReport, OperationReportEntry, ReportOperation,
};

use super::render_legacy;

#[test]
fn legacy_dry_run_text_preserves_pre_feature_bytes_in_report_order() {
    // Arrange
    let report = OperationReport {
        mode: OperationMode::InstallDryRun { max_threads: 6 },
        entries: vec![
            OperationReportEntry {
                operation: ReportOperation::Replace,
                category: OperationAssetCategory::Config,
                name: None,
                path: PathBuf::from("/absolute/codex/config.toml"),
            },
            OperationReportEntry {
                operation: ReportOperation::NoOp,
                category: OperationAssetCategory::Skill,
                name: Some("review".to_owned()),
                path: PathBuf::from("/absolute/skills/review"),
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
            "REPLACE config /absolute/codex/config.toml\n",
            "NO-OP skill review /absolute/skills/review\n",
        )
    );
    assert_eq!(second, first);
}
