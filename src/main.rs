mod branding;
mod cli;
mod commands;
mod dirs;
mod i18n;
mod releases;
mod shim;
mod validate;

use clap::Parser;
use cli::{Cli, Commands, LanguageAction};

#[tokio::main]
async fn main() {
    // Enable ANSI sequences on the Windows console (required for colors).
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    // If the binary was invoked with a shim name, act as a forwarder.
    // pvm use installs copies of pvm.exe as python.exe / pip.exe in ~/.pvm/bin/.
    if let Some(name) = exe_stem() {
        match name.as_str() {
            "python" | "python3" => std::process::exit(shim::forward_python()),
            #[cfg(windows)]
            "pythonw" => std::process::exit(shim::forward("pythonw.exe")),
            "pip" | "pip3" => std::process::exit(shim::forward_pip()),
            _ => {}
        }
    }

    // Load the preferred lang before any user-visible output.
    if let Ok(lang_file) = dirs::language_file() {
        i18n::init(i18n::load(&lang_file));
    }

    let raw_args: Vec<String> = std::env::args().collect();
    let (version_override, cleaned_args) = extract_version_prefix(raw_args);
    let cleaned_args = normalize_lang_args(cleaned_args);

    let cli = Cli::parse_from(&cleaned_args);

    if let Err(e) = dirs::ensure_dirs() {
        eprintln!("{}", t!("Error initializing PVM directories: {}", "Error inicializando directorios pvm: {}", e));
        std::process::exit(1);
    }

    let Some(command) = cli.command else {
        branding::print_help();
        return;
    };

    let result = match command {
        Commands::Install { version } => {
            commands::install::run(&version).await
        }

        Commands::Use { version } => {
            commands::use_ver::run(&version)
        }

        Commands::List => {
            commands::list::run()
        }

        Commands::ListRemote { filter } => {
            commands::list_remote::run(filter).await
        }

        Commands::Uninstall { version } => {
            commands::uninstall::run(&version)
        }

        Commands::Default { version } => {
            commands::default::run(&version)
        }

        Commands::Env { shell } => {
            commands::env::run(shell)
        }

        Commands::Venv { dir, args } => {
            commands::venv::run(&dir, &args, version_override.as_deref())
        }

        Commands::UninstallSelf => {
            commands::uninstall_self::run()
        }

        Commands::Language { action } => match action {
            Some(LanguageAction::Change { language }) => {
                commands::language::run_change(&language)
            }
            None => {
                commands::language::run_list()
            }
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn extract_no_version_flag() {
        let input = s(&["pvm", "list"]);
        let (ver, cleaned) = extract_version_prefix(input.clone());
        assert_eq!(ver, None);
        assert_eq!(cleaned, input);
    }

    #[test]
    fn extract_version_flag_stripped() {
        let input = s(&["pvm", "-3.12.4", "venv", ".venv"]);
        let (ver, cleaned) = extract_version_prefix(input);
        assert_eq!(ver, Some("3.12.4".to_string()));
        assert_eq!(cleaned, s(&["pvm", "venv", ".venv"]));
    }

    #[test]
    fn extract_only_first_version_flag() {
        let input = s(&["pvm", "-3.12.4", "-3.11.0", "venv", ".venv"]);
        let (ver, cleaned) = extract_version_prefix(input);
        assert_eq!(ver, Some("3.12.4".to_string()));
        assert_eq!(cleaned, s(&["pvm", "-3.11.0", "venv", ".venv"]));
    }

    #[test]
    fn extract_ignores_regular_flags() {
        let input = s(&["pvm", "--help"]);
        let (ver, cleaned) = extract_version_prefix(input.clone());
        assert_eq!(ver, None);
        assert_eq!(cleaned, input);
    }

    #[test]
    fn extract_version_at_end() {
        let input = s(&["pvm", "venv", ".venv", "-3.12.4"]);
        let (ver, cleaned) = extract_version_prefix(input);
        assert_eq!(ver, Some("3.12.4".to_string()));
        assert_eq!(cleaned, s(&["pvm", "venv", ".venv"]));
    }

    #[test]
    fn normalize_lang_dash_c() {
        let input = s(&["pvm", "lang", "-c", "english"]);
        let result = normalize_lang_args(input);
        assert_eq!(result, s(&["pvm", "lang", "change", "english"]));
    }

    #[test]
    fn normalize_lang_change_unchanged() {
        let input = s(&["pvm", "lang", "change", "español"]);
        let result = normalize_lang_args(input.clone());
        assert_eq!(result, input);
    }
}

fn exe_stem() -> Option<String> {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_lowercase()))
}

/// Extracts the first argument of the form `-X.Y.Z` and returns the version and cleaned args.
/// Allows `pvm -3.14.0 venv .venv` without clap seeing the version flag.
fn extract_version_prefix(args: Vec<String>) -> (Option<String>, Vec<String>) {
    let re = regex::Regex::new(r"^-(\d+\.\d+\.\d+)$").unwrap();
    let mut version = None;
    let mut cleaned = Vec::with_capacity(args.len());

    for arg in args {
        if version.is_none() {
            if let Some(caps) = re.captures(&arg) {
                version = Some(caps[1].to_string());
                continue;
            }
        }
        cleaned.push(arg);
    }

    (version, cleaned)
}

/// Normalizes `pvm lang -c <lang>` → `pvm lang change <lang>`
/// so clap sees the canonical subcommand instead of an unknown flag.
fn normalize_lang_args(args: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(args.len());
    for arg in args {
        let prev_is_lang = result.last().map(|s: &String| s == "lang").unwrap_or(false);
        if prev_is_lang && arg == "-c" {
            result.push("change".to_string());
        } else {
            result.push(arg);
        }
    }
    result
}
