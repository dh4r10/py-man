mod cli;
mod commands;
mod dirs;
mod releases;
mod shim;
mod validate;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    // Si el binario fue invocado con un nombre de shim, actuar como forwarder.
    // pvm use instala copias de pvm.exe como python.exe / pip.exe en ~/.pvm/bin/.
    if let Some(name) = exe_stem() {
        match name.as_str() {
            "python" | "python3" => std::process::exit(shim::forward("python.exe")),
            "pythonw" => std::process::exit(shim::forward("pythonw.exe")),
            "pip" | "pip3" => std::process::exit(shim::forward_pip()),
            _ => {}
        }
    }

    let cli = Cli::parse();

    if let Err(e) = dirs::ensure_dirs() {
        eprintln!("Error inicializando directorios pvm: {}", e);
        std::process::exit(1);
    }

    let Some(command) = cli.command else {
        Cli::command().print_help().unwrap();
        println!();
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
            commands::venv::run(&dir, &args)
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
