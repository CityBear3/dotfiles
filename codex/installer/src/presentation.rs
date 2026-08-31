use std::fmt::Write as _;
use std::path::Path;

use crate::{
    InstallerError, OperationAssetCategory, OperationMode, OperationReport, ReportOperation,
};

/// Styling capability selected by the outer presentation adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderingCapability {
    /// Emit deterministic text without terminal escapes.
    Plain,
    /// Request status color when the renderer supports it.
    Color,
}

/// Process-independent inputs used to render one complete result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderContext<'a> {
    home: Option<&'a Path>,
    capability: RenderingCapability,
}

impl<'a> RenderContext<'a> {
    pub const fn new(home: Option<&'a Path>, capability: RenderingCapability) -> Self {
        Self { home, capability }
    }

    pub const fn home(self) -> Option<&'a Path> {
        self.home
    }

    pub const fn capability(self) -> RenderingCapability {
        self.capability
    }
}

/// Render one successful semantic report without writing to a process stream.
pub fn render_report(report: &OperationReport, context: RenderContext<'_>) -> String {
    // The explicit capability boundary allows status styling to be added
    // without changing the plain text semantics fixed here.
    let _capability = context.capability();

    match report.mode() {
        OperationMode::InstallDryRun { max_threads } => {
            render_dry_run(report, context.home(), max_threads)
        }
        OperationMode::CompletedInstall => {
            render_completed(report, context.home(), "Install", "Already up to date")
        }
        OperationMode::CompletedRestore => render_completed(
            report,
            context.home(),
            "Restore",
            "Already matches latest backup",
        ),
    }
}

/// Render one typed installer error without writing to a process stream.
pub fn render_error(error: &InstallerError, context: RenderContext<'_>) -> String {
    // Diagnostic paths remain the absolute paths already carried by the typed
    // error, so home context is intentionally not applied.
    let _capability = context.capability();
    if matches!(
        error,
        InstallerError::Cli {
            exit_code: 0,
            use_stderr: false,
            ..
        }
    ) {
        return error.to_string();
    }

    let status = if matches!(error, InstallerError::CommittedCleanupIncomplete { .. }) {
        "!"
    } else {
        "✗"
    };
    let mut output = format!("{status}  {error}");
    if !output.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn render_dry_run(report: &OperationReport, home: Option<&Path>, max_threads: u8) -> String {
    let rows = report
        .entries()
        .iter()
        .map(|entry| TableRow {
            status: match entry.operation() {
                ReportOperation::NoOp => "–",
                ReportOperation::Create | ReportOperation::Replace | ReportOperation::Remove => "•",
            },
            action: operation_label(entry.operation()),
            asset: asset_label(entry.category(), entry.name()),
            path: display_path(entry.path(), home),
        })
        .collect::<Vec<_>>();

    format!(
        "Dry run · max threads {max_threads}\n\n{}",
        render_table(&rows)
    )
}

fn render_completed(
    report: &OperationReport,
    home: Option<&Path>,
    mode: &str,
    no_change_message: &str,
) -> String {
    let unchanged = report
        .entries()
        .iter()
        .filter(|entry| entry.operation() == ReportOperation::NoOp)
        .count();
    let rows = report
        .entries()
        .iter()
        .filter(|entry| entry.operation() != ReportOperation::NoOp)
        .map(|entry| TableRow {
            status: "✓",
            action: operation_label(entry.operation()),
            asset: asset_label(entry.category(), entry.name()),
            path: display_path(entry.path(), home),
        })
        .collect::<Vec<_>>();

    if rows.is_empty() {
        return format!("✓  {no_change_message} · {unchanged} unchanged\n");
    }

    let changed = rows.len();
    format!(
        "{}\n🍺 {mode} complete · {changed} changed · {unchanged} unchanged\n",
        render_table(&rows)
    )
}

struct TableRow {
    status: &'static str,
    action: &'static str,
    asset: String,
    path: String,
}

fn render_table(rows: &[TableRow]) -> String {
    let status_width = column_width("STATUS", rows.iter().map(|row| row.status));
    let action_width = column_width("ACTION", rows.iter().map(|row| row.action));
    let asset_width = column_width("ASSET", rows.iter().map(|row| row.asset.as_str()));
    let path_width = column_width("PATH", rows.iter().map(|row| row.path.as_str()));
    let mut output = String::new();

    write_table_line(
        &mut output,
        "STATUS",
        "ACTION",
        "ASSET",
        "PATH",
        [status_width, action_width, asset_width],
    );
    write_table_line(
        &mut output,
        &"-".repeat(status_width),
        &"-".repeat(action_width),
        &"-".repeat(asset_width),
        &"-".repeat(path_width),
        [status_width, action_width, asset_width],
    );
    for row in rows {
        write_table_line(
            &mut output,
            row.status,
            row.action,
            &row.asset,
            &row.path,
            [status_width, action_width, asset_width],
        );
    }

    output
}

fn column_width<'a>(header: &str, values: impl Iterator<Item = &'a str>) -> usize {
    values
        .map(|value| value.chars().count())
        .fold(header.chars().count(), usize::max)
}

fn write_table_line(
    output: &mut String,
    status: &str,
    action: &str,
    asset: &str,
    path: &str,
    widths: [usize; 3],
) {
    writeln!(
        output,
        "{status:<status_width$}  {action:<action_width$}  {asset:<asset_width$}  {path}",
        status_width = widths[0],
        action_width = widths[1],
        asset_width = widths[2],
    )
    .expect("writing to a String cannot fail");
}

fn display_path(path: &Path, home: Option<&Path>) -> String {
    if let Some(home) = home {
        if path == home {
            return "~".to_owned();
        }
        if let Ok(relative) = path.strip_prefix(home)
            && !relative.as_os_str().is_empty()
        {
            return format!("~/{}", relative.display());
        }
    }
    path.display().to_string()
}

fn operation_label(operation: ReportOperation) -> &'static str {
    match operation {
        ReportOperation::Create => "CREATE",
        ReportOperation::Replace => "REPLACE",
        ReportOperation::Remove => "REMOVE",
        ReportOperation::NoOp => "NO-OP",
    }
}

fn asset_label(category: OperationAssetCategory, name: Option<&str>) -> String {
    let category = category_label(category);
    match name {
        Some(name) => format!("{category}/{name}"),
        None => category.to_owned(),
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
