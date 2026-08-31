use std::process::ExitCode;

fn main() -> ExitCode {
    let home = std::env::var_os("HOME")
        .filter(|value| !value.is_empty())
        .map(std::path::PathBuf::from);
    let context = dotfiles_codex_installer::presentation::RenderContext::new(
        home.as_deref(),
        dotfiles_codex_installer::presentation::RenderingCapability::Plain,
    );

    match dotfiles_codex_installer::run_from(std::env::args_os(), |name| std::env::var_os(name)) {
        Ok(report) => {
            print!(
                "{}",
                dotfiles_codex_installer::presentation::render_report(&report, context)
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            let output = dotfiles_codex_installer::presentation::render_error(&error, context);
            if error.use_stderr() {
                eprint!("{output}");
            } else {
                print!("{output}");
            }
            ExitCode::from(error.exit_code())
        }
    }
}
