use std::io::{self, IsTerminal as _};
use std::path::Path;
use std::process::ExitCode;

use dotfiles_codex_installer::presentation::{
    OutputDestination, RenderContext, capability_for_destination, render_error, render_report,
};

fn main() -> ExitCode {
    let home = std::env::var_os("HOME")
        .filter(|value| !value.is_empty())
        .map(std::path::PathBuf::from);

    match dotfiles_codex_installer::run_from(std::env::args_os(), |name| std::env::var_os(name)) {
        Ok(report) => {
            let context = render_context(home.as_deref(), OutputDestination::Stdout);
            print!("{}", render_report(&report, context));
            ExitCode::SUCCESS
        }
        Err(error) => {
            let destination = if error.use_stderr() {
                OutputDestination::Stderr
            } else {
                OutputDestination::Stdout
            };
            let output = render_error(&error, render_context(home.as_deref(), destination));
            match destination {
                OutputDestination::Stdout => print!("{output}"),
                OutputDestination::Stderr => eprint!("{output}"),
            }
            ExitCode::from(error.exit_code())
        }
    }
}

fn render_context(home: Option<&Path>, destination: OutputDestination) -> RenderContext<'_> {
    let stdout_is_terminal = io::stdout().is_terminal();
    let stderr_is_terminal = io::stderr().is_terminal();
    let capability = capability_for_destination(
        destination,
        stdout_is_terminal,
        stderr_is_terminal,
        std::env::var_os("NO_COLOR").as_deref(),
        std::env::var_os("TERM").as_deref(),
    );
    RenderContext::new(home, capability)
}
