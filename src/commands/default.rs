use anyhow::{bail, Result};
use crate::{dirs, validate};

pub fn run(version: &str) -> Result<()> {
    validate::version(version)?;
    let version_path = dirs::version_dir(version)?;

    if !version_path.exists() {
        bail!(
            "La versión {} no está instalada. Usa `pvm install {}` primero.",
            version, version
        );
    }

    let default_path = dirs::default_alias_path()?;

    if default_path.exists() || default_path.is_symlink() {
        #[cfg(windows)]
        {
            if default_path.is_dir() {
                std::fs::remove_dir(&default_path)?;
            } else {
                std::fs::remove_file(&default_path)?;
            }
        }
        #[cfg(not(windows))]
        {
            std::fs::remove_file(&default_path)?;
        }
    }

    #[cfg(windows)]
    {
        junction::create(&version_path, &default_path)
            .map_err(|e| anyhow::anyhow!("No se pudo crear junction: {}", e))?;
    }
    #[cfg(not(windows))]
    {
        std::os::unix::fs::symlink(&version_path, &default_path)?;
    }

    // Guardar la versión default en un archivo de texto para referencia
    let default_file = dirs::pvm_home()?.join("default");
    std::fs::write(&default_file, version)?;

    println!("Versión por defecto establecida: Python {}", version);
    Ok(())
}
