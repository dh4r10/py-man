use anyhow::{bail, Context, Result};
use crate::{dirs, validate, t};

pub fn run(version: &str) -> Result<()> {
    validate::version(version)?;

    let version_path = dirs::version_dir(version)?;

    if !version_path.exists() {
        bail!(
            "{}",
            t!(
                "Version {} is not installed. Use `pvm install {}` first.",
                "La versión {} no está instalada. Usa `pvm install {}` primero.",
                version,
                version
            )
        );
    }

    let current = dirs::current_alias_dir()?;

    if current.exists() || current.is_symlink() {
        #[cfg(windows)]
        {
            if current.is_dir() {
                std::fs::remove_dir(&current)?;
            } else {
                std::fs::remove_file(&current)?;
            }
        }

        #[cfg(not(windows))]
        {
            std::fs::remove_file(&current)?;
        }
    }

    create_junction_or_symlink(&version_path, &current)?;
    refresh_bin()?;

    println!("{}", t!("Now using Python {}", "Usando Python {}", version));

    Ok(())
}

fn create_junction_or_symlink(
    target: &std::path::Path,
    link: &std::path::Path,
) -> Result<()> {
    #[cfg(windows)]
    {
        junction::create(target, link)
            .map_err(|e| anyhow::anyhow!("Could not create junction: {}", e))?;
    }

    #[cfg(not(windows))]
    {
        std::os::unix::fs::symlink(target, link)?;
    }

    Ok(())
}

/// Installs shims in ~/.pvm/bin/ by copying the pvm binary itself with different names.
/// When invoked as python / pip, the binary detects its own name and acts as a shim:
/// it resolves aliases/current and launches the real Python for that version.
/// This way, sys.executable in the child process points to the exact version directory,
/// ensuring pyvenv.cfg is pinned to that version and venvs are isolated.
fn refresh_bin() -> Result<()> {
    let bin = dirs::bin_dir()?;

    // Remove old junction if present (previous architecture used a junction in bin/).
    #[cfg(windows)]
    let _ = junction::delete(&bin);

    match std::fs::remove_dir(&bin) {
        Ok(()) => {}
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(ref e) if e.raw_os_error() == Some(145) => {} // Windows: ERROR_DIR_NOT_EMPTY
        Err(ref e) if e.raw_os_error() == Some(39) => {}  // Linux: ENOTEMPTY
        Err(ref e) if e.raw_os_error() == Some(66) => {}  // macOS: ENOTEMPTY
        Err(e) => bail!("{}", t!("Could not clean bin/: {}", "No se pudo limpiar bin/: {}", e)),
    }

    std::fs::create_dir_all(&bin)?;

    let pvm_exe = std::env::current_exe()
        .context("Could not determine the pvm executable path")?;

    #[cfg(windows)]
    let shim_names: &[&str] = &["python.exe", "pythonw.exe", "pip.exe", "pip3.exe"];
    #[cfg(not(windows))]
    let shim_names: &[&str] = &["python", "python3", "pip", "pip3"];

    for name in shim_names {
        let dst = bin.join(name);
        let _ = std::fs::remove_file(&dst);
        std::fs::copy(&pvm_exe, &dst)
            .with_context(|| t!("Could not create shim {}", "No se pudo crear shim {}", name))?;
    }

    Ok(())
}
