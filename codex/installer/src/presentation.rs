use std::ffi::OsStr;
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

/// Select rendering capability from explicitly supplied destination state.
pub fn capability_for_destination(
    destination_is_terminal: bool,
    no_color: Option<&OsStr>,
    term: Option<&OsStr>,
) -> RenderingCapability {
    if destination_is_terminal
        && no_color.is_none_or(OsStr::is_empty)
        && term != Some(OsStr::new("dumb"))
    {
        RenderingCapability::Color
    } else {
        RenderingCapability::Plain
    }
}

#[derive(Clone, Copy)]
enum AnsiColor {
    Blue,
    Green,
    LightGray,
    Red,
    Yellow,
}

impl AnsiColor {
    const fn code(self) -> u8 {
        match self {
            Self::Blue => 34,
            Self::Green => 32,
            Self::LightGray => 90,
            Self::Red => 31,
            Self::Yellow => 33,
        }
    }
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
    let capability = context.capability();

    match report.mode() {
        OperationMode::InstallDryRun { max_threads } => {
            render_dry_run(report, context.home(), max_threads, capability)
        }
        OperationMode::CompletedInstall => render_completed(
            report,
            context.home(),
            "Install",
            "Already up to date",
            capability,
        ),
        OperationMode::CompletedRestore => render_completed(
            report,
            context.home(),
            "Restore",
            "Already matches latest backup",
            capability,
        ),
    }
}

/// Render one typed installer error without writing to a process stream.
pub fn render_error(error: &InstallerError, context: RenderContext<'_>) -> String {
    // Diagnostic paths remain the absolute paths already carried by the typed
    // error, so home context is intentionally not applied.
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

    let (status, color) = if matches!(error, InstallerError::CommittedCleanupIncomplete { .. }) {
        ("!", AnsiColor::Yellow)
    } else {
        ("✗", AnsiColor::Red)
    };
    let detail = escape_terminal_text(&error.to_string());
    let status = styled(status, color, context.capability());
    let mut output = format!("{status}  {detail}");
    if !output.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn render_dry_run(
    report: &OperationReport,
    home: Option<&Path>,
    max_threads: u8,
    capability: RenderingCapability,
) -> String {
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
        render_table(&rows, capability)
    )
}

fn render_completed(
    report: &OperationReport,
    home: Option<&Path>,
    mode: &str,
    no_change_message: &str,
    capability: RenderingCapability,
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
        let status = styled("✓", AnsiColor::Green, capability);
        return format!("{status}  {no_change_message} · {unchanged} unchanged\n");
    }

    let changed = rows.len();
    let summary = styled(
        &format!("🍺 {mode} complete · {changed} changed · "),
        AnsiColor::Green,
        capability,
    );
    format!(
        "{}\n{summary}{unchanged} unchanged\n",
        render_table(&rows, capability)
    )
}

struct TableRow {
    status: &'static str,
    action: &'static str,
    asset: String,
    path: String,
}

fn render_table(rows: &[TableRow], capability: RenderingCapability) -> String {
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
        write_table_row(
            &mut output,
            row,
            [status_width, action_width, asset_width],
            capability,
        );
    }

    output
}

fn write_table_row(
    output: &mut String,
    row: &TableRow,
    widths: [usize; 3],
    capability: RenderingCapability,
) {
    let color = match row.status {
        "•" => AnsiColor::Blue,
        "–" => AnsiColor::LightGray,
        "✓" => AnsiColor::Green,
        _ => unreachable!("table status is renderer-owned"),
    };
    let status = styled(row.status, color, capability);
    let status_padding = widths[0] - row.status.chars().count();
    writeln!(
        output,
        "{status}{padding}  {action:<action_width$}  {asset:<asset_width$}  {path}",
        padding = " ".repeat(status_padding),
        action = row.action,
        action_width = widths[1],
        asset = row.asset,
        asset_width = widths[2],
        path = row.path,
    )
    .expect("writing to a String cannot fail");
}

fn styled(text: &str, color: AnsiColor, capability: RenderingCapability) -> String {
    match capability {
        RenderingCapability::Plain => text.to_owned(),
        RenderingCapability::Color => format!("\u{1b}[{}m{text}\u{1b}[0m", color.code()),
    }
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
    let display = if let Some(home) = home {
        if path == home {
            return "~".to_owned();
        }
        if let Ok(relative) = path.strip_prefix(home)
            && !relative.as_os_str().is_empty()
        {
            format!("~/{}", relative.display())
        } else {
            path.display().to_string()
        }
    } else {
        path.display().to_string()
    };
    escape_terminal_text(&display)
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
        Some(name) => format!("{category}/{}", escape_terminal_text(name)),
        None => category.to_owned(),
    }
}

fn escape_terminal_text(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character
                if character.is_control()
                    || is_bidi_control(character)
                    || matches!(character, '\u{2028}' | '\u{2029}') =>
            {
                let codepoint = character as u32;
                if codepoint <= u8::MAX.into() {
                    write!(escaped, "\\x{codepoint:02X}").expect("writing to a String cannot fail");
                } else {
                    write!(escaped, "\\u{{{codepoint:X}}}")
                        .expect("writing to a String cannot fail");
                }
            }
            character => escaped.push(character),
        }
    }
    escaped
}

fn is_bidi_control(character: char) -> bool {
    matches!(
        character,
        '\u{061c}'
            | '\u{200e}'
            | '\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2066}'..='\u{2069}'
    )
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
