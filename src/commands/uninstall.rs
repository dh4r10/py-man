use anyhow::{bail, Context, Result};
use crate::{dirs, validate};

pub fn run(version: &str) -> Result<()> {
    validate::version(version)?;
    let version_path = dirs::version_dir(version)?;

    if !version_path.exists() {
        bail!("La versión {} no está instalada.", version);
    }

    let current = dirs::current_alias_dir()?;
    if current.exists() {
        let target = std::fs::read_link(&current).unwrap_or_else(|_| current.clone());
        if target == version_path {
            bail!(
                "No se puede desinstalar la versión activa ({}). \
                 Cambia primero con `pvm use <otra_version>`.",
                version
            );
        }
    }

    print!("Eliminando Python {} ... ", version);
    use std::io::Write;
    let _ = std::io::stdout().flush();

    // On Windows, deregister from the MSI product database before deleting files.
    // If we delete files first, msiexec can no longer uninstall cleanly, leaving
    // stale registry entries that cause error 1638 on reinstall.
    #[cfg(windows)]
    windows_deregister(version, &version_path);

    #[cfg(windows)]
    strip_readonly(&version_path);

    std::fs::remove_dir_all(&version_path)
        .with_context(|| format!("No se pudo eliminar el directorio de Python {}", version))?;

    println!("hecho.");
    Ok(())
}

/// Finds the MSI product GUID for the given PVM version and runs `msiexec /x`
/// to remove all MSI-managed registry entries. Also removes Python's own
/// HKCU\SOFTWARE\Python\PythonCore registry tree for this major.minor if it
/// points to our installation.
#[cfg(windows)]
fn windows_deregister(version: &str, version_path: &std::path::Path) {
    // Remove the MSI product registration (prevents error 1638 on reinstall).
    if let Some(guid) = find_msi_guid_for_path(version_path) {
        let _ = std::process::Command::new("msiexec")
            .args(["/x", &guid, "/quiet", "/norestart"])
            .output();
    }

    // Belt-and-suspenders: also clean Python's own registry key.
    let parts: Vec<&str> = version.splitn(3, '.').collect();
    if parts.len() < 2 { return; }
    let major_minor = format!("{}.{}", parts[0], parts[1]);
    let install_key = format!(r"HKCU\SOFTWARE\Python\PythonCore\{}\InstallPath", major_minor);

    let Ok(out) = std::process::Command::new("reg")
        .args(["query", &install_key])
        .output() else { return };

    let out_lower = String::from_utf8_lossy(&out.stdout).to_lowercase();
    let path_lower = version_path.to_string_lossy().to_lowercase();
    if out_lower.contains(path_lower.trim_end_matches(['\\', '/'])) {
        let core_key = format!(r"HKCU\SOFTWARE\Python\PythonCore\{}", major_minor);
        let _ = std::process::Command::new("reg")
            .args(["delete", &core_key, "/f"])
            .output();
    }
}

/// Scans the per-user MSI Uninstall registry to find the product GUID whose
/// `InstallLocation` matches `version_path`.
#[cfg(windows)]
fn find_msi_guid_for_path(version_path: &std::path::Path) -> Option<String> {
    let path_lower = version_path.to_string_lossy().to_lowercase();
    let path_lower = path_lower.trim_end_matches(['\\', '/']);

    let base = r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall";
    let list = std::process::Command::new("reg")
        .args(["query", base])
        .output()
        .ok()?;

    for key in String::from_utf8_lossy(&list.stdout).lines() {
        let key = key.trim();
        if key.is_empty() { continue; }

        let loc = std::process::Command::new("reg")
            .args(["query", key, "/v", "InstallLocation"])
            .output()
            .ok()?;

        if String::from_utf8_lossy(&loc.stdout)
            .to_lowercase()
            .contains(path_lower)
        {
            return key.rsplit('\\').next().map(|s| s.to_string());
        }
    }

    None
}

/// Recursively removes the read-only attribute from all files under `path`.
/// Ignores errors (best-effort) so a single locked file doesn't abort the walk.
#[cfg(windows)]
fn strip_readonly(path: &std::path::Path) {
    let entries = match std::fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let p = entry.path();
        if let Ok(meta) = p.metadata() {
            if meta.permissions().readonly() {
                let mut perms = meta.permissions();
                perms.set_readonly(false);
                let _ = std::fs::set_permissions(&p, perms);
            }
            if meta.is_dir() {
                strip_readonly(&p);
            }
        }
    }
}
