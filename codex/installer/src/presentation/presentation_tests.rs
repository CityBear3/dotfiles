use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::{
    InstallerError, OperationAssetCategory, OperationMode, OperationReport, OperationReportEntry,
    ReportOperation,
};

use super::{
    RenderContext, RenderingCapability, capability_for_destination, render_error, render_report,
};

const ANSI_BLUE: &str = "\u{1b}[34m";
const ANSI_GREEN: &str = "\u{1b}[32m";
const ANSI_LIGHT_GRAY: &str = "\u{1b}[90m";
const ANSI_RED: &str = "\u{1b}[31m";
const ANSI_RESET: &str = "\u{1b}[0m";
const ANSI_YELLOW: &str = "\u{1b}[33m";

fn render_report_subject(report: &OperationReport, home: Option<&Path>) -> String {
    render_report(report, RenderContext::new(home, RenderingCapability::Plain))
}

fn render_error_subject(error: &InstallerError, home: Option<&Path>) -> String {
    render_error(error, RenderContext::new(home, RenderingCapability::Plain))
}

fn render_report_with_color(report: &OperationReport, home: Option<&Path>) -> String {
    render_report(report, RenderContext::new(home, RenderingCapability::Color))
}

fn render_error_with_color(error: &InstallerError, home: Option<&Path>) -> String {
    render_error(error, RenderContext::new(home, RenderingCapability::Color))
}

fn strip_contracted_ansi(value: &str) -> String {
    [
        ANSI_BLUE,
        ANSI_GREEN,
        ANSI_LIGHT_GRAY,
        ANSI_RED,
        ANSI_YELLOW,
        ANSI_RESET,
    ]
    .into_iter()
    .fold(value.to_owned(), |plain, sequence| {
        plain.replace(sequence, "")
    })
}

fn assert_capability_case(
    destination_is_terminal: bool,
    no_color: Option<&str>,
    term: Option<&str>,
    expected: RenderingCapability,
) {
    assert_eq!(
        capability_for_destination(
            destination_is_terminal,
            no_color.map(OsStr::new),
            term.map(OsStr::new),
        ),
        expected,
    );
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
fn plain_table_escapes_terminal_controls_in_dynamic_cells_before_layout() {
    // Arrange
    let report = OperationReport {
        mode: OperationMode::InstallDryRun { max_threads: 6 },
        entries: vec![entry(
            ReportOperation::Create,
            OperationAssetCategory::Skill,
            Some("name\nSTATUS\t\u{1b}\u{7}\u{85}\\literal"),
            "/absolute/control\n🍺 Install complete\rSTATUS ACTION\t\u{1b}[31m\u{7}\u{85}\\literal",
        )],
    };
    let expected_asset = "skill/name\\nSTATUS\\t\\x1B\\x07\\x85\\\\literal";
    let expected_path = concat!(
        "/absolute/control\\n🍺 Install complete\\rSTATUS ACTION\\t",
        "\\x1B[31m\\x07\\x85\\\\literal",
    );
    let expected_asset_width = expected_asset.chars().count();
    let expected_path_width = expected_path.chars().count();

    // Act
    let output = render_report_subject(&report, None);
    let lines = output.lines().map(str::to_owned).collect::<Vec<_>>();

    // Assert
    assert_eq!(
        lines,
        vec![
            "Dry run · max threads 6".to_owned(),
            String::new(),
            format!(
                "STATUS  ACTION  {asset:<asset_width$}  PATH",
                asset = "ASSET",
                asset_width = expected_asset_width,
            ),
            format!(
                "------  ------  {}  {}",
                "-".repeat(expected_asset_width),
                "-".repeat(expected_path_width),
            ),
            format!("•       CREATE  {expected_asset}  {expected_path}"),
        ],
        "one escaped entry must determine exactly one complete physical table row",
    );
    assert!(!output.contains('\r'));
    assert!(!output.contains('\t'));
    assert!(!output.contains('\u{1b}'));
    assert!(!output.contains('\u{7}'));
    assert!(!output.contains('\u{85}'));
    assert!(!lines.iter().any(|line| line.starts_with('🍺')));
}

#[test]
fn plain_table_escapes_bidi_controls_and_line_separators_but_preserves_unicode() {
    // Arrange
    const BIDI_AND_LINE_SEPARATORS: &str = concat!(
        "\u{61c}\u{200e}\u{200f}",
        "\u{202a}\u{202b}\u{202c}\u{202d}\u{202e}",
        "\u{2066}\u{2067}\u{2068}\u{2069}",
        "\u{2028}\u{2029}",
    );
    const ESCAPED_BIDI_AND_LINE_SEPARATORS: &str = concat!(
        "\\u{61C}\\u{200E}\\u{200F}",
        "\\u{202A}\\u{202B}\\u{202C}\\u{202D}\\u{202E}",
        "\\u{2066}\\u{2067}\\u{2068}\\u{2069}",
        "\\u{2028}\\u{2029}",
    );
    let name =
        format!("\u{540d}\u{524d}\u{1f469}\u{200d}\u{1f4bb}{BIDI_AND_LINE_SEPARATORS}\u{7d42}");
    let path = format!(
        "/\u{7d76}\u{5bfe}/\u{1f469}\u{200d}\u{1f4bb}{BIDI_AND_LINE_SEPARATORS}/config.toml"
    );
    let expected_asset = format!(
        "skill/\u{540d}\u{524d}\u{1f469}\u{200d}\u{1f4bb}{ESCAPED_BIDI_AND_LINE_SEPARATORS}\u{7d42}"
    );
    let expected_path = format!(
        "/\u{7d76}\u{5bfe}/\u{1f469}\u{200d}\u{1f4bb}{ESCAPED_BIDI_AND_LINE_SEPARATORS}/config.toml"
    );
    let report = OperationReport {
        mode: OperationMode::InstallDryRun { max_threads: 6 },
        entries: vec![entry(
            ReportOperation::Create,
            OperationAssetCategory::Skill,
            Some(&name),
            &path,
        )],
    };

    // Act
    let output = render_report_subject(&report, None);
    let lines = output.lines().collect::<Vec<_>>();

    // Assert
    assert_eq!(lines.len(), 5, "one entry must remain one physical row");
    assert_eq!(
        lines[4],
        format!("•       CREATE  {expected_asset}  {expected_path}")
    );
    assert!(output.contains("\u{540d}\u{524d}\u{1f469}\u{200d}\u{1f4bb}"));
    for character in BIDI_AND_LINE_SEPARATORS.chars() {
        assert!(!output.contains(character));
    }
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
fn plain_table_preserves_a_path_longer_than_a_typical_terminal_width() {
    // Arrange
    const LONG_PATH: &str = concat!(
        "/outside/this-path-is-deliberately-longer-than-one-hundred-and-twenty-characters-",
        "so-the-plain-renderer-must-preserve-every-character-without-an-ellipsis-or-any-",
        "other-truncation/config.toml",
    );
    assert!(LONG_PATH.chars().count() > 120);
    let report = OperationReport {
        mode: OperationMode::InstallDryRun { max_threads: 2 },
        entries: vec![entry(
            ReportOperation::Create,
            OperationAssetCategory::Config,
            None,
            LONG_PATH,
        )],
    };
    let expected_separator = "-".repeat(LONG_PATH.chars().count());

    // Act
    let output = render_report_subject(&report, None);
    let lines = output.lines().map(str::to_owned).collect::<Vec<_>>();

    // Assert
    assert_eq!(
        lines,
        vec![
            "Dry run · max threads 2".to_owned(),
            String::new(),
            "STATUS  ACTION  ASSET   PATH".to_owned(),
            format!("------  ------  ------  {expected_separator}"),
            format!("•       CREATE  config  {LONG_PATH}"),
        ]
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
fn non_clap_error_escapes_terminal_controls_without_losing_absolute_detail() {
    // Arrange
    let error = InstallerError::Filesystem {
        message: concat!(
            "/absolute/live\\literal\n🍺 Install complete\rSTATUS ACTION\t",
            "\u{1b}[31m\u{0}\u{85}remaining detail",
        )
        .to_owned(),
    };

    // Act
    let output = render_error_subject(&error, Some(Path::new("/absolute")));

    // Assert
    assert_eq!(
        output,
        concat!(
            "✗  /absolute/live\\\\literal\\n🍺 Install complete\\rSTATUS ACTION\\t",
            "\\x1B[31m\\x00\\x85remaining detail\n",
        )
    );
    assert_eq!(output.lines().count(), 1);
    assert!(!output.contains('\r'));
    assert!(!output.contains('\t'));
    assert!(!output.contains('\u{1b}'));
    assert!(!output.contains('\u{0}'));
    assert!(!output.contains('\u{85}'));
    assert!(!output.lines().any(|line| line.starts_with('🍺')));
    assert!(!output.contains("STATUS  ACTION  ASSET  PATH"));
}

#[test]
fn non_clap_error_escapes_bidi_controls_and_line_separators_but_preserves_unicode() {
    // Arrange
    const BIDI_AND_LINE_SEPARATORS: &str = concat!(
        "\u{61c}\u{200e}\u{200f}",
        "\u{202a}\u{202b}\u{202c}\u{202d}\u{202e}",
        "\u{2066}\u{2067}\u{2068}\u{2069}",
        "\u{2028}\u{2029}",
    );
    const ESCAPED_BIDI_AND_LINE_SEPARATORS: &str = concat!(
        "\\u{61C}\\u{200E}\\u{200F}",
        "\\u{202A}\\u{202B}\\u{202C}\\u{202D}\\u{202E}",
        "\\u{2066}\\u{2067}\\u{2068}\\u{2069}",
        "\\u{2028}\\u{2029}",
    );
    let error = InstallerError::Filesystem {
        message: format!(
            "/\u{7d76}\u{5bfe}/\u{8a73}\u{7d30}\u{1f469}\u{200d}\u{1f4bb}{BIDI_AND_LINE_SEPARATORS}\u{7d42}"
        ),
    };
    let expected = format!(
        "✗  /\u{7d76}\u{5bfe}/\u{8a73}\u{7d30}\u{1f469}\u{200d}\u{1f4bb}{ESCAPED_BIDI_AND_LINE_SEPARATORS}\u{7d42}\n"
    );

    // Act
    let output = render_error_subject(&error, Some(Path::new("/\u{7d76}\u{5bfe}")));

    // Assert
    assert_eq!(output, expected);
    assert_eq!(output.lines().count(), 1);
    assert!(output.contains("/\u{7d76}\u{5bfe}/\u{8a73}\u{7d30}\u{1f469}\u{200d}\u{1f4bb}"));
    for character in BIDI_AND_LINE_SEPARATORS.chars() {
        assert!(!output.contains(character));
    }
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
        message: concat!(
            "Usage:\tdotfiles-codex-installer install [OPTIONS]\n",
            "\u{1b}[1mhelp\u{1b}[0m\\literal\n",
        )
        .to_owned(),
        exit_code: 0,
        use_stderr: false,
    };

    // Act
    let output = render_error_subject(&error, None);

    // Assert
    assert_eq!(
        output,
        concat!(
            "Usage:\tdotfiles-codex-installer install [OPTIONS]\n",
            "\u{1b}[1mhelp\u{1b}[0m\\literal\n",
        )
    );
}

#[test]
fn color_dry_run_styles_only_planned_and_no_op_statuses() {
    // Arrange
    let report = OperationReport {
        mode: OperationMode::InstallDryRun { max_threads: 6 },
        entries: vec![
            entry(
                ReportOperation::Create,
                OperationAssetCategory::Config,
                None,
                "/outside/create",
            ),
            entry(
                ReportOperation::NoOp,
                OperationAssetCategory::Skill,
                Some("review"),
                "/outside/review",
            ),
        ],
    };
    let plain = render_report_subject(&report, None);

    // Act
    let output = render_report_with_color(&report, None);

    // Assert
    assert_eq!(
        output,
        concat!(
            "Dry run · max threads 6\n",
            "\n",
            "STATUS  ACTION  ASSET         PATH\n",
            "------  ------  ------------  ---------------\n",
            "\u{1b}[34m•\u{1b}[0m       CREATE  config        /outside/create\n",
            "\u{1b}[90m–\u{1b}[0m       NO-OP   skill/review  /outside/review\n",
        )
    );
    assert_eq!(strip_contracted_ansi(&output), plain);
    assert!(!output.contains("\u{1b}[34mCREATE"));
    assert!(!output.contains("\u{1b}[90mNO-OP"));
}

#[test]
fn color_changed_summaries_leave_actions_and_unchanged_counts_standard() {
    // Arrange
    let entries = vec![
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
    ];

    for (mode, mode_label) in [
        (OperationMode::CompletedInstall, "Install"),
        (OperationMode::CompletedRestore, "Restore"),
    ] {
        let report = OperationReport {
            mode,
            entries: entries.clone(),
        };
        let plain = render_report_subject(&report, Some(Path::new("/Users/example")));

        // Act
        let output = render_report_with_color(&report, Some(Path::new("/Users/example")));

        // Assert
        assert_eq!(
            output,
            format!(
                concat!(
                    "STATUS  ACTION   ASSET         PATH\n",
                    "------  -------  ------------  -------------------------\n",
                    "\u{1b}[32m✓\u{1b}[0m       REPLACE  config        ~/.codex/config.toml\n",
                    "\u{1b}[32m✓\u{1b}[0m       REMOVE   agent/legacy  /outside/a-very-long-path\n",
                    "\n",
                    "\u{1b}[32m🍺 {} complete · 2 changed · ",
                    "\u{1b}[0m1 unchanged\n",
                ),
                mode_label,
            )
        );
        assert_eq!(strip_contracted_ansi(&output), plain);
        assert!(
            !output.contains(ANSI_RED),
            "successful REMOVE must not be red"
        );
        assert!(!output.contains("\u{1b}[32mREPLACE"));
        assert!(!output.contains("\u{1b}[32mREMOVE"));
        assert!(!output.contains("\u{1b}[32m1 unchanged"));
    }
}

#[test]
fn color_no_op_summaries_style_only_the_status_icon() {
    // Arrange
    let cases = [
        (
            OperationMode::CompletedInstall,
            "Already up to date",
            "/absolute/config.toml",
        ),
        (
            OperationMode::CompletedRestore,
            "Already matches latest backup",
            "/absolute/manifest-v1.json",
        ),
    ];

    for (mode, message, path) in cases {
        let report = OperationReport {
            mode,
            entries: vec![entry(
                ReportOperation::NoOp,
                OperationAssetCategory::Manifest,
                None,
                path,
            )],
        };
        let plain = render_report_subject(&report, None);

        // Act
        let output = render_report_with_color(&report, None);

        // Assert
        assert_eq!(
            output,
            format!("{ANSI_GREEN}✓{ANSI_RESET}  {message} · 1 unchanged\n")
        );
        assert_eq!(strip_contracted_ansi(&output), plain);
        assert_eq!(output.matches(ANSI_GREEN).count(), 1);
        assert_eq!(output.matches(ANSI_RESET).count(), 1);
    }
}

#[test]
fn color_failure_classes_style_only_the_failure_heading() {
    // Arrange
    let errors = vec![
        InstallerError::Filesystem {
            message: "cannot read /absolute/live".to_owned(),
        },
        InstallerError::TransactionRolledBack {
            transaction_id: "tx-clean".to_owned(),
            cause: Box::new(InstallerError::Filesystem {
                message: "rename failed".to_owned(),
            }),
        },
        InstallerError::TransactionRollbackFailed {
            transaction_id: "tx-rollback".to_owned(),
            wal: PathBuf::from("/absolute/state/transaction/wal-v1.json"),
            paths: vec![PathBuf::from("/absolute/live")],
            cause: None,
            rollback_cause: Box::new(InstallerError::Filesystem {
                message: "rollback failed".to_owned(),
            }),
        },
        InstallerError::UnclassifiableTransaction {
            transaction_id: "tx-unknown".to_owned(),
            wal: PathBuf::from("/absolute/state/transaction/wal-v1.json"),
            paths: vec![PathBuf::from("/absolute/live")],
            message: "both endpoints exist".to_owned(),
        },
        InstallerError::UnresolvedWalAuthority {
            transaction_id: "tx-wal".to_owned(),
            wal: PathBuf::from("/absolute/state/transaction/wal-v1.json"),
            paths: vec![PathBuf::from("/absolute/tombstone")],
            message: "canonical reload failed".to_owned(),
        },
    ];

    for error in errors {
        let plain = render_error_subject(&error, None);

        // Act
        let output = render_error_with_color(&error, None);

        // Assert
        assert_eq!(
            output,
            format!("{ANSI_RED}✗{ANSI_RESET}{}", &plain['✗'.len_utf8()..])
        );
        assert_eq!(strip_contracted_ansi(&output), plain);
        assert_eq!(output.matches(ANSI_RED).count(), 1);
        assert_eq!(output.matches(ANSI_RESET).count(), 1);
    }
}

#[test]
fn color_committed_cleanup_incomplete_styles_only_the_warning_heading() {
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
    let plain = render_error_subject(&error, None);

    // Act
    let output = render_error_with_color(&error, None);

    // Assert
    assert_eq!(output, format!("{ANSI_YELLOW}!{ANSI_RESET}{}", &plain[1..]));
    assert_eq!(strip_contracted_ansi(&output), plain);
    assert_eq!(output.matches(ANSI_YELLOW).count(), 1);
    assert_eq!(output.matches(ANSI_RESET).count(), 1);
}

#[test]
fn color_rendering_never_reactivates_input_derived_ansi() {
    // Arrange
    let report = OperationReport {
        mode: OperationMode::InstallDryRun { max_threads: 2 },
        entries: vec![entry(
            ReportOperation::Create,
            OperationAssetCategory::Skill,
            Some("malicious\u{1b}[31mname"),
            "/absolute/malicious\u{1b}[33mpath",
        )],
    };
    let plain = render_report_subject(&report, None);

    // Act
    let output = render_report_with_color(&report, None);

    // Assert
    assert_eq!(output.matches('\u{1b}').count(), 2);
    assert!(output.contains("skill/malicious\\x1B[31mname"));
    assert!(output.contains("/absolute/malicious\\x1B[33mpath"));
    assert_eq!(strip_contracted_ansi(&output), plain);
}

#[test]
fn color_capability_does_not_decorate_successful_clap_display() {
    // Arrange
    let error = InstallerError::Cli {
        message: "Usage: dotfiles-codex-installer install [OPTIONS]\n".to_owned(),
        exit_code: 0,
        use_stderr: false,
    };

    // Act
    let output = render_error_with_color(&error, None);

    // Assert
    assert_eq!(
        output,
        "Usage: dotfiles-codex-installer install [OPTIONS]\n"
    );
    assert!(!output.contains('\u{1b}'));
}

#[test]
fn stdout_capability_truth_table_requires_tty_and_eligible_environment() {
    // Arrange / Act / Assert
    assert_capability_case(false, None, None, RenderingCapability::Plain);
    assert_capability_case(false, Some(""), Some("xterm"), RenderingCapability::Plain);
    assert_capability_case(true, None, None, RenderingCapability::Color);
    assert_capability_case(true, Some(""), Some("xterm"), RenderingCapability::Color);
    assert_capability_case(true, Some("1"), Some("xterm"), RenderingCapability::Plain);
    assert_capability_case(true, None, Some("dumb"), RenderingCapability::Plain);
    assert_capability_case(true, None, Some("DUMB"), RenderingCapability::Color);
}

#[test]
fn stderr_capability_truth_table_is_independent_of_stdout_policy_use() {
    // Arrange / Act / Assert
    let stderr_is_terminal = true;
    assert_capability_case(
        stderr_is_terminal,
        None,
        Some("xterm-256color"),
        RenderingCapability::Color,
    );
    assert_capability_case(
        !stderr_is_terminal,
        None,
        Some("xterm-256color"),
        RenderingCapability::Plain,
    );
    assert_capability_case(
        stderr_is_terminal,
        Some("0"),
        None,
        RenderingCapability::Plain,
    );
    assert_capability_case(
        stderr_is_terminal,
        Some(""),
        None,
        RenderingCapability::Color,
    );
    assert_capability_case(
        stderr_is_terminal,
        Some(""),
        Some("dumb"),
        RenderingCapability::Plain,
    );
}
