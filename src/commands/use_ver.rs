use anyhow::{bail, Context, Result};
use crate::{dirs, validate};

pub fn run(version: &str) -> Result<()> {
    validate::version(version)?;

    let version_path = dirs::version_dir(version)?;

    if !version_path.exists() {
        bail!(
            "La versión {} no está instalada. Usa `pvm install {}` primero.",
            version,
            version
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

    println!("Usando Python {}", version);

    Ok(())
}

fn create_junction_or_symlink(
    target: &std::path::Path,
    link: &std::path::Path,
) -> Result<()> {
    #[cfg(windows)]
    {
        junction::create(target, link)
            .map_err(|e| anyhow::anyhow!("No se pudo crear junction: {}", e))?;
    }

    #[cfg(not(windows))]
    {
        std::os::unix::fs::symlink(target, link)?;
    }

    Ok(())
}

/// Instala shims en ~/.pvm/bin/ copiando el propio binario pvm.exe con distintos nombres.
/// Cuando son invocados como python.exe / pip.exe, el binario detecta su nombre y actúa
/// como shim: resuelve aliases/current y lanza el Python real de esa versión.
/// Así sys.executable del proceso hijo apunta al directorio de versión exacto, lo que
/// garantiza que pyvenv.cfg quede fijado a esa versión y los venvs sean aislados.
fn refresh_bin() -> Result<()> {
    let bin = dirs::bin_dir()?;

    // Romper junction anterior si existe (arquitectura previa usaba junction en bin/).
    // junction::delete usa FILE_FLAG_OPEN_REPARSE_POINT — funciona aunque el target no exista.
    // Errores ignorados: puede que bin no sea un junction o no exista todavía.
    #[cfg(windows)]
    let _ = junction::delete(&bin);

    // remove_dir elimina el directorio vacío resultante o el directorio vacío de ensure_dirs.
    // "Not empty" (tiene shims anteriores) y NotFound son ambos aceptables — se ignoran.
    match std::fs::remove_dir(&bin) {
        Ok(()) => {}
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(ref e) if e.raw_os_error() == Some(145) => {} // Windows: ERROR_DIR_NOT_EMPTY
        Err(ref e) if e.raw_os_error() == Some(66) => {}  // Unix: ENOTEMPTY
        Err(e) => bail!("No se pudo limpiar bin/: {}", e),
    }

    std::fs::create_dir_all(&bin)?;

    // Copiar pvm.exe con los nombres de shim. El binario detecta su propio nombre
    // en tiempo de ejecución y actúa como shim en lugar de como CLI pvm.
    let pvm_exe = std::env::current_exe()
        .context("No se pudo determinar la ruta del ejecutable pvm")?;

    for name in &["python.exe", "pythonw.exe", "pip.exe", "pip3.exe"] {
        let dst = bin.join(name);
        let _ = std::fs::remove_file(&dst);
        std::fs::copy(&pvm_exe, &dst)
            .with_context(|| format!("No se pudo crear shim {}", name))?;
    }

    Ok(())
}
