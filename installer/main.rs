// pvm-setup — instalador nativo de PVM para Windows.
// Embebe pvm.exe en tiempo de compilación (build.rs) y lo instala en:
//   %LOCALAPPDATA%\pvm\pvm.exe
// Añade esa ruta al PATH del usuario (permanente, sin admin).
// Opcionalmente configura el perfil de PowerShell.

include!(concat!(env!("OUT_DIR"), "/pvm_bytes.rs"));

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("PVM — Python Version Manager  v{VERSION}");
    println!("{}", "=".repeat(44));
    println!();

    if PVM_EXE.is_empty() {
        eprintln!("Error: el instalador fue compilado sin pvm.exe embebido.");
        eprintln!();
        eprintln!("Pasos para generar el instalador correctamente:");
        eprintln!("  1. cargo build --release --bin pvm");
        eprintln!("  2. cargo build --release --bin pvm-setup");
        std::process::exit(1);
    }

    #[cfg(not(windows))]
    {
        eprintln!("Este instalador solo funciona en Windows.");
        std::process::exit(1);
    }

    #[cfg(windows)]
    run_install();
}

#[cfg(windows)]
fn run_install() {
    let install_dir = get_install_dir();
    let pvm_exe_dst = install_dir.join("pvm.exe");

    println!("Directorio de instalación:");
    println!("  {}", install_dir.display());
    println!();

    // Comprobar si ya está instalado
    if pvm_exe_dst.exists() {
        println!("PVM ya está instalado. ¿Actualizar? [S/n]: ");
        if !confirm() {
            println!("Instalación cancelada.");
            return;
        }
    } else {
        println!("¿Instalar PVM? [S/n]: ");
        if !confirm() {
            println!("Instalación cancelada.");
            return;
        }
    }

    println!();

    // 1. Crear directorio e instalar pvm.exe
    step("Instalando pvm.exe", || {
        std::fs::create_dir_all(&install_dir)?;
        std::fs::write(&pvm_exe_dst, PVM_EXE)?;
        Ok(())
    });

    // 2. Añadir al PATH del usuario (permanente)
    let already_in_path = is_in_path(&install_dir.to_string_lossy());
    if already_in_path {
        println!("  PATH  ya contiene el directorio — sin cambios.");
    } else {
        step("Añadiendo al PATH del usuario", || add_to_path(&install_dir.to_string_lossy()));
    }

    // 3. Perfil de PowerShell
    println!();
    println!("¿Configurar el perfil de PowerShell automáticamente?");
    println!("  (añade: pvm env | Out-String | Invoke-Expression)");
    print!("[S/n]: ");
    if confirm() {
        step("Configurando perfil de PowerShell", setup_powershell_profile);
    } else {
        println!();
        println!("  Añade manualmente a tu perfil (notepad $PROFILE):");
        println!();
        println!("    pvm env | Out-String | Invoke-Expression");
    }

    // Resumen final
    println!();
    println!("{}", "=".repeat(44));
    println!("  PVM instalado correctamente.");
    println!("{}", "=".repeat(44));
    println!();
    println!("Pasos siguientes:");
    println!();
    println!("  1. Abre una nueva terminal PowerShell");
    println!("  2. Instala una versión de Python:");
    println!("       pvm install 3.12.4");
    println!("  3. Actívala:");
    println!("       pvm use 3.12.4");
    println!("  4. Verifica:");
    println!("       python -V");
    println!();

    pause();
}

// ── Utilidades ──────────────────────────────────────────────────────────────

fn get_install_dir() -> std::path::PathBuf {
    let local_app_data = std::env::var("LOCALAPPDATA")
        .unwrap_or_else(|_| {
            let home = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\User".into());
            format!("{}\\AppData\\Local", home)
        });
    std::path::PathBuf::from(local_app_data).join("pvm")
}

fn confirm() -> bool {
    use std::io::{self, Write};
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let trimmed = input.trim().to_lowercase();
    trimmed.is_empty() || trimmed == "s" || trimmed == "y" || trimmed == "si" || trimmed == "yes"
}

fn step<F: FnOnce() -> Result<(), Box<dyn std::error::Error>>>(label: &str, f: F) {
    print!("  {label} ... ");
    let _ = std::io::Write::flush(&mut std::io::stdout());
    match f() {
        Ok(()) => println!("listo."),
        Err(e) => {
            println!("ERROR");
            eprintln!("    {e}");
            std::process::exit(1);
        }
    }
}

fn is_in_path(dir: &str) -> bool {
    let output = std::process::Command::new("powershell")
        .args([
            "-NonInteractive",
            "-Command",
            &format!(
                r#"$p = [Environment]::GetEnvironmentVariable('PATH','User'); \
                   if ($p -like '*{dir}*') {{ 'yes' }} else {{ 'no' }}"#
            ),
        ])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_lowercase())
        .unwrap_or_default();
    output.contains("yes")
}

fn add_to_path(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let script = format!(
        r#"
        $dir = '{dir}'
        $old = [Environment]::GetEnvironmentVariable('PATH', 'User')
        if ($old -notlike "*$dir*") {{
            [Environment]::SetEnvironmentVariable('PATH', "$dir;$old", 'User')
        }}
        "#
    );
    let status = std::process::Command::new("powershell")
        .args(["-NonInteractive", "-Command", &script])
        .status()?;
    if !status.success() {
        return Err("PowerShell devolvió error al modificar PATH".into());
    }
    Ok(())
}

fn setup_powershell_profile() -> Result<(), Box<dyn std::error::Error>> {
    const LINE: &str = "pvm env | Out-String | Invoke-Expression";
    const COMMENT: &str = "# pvm — Python Version Manager";

    let script = format!(
        r#"
        $line    = '{LINE}'
        $comment = '{COMMENT}'
        $profile_path = $PROFILE

        # Crear directorio del perfil si no existe
        $dir = Split-Path $profile_path
        if (-not (Test-Path $dir)) {{ New-Item -ItemType Directory -Force -Path $dir | Out-Null }}

        # Comprobar si ya está
        $content = if (Test-Path $profile_path) {{ Get-Content $profile_path -Raw }} else {{ '' }}
        if ($content -notlike "*$line*") {{
            Add-Content -Path $profile_path -Value "`n$comment`n$line"
        }}
        "#
    );
    let status = std::process::Command::new("powershell")
        .args(["-NonInteractive", "-Command", &script])
        .status()?;
    if !status.success() {
        return Err("PowerShell devolvió error al modificar el perfil".into());
    }
    Ok(())
}

fn pause() {
    print!("Presiona Enter para salir...");
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
}
