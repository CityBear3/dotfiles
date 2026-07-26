use std::process::ExitCode;

fn main() -> ExitCode {
    match dotfiles_codex_installer::run_from(std::env::args_os(), |name| std::env::var_os(name)) {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            if error.use_stderr() {
                eprintln!("{error}");
            } else {
                print!("{error}");
            }
            ExitCode::from(error.exit_code())
        }
    }
}
