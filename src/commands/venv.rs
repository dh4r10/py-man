use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use crate::dirs;

pub fn run(dir: &str, extra_args: &[String]) -> Result<()> {
    let python = resolve_pinned_python()?;

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

/// Resolves the current/ junction to the real version directory so the resulting
/// venv's pyvenv.cfg has `home = ~/.pvm/versions/X.Y.Z` (pinned), not the
/// mutable current/ alias that changes every time `pvm use` is called.
fn resolve_pinned_python() -> Result<PathBuf> {
    let current = dirs::current_alias_dir()?;
    if !current.exists() {
        bail!("No hay ninguna versión activa. Usa `pvm use <version>` primero.");
    }

    // canonicalize() follows NTFS junctions on Windows, giving the real version path.
    let resolved = std::fs::canonicalize(&current)
        .context("No se pudo resolver la ruta de la versión activa")?;

    #[cfg(windows)]
    let version_dir = {
        // canonicalize() prepends \\?\ on Windows — strip it for normal usage
        let s = resolved.to_string_lossy();
        PathBuf::from(s.trim_start_matches(r"\\?\"))
    };
    #[cfg(not(windows))]
    let version_dir = resolved;

    #[cfg(windows)]
    let python = version_dir
        .join("tools")
        .join("python.exe");
    #[cfg(not(windows))]
    let python = version_dir.join("bin").join("python3");

    if !python.exists() {
        bail!("No se encontró el ejecutable de Python en la versión activa.");
    }

    Ok(python)
}
