use anyhow::{bail, Result};
use colored::Colorize;
use std::io::{self, Write};
use crate::t;

pub fn run() -> Result<()> {
    println!("{}", t!("PVM Uninstaller", "Desinstalador de PVM").bold().magenta());
    println!();
    println!("{}", t!("This will remove:", "Esto eliminará:"));
    println!("  • ~/.pvm/          {}", t!("(versions, aliases, shims)", "(versiones, aliases, shims)"));

    #[cfg(windows)]
    {
        println!("  • {}", t!("User PATH (PVM entry)", "PATH del usuario (entrada de PVM)"));
        println!("  • {}", t!("PowerShell profile (pvm env line)", "Perfil de PowerShell (línea pvm env)"));
        println!("  • {}", t!("pvm.exe installation directory", "Directorio de instalación de pvm.exe"));
    }

    #[cfg(not(windows))]
    {
        println!("  • {}", t!("~/.local/bin/pvm (binary)", "~/.local/bin/pvm (binario)"));
        println!("  • {}", t!(
            "PVM lines in .bashrc / .zshrc / config.fish",
            "Líneas de pvm en .bashrc / .zshrc / config.fish"
        ));
    }

    println!();

    // Use language-appropriate confirmation key
    let prompt = t!("Confirm uninstall? [y/N]: ", "¿Confirmar desinstalación? [s/N]: ");
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_lowercase();
    let confirmed = matches!(trimmed.as_str(), "y" | "yes" | "s" | "si" | "sí");

    if !confirmed {
        println!("{}", t!("Cancelled.", "Cancelado."));
        return Ok(());
    }

    println!();

    remove_pvm_data()?;
    remove_from_path()?;
    remove_from_profile()?;
    schedule_binary_removal()?;

    println!();
    println!("{}", t!("PVM uninstalled successfully.", "PVM desinstalado correctamente.").bold().green());
    println!("{}", t!(
        "Open a new terminal for the changes to take effect.",
        "Abre una nueva terminal para que los cambios surtan efecto."
    ));

    Ok(())
}

fn remove_pvm_data() -> Result<()> {
    let pvm_home = crate::dirs::pvm_home()?;
    if pvm_home.exists() {
        std::fs::remove_dir_all(&pvm_home)?;
        println!("{} ~/.pvm/", "✓".green());
    } else {
        println!("{} ~/.pvm/ {}", "·".dimmed(), t!("(already removed)", "(ya no existía)").dimmed());
    }
    Ok(())
}

fn remove_from_path() -> Result<()> {
    #[cfg(windows)]
    {
        use std::process::Command;

        let install_dir = install_dir()?;
        let install_str = install_dir.to_string_lossy();

        let script = format!(
            r#"
$key = 'HKCU:\Environment'
$old = (Get-ItemProperty -Path $key -Name PATH -ErrorAction SilentlyContinue).PATH
if ($old) {{
    $parts = $old -split ';' | Where-Object {{ $_ -ne '' -and $_.TrimEnd('\') -ne '{0}'.TrimEnd('\') }}
    $new = $parts -join ';'
    Set-ItemProperty -Path $key -Name PATH -Value $new
}}
"#,
            install_str.replace('\'', "''")
        );

        let status = Command::new("powershell")
            .args(["-NonInteractive", "-Command", &script])
            .status()?;

        if status.success() {
            println!("{} {}", "✓".green(), t!("User PATH", "PATH del usuario"));
        } else {
            eprintln!("{} {}", "!".yellow(), t!("Could not clean PATH", "No se pudo limpiar el PATH"));
        }
    }

    #[cfg(not(windows))]
    {
        // On Linux, PATH is managed via shell profiles — cleaned in remove_from_profile()
    }

    Ok(())
}

fn remove_from_profile() -> Result<()> {
    #[cfg(windows)]
    {
        use std::process::Command;

        let script = r#"
$profile_path = $PROFILE
if (Test-Path $profile_path) {
    $lines = Get-Content $profile_path
    $filtered = $lines | Where-Object {
        $_ -notmatch 'pvm env' -and $_ -notmatch '# pvm'
    }
    $filtered | Set-Content $profile_path
}
"#;

        let status = Command::new("powershell")
            .args(["-NonInteractive", "-Command", script])
            .status()?;

        if status.success() {
            println!("{} {}", "✓".green(), t!("PowerShell profile", "Perfil de PowerShell"));
        } else {
            eprintln!("{} {}", "!".yellow(), t!(
                "Could not clean PowerShell profile",
                "No se pudo limpiar el perfil de PowerShell"
            ));
        }
    }

    #[cfg(not(windows))]
    {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("{}", t!(
            "Could not find the home directory",
            "No se encontró el directorio home"
        )))?;

        let profiles = [
            home.join(".bashrc"),
            home.join(".zshrc"),
        ];

        for profile in &profiles {
            if profile.exists() {
                if let Err(e) = clean_shell_profile(profile) {
                    eprintln!("{} {}: {}", "!".yellow(), t!(
                        "Could not clean",
                        "No se pudo limpiar"
                    ), profile.display());
                    eprintln!("  {}", e);
                }
            }
        }

        let fish = home.join(".config/fish/config.fish");
        if fish.exists() {
            if let Err(e) = clean_shell_profile(&fish) {
                eprintln!("{} {}: {}", "!".yellow(), t!(
                    "Could not clean",
                    "No se pudo limpiar"
                ), fish.display());
                eprintln!("  {}", e);
            }
        }
    }

    Ok(())
}

/// Removes PVM-added lines from a shell profile.
/// Removes lines containing "pvm" and the "rehash" line that immediately follows.
#[cfg(not(windows))]
fn clean_shell_profile(path: &std::path::Path) -> Result<()> {
    let content = std::fs::read_to_string(path)?;
    let ends_with_newline = content.ends_with('\n');

    let mut new_lines: Vec<&str> = Vec::new();
    let mut skip_next_rehash = false;

    for line in content.lines() {
        let is_pvm_line = line.contains("pvm env")
            || line.contains("pvm env --shell fish")
            || (line.contains("fish_add_path") && line.contains(".local/bin"))
            || (line.contains("export PATH") && line.contains(".local/bin") && line.contains("pvm"))
            || line.contains("function pvm")
            || line.contains("pvm()") && line.contains("command pvm");

        if is_pvm_line {
            skip_next_rehash = true;
            continue;
        }

        if skip_next_rehash && line.trim() == "rehash" {
            skip_next_rehash = false;
            continue;
        }

        skip_next_rehash = false;
        new_lines.push(line);
    }

    while new_lines.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
        new_lines.pop();
    }

    let mut new_content = new_lines.join("\n");
    if ends_with_newline {
        new_content.push('\n');
    }

    std::fs::write(path, new_content)?;
    println!("{} {}", "✓".green(), path.display());

    Ok(())
}

fn schedule_binary_removal() -> Result<()> {
    #[cfg(windows)]
    {
        use std::process::Command;

        let install_dir = install_dir()?;
        if !install_dir.exists() {
            return Ok(());
        }

        let dir_str = install_dir.to_string_lossy();
        let script = format!(
            "Start-Sleep -Seconds 2; Remove-Item -Recurse -Force '{}'",
            dir_str.replace('\'', "''")
        );

        Command::new("powershell")
            .args(["-NonInteractive", "-WindowStyle", "Hidden", "-Command", &script])
            .spawn()?;

        println!("{} {}", "✓".green(), t!(
            "Installation directory (will be removed in seconds)",
            "Directorio de instalación (se eliminará en segundos)"
        ));
    }

    #[cfg(not(windows))]
    {
        let exe = std::env::current_exe()?;
        if exe.exists() {
            std::fs::remove_file(&exe)?;
            println!("{} {}", "✓".green(), exe.display());
        }
    }

    Ok(())
}

fn install_dir() -> Result<std::path::PathBuf> {
    let exe = std::env::current_exe()?;
    match exe.parent() {
        Some(dir) => Ok(dir.to_path_buf()),
        None => bail!("{}", t!(
            "Could not determine the installation directory",
            "No se pudo determinar el directorio de instalación"
        )),
    }
}
