use crate::dirs;

const PVM_LINE: &str = "pvm env | Out-String | Invoke-Expression";
const PVM_COMMENT: &str = "# pvm — Python Version Manager";

/// Called at startup. If pvm's shims are not yet on PATH:
///   - First time: automatically adds `pvm env | Out-String | Invoke-Expression`
///     to the PowerShell profile and tells the user to reload.
///   - Already in profile but session not updated: shows the one-liner to apply now.
///   - Can't write profile: shows the manual instruction.
pub fn ensure_configured() {
    if shims_in_path() {
        return;
    }

    #[cfg(windows)]
    configure_windows();
}

#[cfg(windows)]
fn configure_windows() {
    match get_powershell_profile() {
        Err(e) => {
            eprintln!(
                "\n[pvm] No se pudo leer $PROFILE: {}\n\
                 Añade manualmente a tu perfil de PowerShell:\n\n  \
                 {}\n",
                e, PVM_LINE
            );
        }
        Ok(profile_path) => {
            let profile = std::path::Path::new(&profile_path);

            // Check if pvm is already in the profile
            let already_there = profile
                .exists()
                .then(|| std::fs::read_to_string(profile).unwrap_or_default())
                .map(|c| c.contains(PVM_LINE))
                .unwrap_or(false);

            if already_there {
                // Profile has the line but it hasn't been applied to this session yet.
                eprintln!(
                    "\n[pvm] pvm está en tu perfil pero no en esta sesión.\n\
                     Ejecuta para activarlo ahora:\n\n  \
                     {}\n",
                    PVM_LINE
                );
            } else {
                // First time: write to profile automatically.
                match write_to_profile(profile) {
                    Ok(()) => {
                        println!(
                            "\n[pvm] Configurado en: {}\n\
                             Para aplicar ahora mismo sin reiniciar la terminal:\n\n  \
                             {}\n",
                            profile_path, PVM_LINE
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "\n[pvm] No se pudo escribir en $PROFILE: {}\n\
                             Añade manualmente:\n\n  \
                             {}\n",
                            e, PVM_LINE
                        );
                    }
                }
            }
        }
    }
}

fn write_to_profile(profile: &std::path::Path) -> std::io::Result<()> {
    use std::io::Write;
    if let Some(parent) = profile.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(profile)?;
    writeln!(file)?;
    writeln!(file, "{}", PVM_COMMENT)?;
    writeln!(file, "{}", PVM_LINE)?;
    Ok(())
}

fn get_powershell_profile() -> anyhow::Result<String> {
    let out = std::process::Command::new("powershell")
        .args(["-NonInteractive", "-Command", "$PROFILE"])
        .output()?;
    let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if path.is_empty() {
        anyhow::bail!("$PROFILE está vacío");
    }
    Ok(path)
}

fn shims_in_path() -> bool {
    let Ok(shims) = dirs::shims_dir() else {
        return false;
    };
    let shims_lower = shims.to_string_lossy().to_lowercase();
    let sep = if cfg!(windows) { ';' } else { ':' };
    std::env::var("PATH")
        .unwrap_or_default()
        .to_lowercase()
        .split(sep)
        .any(|p| p == shims_lower.as_str())
}
