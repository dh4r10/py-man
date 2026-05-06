use anyhow::{bail, Result};
use colored::Colorize;
use std::io::{self, Write};

pub fn run() -> Result<()> {
    println!("{}", "Desinstalador de PVM".bold().magenta());
    println!();
    println!("Esto eliminará:");
    println!("  • ~/.pvm/          (versiones, aliases, shims)");

    #[cfg(windows)]
    {
        println!("  • PATH del usuario (entrada de PVM)");
        println!("  • Perfil de PowerShell (línea pvm env)");
        println!("  • Directorio de instalación de pvm.exe");
    }

    #[cfg(not(windows))]
    {
        println!("  • ~/.local/bin/pvm (binario)");
        println!("  • Líneas de pvm en .bashrc / .zshrc / config.fish");
    }

    println!();

    print!("¿Confirmar desinstalación? [s/N]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if !matches!(input.trim().to_lowercase().as_str(), "s" | "si" | "sí" | "y" | "yes") {
        println!("Cancelado.");
        return Ok(());
    }

    println!();

    remove_pvm_data()?;
    remove_from_path()?;
    remove_from_profile()?;
    schedule_binary_removal()?;

    println!();
    println!("{}", "PVM desinstalado correctamente.".bold().green());
    println!("Abre una nueva terminal para que los cambios surtan efecto.");

    Ok(())
}

fn remove_pvm_data() -> Result<()> {
    let pvm_home = crate::dirs::pvm_home()?;
    if pvm_home.exists() {
        std::fs::remove_dir_all(&pvm_home)?;
        println!("{} ~/.pvm/", "✓".green());
    } else {
        println!("{} ~/.pvm/ (ya no existía)", "·".dimmed());
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
            println!("{} PATH del usuario", "✓".green());
        } else {
            eprintln!("{} No se pudo limpiar el PATH", "!".yellow());
        }
    }

    #[cfg(not(windows))]
    {
        // En Linux el PATH se gestiona desde los profiles — se limpia en remove_from_profile()
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
            println!("{} Perfil de PowerShell", "✓".green());
        } else {
            eprintln!("{} No se pudo limpiar el perfil de PowerShell", "!".yellow());
        }
    }

    #[cfg(not(windows))]
    {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No se encontró el directorio home"))?;

        let profiles = [
            home.join(".bashrc"),
            home.join(".zshrc"),
        ];

        for profile in &profiles {
            if profile.exists() {
                if let Err(e) = clean_shell_profile(profile) {
                    eprintln!("{} No se pudo limpiar {}: {}", "!".yellow(), profile.display(), e);
                }
            }
        }

        let fish = home.join(".config/fish/config.fish");
        if fish.exists() {
            if let Err(e) = clean_shell_profile(&fish) {
                eprintln!("{} No se pudo limpiar {}: {}", "!".yellow(), fish.display(), e);
            }
        }
    }

    Ok(())
}

/// Elimina las líneas añadidas por pvm del profile dado.
/// Elimina: líneas con "pvm", y la línea "rehash" que siga inmediatamente a una de ellas.
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
            || (line.contains("export PATH") && line.contains(".local/bin") && line.contains("pvm"));

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

    // Eliminar líneas vacías consecutivas al final
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

        println!("{} Directorio de instalación (se eliminará en segundos)", "✓".green());
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
        None => bail!("No se pudo determinar el directorio de instalación"),
    }
}
