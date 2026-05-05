mod branding;
mod cli;
mod commands;
mod dirs;
mod releases;
mod shim;
mod validate;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    // Habilita secuencias ANSI en la consola de Windows (necesario para colores).
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    // Si el binario fue invocado con un nombre de shim, actuar como forwarder.
    // pvm use instala copias de pvm.exe como python.exe / pip.exe en ~/.pvm/bin/.
    if let Some(name) = exe_stem() {
        match name.as_str() {
            "python" | "python3" => std::process::exit(shim::forward_python()),
            #[cfg(windows)]
            "pythonw" => std::process::exit(shim::forward("pythonw.exe")),
            "pip" | "pip3" => std::process::exit(shim::forward_pip()),
            _ => {}
        }
    }

    let raw_args: Vec<String> = std::env::args().collect();
    let (version_override, cleaned_args) = extract_version_prefix(raw_args);

    let cli = Cli::parse_from(&cleaned_args);

    if let Err(e) = dirs::ensure_dirs() {
        eprintln!("Error inicializando directorios pvm: {}", e);
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
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn exe_stem() -> Option<String> {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_lowercase()))
}

/// Extrae el primer argumento con forma `-X.Y.Z` y devuelve la versión y los args limpios.
/// Permite `pvm -3.14.0 venv .venv` sin que clap vea el flag de versión.
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
