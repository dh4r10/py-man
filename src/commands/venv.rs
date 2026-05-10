use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use crate::{dirs, validate, t};

pub fn run(dir: &str, extra_args: &[String], version_override: Option<&str>) -> Result<()> {
    let python = match version_override {
        Some(v) => {
            println!("{}", t!("Creating venv with Python {} ...", "Creando venv con Python {} ...", v));
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
        .context("Error running python -m venv")?;

    if !status.success() {
        bail!("{}", t!(
            "Error creating the virtual environment (exit code {:?}).",
            "Error al crear el entorno virtual (código {:?}).",
            status.code()
        ));
    }

    Ok(())
}

fn resolve_python_for_version(version: &str) -> Result<PathBuf> {
    validate::version(version)?;

    let version_dir = dirs::version_dir(version)?;

    if !version_dir.exists() {
        bail!(
            "{}",
            t!(
                "Version {} is not installed. Use `pvm install {}` first.",
                "La versión {} no está instalada. Usa `pvm install {}` primero.",
                version, version
            )
        );
    }

    #[cfg(windows)]
    let python = version_dir.join("tools").join("python.exe");
    #[cfg(not(windows))]
    let python = version_dir.join("bin").join("python3");

    if !python.exists() {
        bail!("{}", t!(
            "Python executable not found for version {}.",
            "No se encontró el ejecutable de Python en la versión {}.",
            version
        ));
    }

    Ok(python)
}

fn resolve_pinned_python() -> Result<PathBuf> {
    let current = dirs::current_alias_dir()?;
    if !current.exists() {
        bail!("{}", t!(
            "No active version. Use `pvm use <version>` first.",
            "No hay ninguna versión activa. Usa `pvm use <version>` primero."
        ));
    }

    let resolved = std::fs::canonicalize(&current)
        .context("Could not resolve the active version path")?;

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
        bail!("{}", t!(
            "Python executable not found for the active version.",
            "No se encontró el ejecutable de Python en la versión activa."
        ));
    }

    Ok(python)
}
