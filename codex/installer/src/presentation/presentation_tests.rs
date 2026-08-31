use std::path::{Path, PathBuf};

use crate::{
    InstallerError, OperationAssetCategory, OperationMode, OperationReport, OperationReportEntry,
    ReportOperation,
};

use super::{RenderContext, RenderingCapability, render_error, render_report};

fn render_report_subject(report: &OperationReport, home: Option<&Path>) -> String {
    render_report(report, RenderContext::new(home, RenderingCapability::Plain))
}

fn render_error_subject(error: &InstallerError, home: Option<&Path>) -> String {
    render_error(error, RenderContext::new(home, RenderingCapability::Plain))
}

fn entry(
    operation: ReportOperation,
    category: OperationAssetCategory,
    name: Option<&str>,
    path: &str,
) -> OperationReportEntry {
    OperationReportEntry {
        operation,
        category,
        name: name.map(str::to_owned),
        path: PathBuf::from(path),
    }
}

#[test]
fn plain_dry_run_renders_every_action_in_order_with_context_and_complete_paths() {
    // Arrange
    let home = Path::new("/Users/example");
    let report = OperationReport {
        mode: OperationMode::InstallDryRun { max_threads: 6 },
        entries: vec![
            entry(
                ReportOperation::Create,
                OperationAssetCategory::Config,
                None,
                "/Users/example/.codex/config.toml",
            ),
            entry(
                ReportOperation::Replace,
                OperationAssetCategory::GlobalAgents,
                None,
                "/Users/example/.codex/AGENTS.md",
            ),
            entry(
                ReportOperation::NoOp,
                OperationAssetCategory::Skill,
                Some("review"),
                "/Users/example/.agents/skills/review",
            ),
            entry(
                ReportOperation::Remove,
                OperationAssetCategory::Agent,
                Some("legacy"),
                "/outside/a-very-long-path",
            ),
        ],
    };

    // Act
    let output = render_report_subject(&report, Some(home));
    let repeated = render_report_subject(&report, Some(home));

    // Assert
    assert_eq!(
        output,
        concat!(
            "Dry run · max threads 6\n",
            "\n",
            "STATUS  ACTION   ASSET          PATH\n",
            "------  -------  -------------  -------------------------\n",
            "•       CREATE   config         ~/.codex/config.toml\n",
            "•       REPLACE  global-agents  ~/.codex/AGENTS.md\n",
            "–       NO-OP    skill/review   ~/.agents/skills/review\n",
            "•       REMOVE   agent/legacy   /outside/a-very-long-path\n",
        )
    );
    assert!(!output.contains('\u{1b}'));
    assert!(!output.contains('✓'));
    assert!(!output.contains('🍺'));
    assert_eq!(repeated, output);
}

#[test]
fn plain_changed_install_renders_changed_rows_and_both_counts() {
    // Arrange
    let home = Path::new("/Users/example");
    let report = OperationReport {
        mode: OperationMode::CompletedInstall,
        entries: vec![
            entry(
                ReportOperation::Replace,
                OperationAssetCategory::Config,
                None,
                "/Users/example/.codex/config.toml",
            ),
            entry(
                ReportOperation::NoOp,
                OperationAssetCategory::Skill,
                Some("review"),
                "/Users/example/.agents/skills/review",
            ),
            entry(
                ReportOperation::Remove,
                OperationAssetCategory::Agent,
                Some("legacy"),
                "/outside/a-very-long-path",
            ),
        ],
    };

    // Act
    let output = render_report_subject(&report, Some(home));

    // Assert
    assert_eq!(
        output,
        concat!(
            "STATUS  ACTION   ASSET         PATH\n",
            "------  -------  ------------  -------------------------\n",
            "✓       REPLACE  config        ~/.codex/config.toml\n",
            "✓       REMOVE   agent/legacy  /outside/a-very-long-path\n",
            "\n",
            "🍺 Install complete · 2 changed · 1 unchanged\n",
        )
    );
    assert!(!output.contains("NO-OP"));
    assert!(!output.contains('\u{1b}'));
}

#[test]
fn plain_changed_restore_names_restore_and_preserves_plan_order() {
    // Arrange
    let home = Path::new("/Users/example");
    let report = OperationReport {
        mode: OperationMode::CompletedRestore,
        entries: vec![
            entry(
                ReportOperation::Create,
                OperationAssetCategory::GlobalAgents,
                None,
                "/Users/example/.codex/AGENTS.md",
            ),
            entry(
                ReportOperation::NoOp,
                OperationAssetCategory::Manifest,
                None,
                "/outside/state/manifest-v1.json",
            ),
            entry(
                ReportOperation::Replace,
                OperationAssetCategory::Skill,
                Some("review"),
                "/Users/example/.agents/skills/review",
            ),
        ],
    };

    // Act
    let output = render_report_subject(&report, Some(home));

    // Assert
    assert_eq!(
        output,
        concat!(
            "STATUS  ACTION   ASSET          PATH\n",
            "------  -------  -------------  -----------------------\n",
            "✓       CREATE   global-agents  ~/.codex/AGENTS.md\n",
            "✓       REPLACE  skill/review   ~/.agents/skills/review\n",
            "\n",
            "🍺 Restore complete · 2 changed · 1 unchanged\n",
        )
    );
    assert!(!output.contains("manifest"));
    assert!(!output.contains('\u{1b}'));
}

#[test]
fn plain_install_no_op_omits_table_and_completion_icon() {
    // Arrange
    let report = OperationReport {
        mode: OperationMode::CompletedInstall,
        entries: vec![
            entry(
                ReportOperation::NoOp,
                OperationAssetCategory::Config,
                None,
                "/Users/example/.codex/config.toml",
            ),
            entry(
                ReportOperation::NoOp,
                OperationAssetCategory::Manifest,
                None,
                "/outside/state/manifest-v1.json",
            ),
        ],
    };

    // Act
    let output = render_report_subject(&report, Some(Path::new("/Users/example")));

    // Assert
    assert_eq!(output, "✓  Already up to date · 2 unchanged\n");
    assert!(!output.contains("STATUS"));
    assert!(!output.contains('🍺'));
    assert!(!output.contains('\u{1b}'));
}

#[test]
fn plain_restore_no_op_omits_table_and_completion_icon() {
    // Arrange
    let report = OperationReport {
        mode: OperationMode::CompletedRestore,
        entries: vec![entry(
            ReportOperation::NoOp,
            OperationAssetCategory::Manifest,
            None,
            "/outside/state/manifest-v1.json",
        )],
    };

    // Act
    let output = render_report_subject(&report, Some(Path::new("/Users/example")));

    // Assert
    assert_eq!(output, "✓  Already matches latest backup · 1 unchanged\n");
    assert!(!output.contains("STATUS"));
    assert!(!output.contains('🍺'));
    assert!(!output.contains('\u{1b}'));
}

#[test]
fn explicit_missing_home_context_keeps_success_paths_absolute() {
    // Arrange
    let report = OperationReport {
        mode: OperationMode::InstallDryRun { max_threads: 2 },
        entries: vec![entry(
            ReportOperation::Create,
            OperationAssetCategory::Config,
            None,
            "/Users/example",
        )],
    };

    // Act
    let output = render_report_subject(&report, None);

    // Assert
    assert_eq!(
        output,
        concat!(
            "Dry run · max threads 2\n",
            "\n",
            "STATUS  ACTION  ASSET   PATH\n",
            "------  ------  ------  --------------\n",
            "•       CREATE  config  /Users/example\n",
        )
    );
}

#[test]
fn explicit_home_context_renders_the_home_path_as_tilde() {
    // Arrange
    let home = Path::new("/Users/example");
    let report = OperationReport {
        mode: OperationMode::InstallDryRun { max_threads: 4 },
        entries: vec![entry(
            ReportOperation::NoOp,
            OperationAssetCategory::Manifest,
            None,
            "/Users/example",
        )],
    };

    // Act
    let output = render_report_subject(&report, Some(home));

    // Assert
    assert_eq!(
        output,
        concat!(
            "Dry run · max threads 4\n",
            "\n",
            "STATUS  ACTION  ASSET     PATH\n",
            "------  ------  --------  ----\n",
            "–       NO-OP   manifest  ~\n",
        )
    );
    assert!(!output.contains('\u{1b}'));
}

#[test]
fn ordinary_failure_has_failure_heading_and_preserves_detail() {
    // Arrange
    let error = InstallerError::Filesystem {
        message: "cannot read /absolute/live/config.toml".to_owned(),
    };

    // Act
    let output = render_error_subject(&error, Some(Path::new("/absolute")));

    // Assert
    assert_eq!(output, "✗  cannot read /absolute/live/config.toml\n");
    assert!(!output.contains("STATUS"));
    assert!(!output.contains('🍺'));
    assert!(!output.contains('\u{1b}'));
}

#[test]
fn clean_rollback_has_failure_heading_and_transaction_detail() {
    // Arrange
    let error = InstallerError::TransactionRolledBack {
        transaction_id: "tx-clean".to_owned(),
        cause: Box::new(InstallerError::Filesystem {
            message: "rename failed at /absolute/live".to_owned(),
        }),
    };

    // Act
    let output = render_error_subject(&error, None);

    // Assert
    assert_eq!(
        output,
        concat!(
            "✗  transaction tx-clean failed and was rolled back: ",
            "rename failed at /absolute/live\n",
        )
    );
    assert!(!output.contains("STATUS"));
    assert!(!output.contains('🍺'));
}

#[test]
fn rollback_failure_has_failure_heading_and_absolute_recovery_paths() {
    // Arrange
    let error = InstallerError::TransactionRollbackFailed {
        transaction_id: "tx-rollback".to_owned(),
        wal: PathBuf::from("/absolute/state/transaction/wal-v1.json"),
        paths: vec![
            PathBuf::from("/absolute/live"),
            PathBuf::from("/absolute/stage"),
        ],
        cause: Some(Box::new(InstallerError::Filesystem {
            message: "apply failed".to_owned(),
        })),
        rollback_cause: Box::new(InstallerError::Filesystem {
            message: "rollback rename failed".to_owned(),
        }),
    };

    // Act
    let output = render_error_subject(&error, Some(Path::new("/absolute")));

    // Assert
    assert_eq!(
        output,
        concat!(
            "✗  transaction tx-rollback rollback failed at ",
            "/absolute/state/transaction/wal-v1.json: rollback rename failed; ",
            "original cause: Some(Filesystem { message: \"apply failed\" }); ",
            "paths: [\"/absolute/live\", \"/absolute/stage\"]\n",
        )
    );
    assert!(!output.contains("STATUS"));
    assert!(!output.contains('🍺'));
}

#[test]
fn unclassifiable_transaction_has_failure_heading_and_absolute_recovery_paths() {
    // Arrange
    let error = InstallerError::UnclassifiableTransaction {
        transaction_id: "tx-unknown".to_owned(),
        wal: PathBuf::from("/absolute/state/transaction/wal-v1.json"),
        paths: vec![PathBuf::from("/absolute/live")],
        message: "both move endpoints exist".to_owned(),
    };

    // Act
    let output = render_error_subject(&error, Some(Path::new("/absolute")));

    // Assert
    assert_eq!(
        output,
        concat!(
            "✗  transaction tx-unknown state cannot be classified at ",
            "/absolute/state/transaction/wal-v1.json: both move endpoints exist; ",
            "paths: [\"/absolute/live\"]\n",
        )
    );
    assert!(!output.contains("STATUS"));
    assert!(!output.contains('🍺'));
}

#[test]
fn unresolved_wal_authority_has_failure_heading_and_absolute_recovery_paths() {
    // Arrange
    let error = InstallerError::UnresolvedWalAuthority {
        transaction_id: "tx-wal".to_owned(),
        wal: PathBuf::from("/absolute/state/transaction/wal-v1.json"),
        paths: vec![PathBuf::from("/absolute/tombstone")],
        message: "canonical reload failed".to_owned(),
    };

    // Act
    let output = render_error_subject(&error, Some(Path::new("/absolute")));

    // Assert
    assert_eq!(
        output,
        concat!(
            "✗  transaction tx-wal WAL authority is unresolved at ",
            "/absolute/state/transaction/wal-v1.json: canonical reload failed; ",
            "paths: [\"/absolute/tombstone\"]\n",
        )
    );
    assert!(!output.contains("STATUS"));
    assert!(!output.contains('🍺'));
}

#[test]
fn committed_cleanup_incomplete_uses_warning_heading_and_preserves_guidance() {
    // Arrange
    let error = InstallerError::CommittedCleanupIncomplete {
        transaction_id: "tx-committed".to_owned(),
        wal: PathBuf::from("/absolute/state/transaction/wal-v1.json"),
        paths: vec![PathBuf::from("/absolute/stage")],
        cause: None,
        cleanup_cause: Box::new(InstallerError::Filesystem {
            message: "cleanup failed".to_owned(),
        }),
    };

    // Act
    let output = render_error_subject(&error, Some(Path::new("/absolute")));

    // Assert
    assert_eq!(
        output,
        concat!(
            "!  transaction tx-committed committed live state, but ",
            "finalization/cleanup is incomplete at ",
            "/absolute/state/transaction/wal-v1.json; retry a mutating command: ",
            "cleanup failed; original cause: None; paths: [\"/absolute/stage\"]\n",
        )
    );
    assert!(!output.contains("STATUS"));
    assert!(!output.contains('🍺'));
    assert!(!output.contains('\u{1b}'));
}

#[test]
fn clap_display_output_is_not_decorated_as_an_installer_failure() {
    // Arrange
    let error = InstallerError::Cli {
        message: "Usage: dotfiles-codex-installer install [OPTIONS]\n".to_owned(),
        exit_code: 0,
        use_stderr: false,
    };

    // Act
    let output = render_error_subject(&error, None);

    // Assert
    assert_eq!(
        output,
        "Usage: dotfiles-codex-installer install [OPTIONS]\n"
    );
}
