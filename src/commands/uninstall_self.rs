use anyhow::{bail, Result};
use colored::Colorize;
use std::io::{self, Write};

pub fn run() -> Result<()> {
    println!("{}", "Desinstalador de PVM".bold().magenta());
    println!();
    println!("Esto eliminará:");
    println!("  • ~/.pvm/          (versiones, aliases, shims)");
    println!("  • PATH del usuario (entrada de PVM)");
    println!("  • Perfil de PowerShell (línea pvm env)");
    println!("  • Directorio de instalación de pvm.exe");
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
    println!("Abre una nueva terminal para que los cambios en PATH surtan efecto.");

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
    println!("{} PATH (no aplica en esta plataforma)", "·".dimmed());

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
    println!("{} Perfil (no aplica en esta plataforma)", "·".dimmed());

    Ok(())
}

fn schedule_binary_removal() -> Result<()> {
    let install_dir = install_dir()?;
    if !install_dir.exists() {
        return Ok(());
    }

    #[cfg(windows)]
    {
        use std::process::Command;

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
        std::fs::remove_dir_all(&install_dir)?;
        println!("{} Directorio de instalación", "✓".green());
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
