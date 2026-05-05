use anyhow::Result;
use crate::dirs;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Cmd,
}

pub fn run(shell: Option<Shell>) -> Result<()> {
    let bin = dirs::bin_dir()?;
    let bin_str = bin.display().to_string();

    let shell = shell.unwrap_or_else(detect_shell);

    match shell {
        Shell::PowerShell => print_powershell(&bin_str),
        Shell::Bash | Shell::Zsh => print_posix(&bin_str),
        Shell::Fish => print_fish(&bin_str),
        Shell::Cmd => print_cmd(&bin_str),
    }

    Ok(())
}

fn print_powershell(bin: &str) {
    println!("$env:PATH = \"{bin};\" + $env:PATH", bin = bin);
}

fn print_posix(bin: &str) {
    let bin_u = bin.replace('\\', "/");
    println!("export PATH=\"{bin}:$PATH\"", bin = bin_u);
}

fn print_fish(bin: &str) {
    let bin_u = bin.replace('\\', "/");
    println!("set -gx PATH \"{bin}\" $PATH", bin = bin_u);
}

fn print_cmd(bin: &str) {
    println!("@SET \"PATH={bin};%PATH%\"", bin = bin);
}

fn detect_shell() -> Shell {
    if std::env::var("PSModulePath").is_ok() {
        return Shell::PowerShell;
    }

    if let Ok(shell) = std::env::var("SHELL") {
        if shell.contains("fish") {
            return Shell::Fish;
        }

        if shell.contains("zsh") {
            return Shell::Zsh;
        }

        return Shell::Bash;
    }

    if cfg!(windows) {
        Shell::PowerShell
    } else {
        Shell::Bash
    }
}
