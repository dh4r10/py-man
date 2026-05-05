use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use crate::{dirs, validate};

pub fn run(dir: &str, extra_args: &[String], version_override: Option<&str>) -> Result<()> {
    let python = match version_override {
        Some(v) => {
            println!("Creando venv con Python {} ...", v);
            resolve_python_for_version(v)?
        }
        None => resolve_pinned_python()?,
    };

    let mut cmd_args: Vec<&str> = vec!["-m", "venv", dir];
    let extras: Vec<&str> = extra_args.iter().map(|s| s.as_str()).collect();
    cmd_args.extend(extras);

    let status = std::process::Command::new(&python)
        .args(&cmd_args)
        .status()
        .context("Error ejecutando python -m venv")?;

    if !status.success() {
        bail!("Error al crear el entorno virtual (código {:?}).", status.code());
    }

    Ok(())
}

/// Resuelve el Python de una versión instalada específica.
fn resolve_python_for_version(version: &str) -> Result<PathBuf> {
    validate::version(version)?;

    let version_dir = dirs::version_dir(version)?;

    if !version_dir.exists() {
        bail!(
            "La versión {} no está instalada. Usa `pvm install {}` primero.",
            version, version
        );
    }

    #[cfg(windows)]
    let python = version_dir.join("tools").join("python.exe");
    #[cfg(not(windows))]
    let python = version_dir.join("bin").join("python3");

    if !python.exists() {
        bail!("No se encontró el ejecutable de Python en la versión {}.", version);
    }

    Ok(python)
}

/// Resuelve el Python real de la versión activa a través del alias current.
/// El venv resultante queda anclado a la versión exacta (pyvenv.cfg con ruta real).
fn resolve_pinned_python() -> Result<PathBuf> {
    let current = dirs::current_alias_dir()?;
    if !current.exists() {
        bail!("No hay ninguna versión activa. Usa `pvm use <version>` primero.");
    }

    let resolved = std::fs::canonicalize(&current)
        .context("No se pudo resolver la ruta de la versión activa")?;

    #[cfg(windows)]
    let version_dir = {
        let s = resolved.to_string_lossy();
        PathBuf::from(s.trim_start_matches(r"\\?\"))
    };
    #[cfg(not(windows))]
    let version_dir = resolved;

    #[cfg(windows)]
    let python = version_dir.join("tools").join("python.exe");
    #[cfg(not(windows))]
    let python = version_dir.join("bin").join("python3");

    if !python.exists() {
        bail!("No se encontró el ejecutable de Python en la versión activa.");
    }

    Ok(python)
}
