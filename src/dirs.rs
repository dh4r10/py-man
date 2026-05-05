use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn pvm_home() -> Result<PathBuf> {
    let home = dirs::home_dir().context("No se pudo encontrar el directorio home")?;
    Ok(home.join(".pvm"))
}

pub fn versions_dir() -> Result<PathBuf> {
    Ok(pvm_home()?.join("versions"))
}

pub fn aliases_dir() -> Result<PathBuf> {
    Ok(pvm_home()?.join("aliases"))
}

pub fn current_alias_dir() -> Result<PathBuf> {
    Ok(aliases_dir()?.join("current"))
}

pub fn default_alias_path() -> Result<PathBuf> {
    Ok(aliases_dir()?.join("default"))
}

/// Builds the path for a specific version and verifies it stays within versions_dir.
pub fn version_dir(version: &str) -> Result<PathBuf> {
    let base = versions_dir()?;
    let target = base.join(version);
    crate::validate::path_within(&base, &target)?;
    Ok(target)
}


pub fn bin_dir() -> Result<PathBuf> {
    Ok(pvm_home()?.join("bin"))
}

pub fn ensure_dirs() -> Result<()> {
    std::fs::create_dir_all(versions_dir()?)?;
    std::fs::create_dir_all(aliases_dir()?)?;
    std::fs::create_dir_all(bin_dir()?)?;
    Ok(())
}
