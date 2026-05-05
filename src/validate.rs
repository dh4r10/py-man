use anyhow::{bail, Result};
use regex::Regex;
use std::path::Path;

/// Validates that a version string strictly matches X.Y.Z (only digits).
/// This blocks path traversal, URL injection, and command injection through version args.
pub fn version(v: &str) -> Result<()> {
    let re = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap();
    if !re.is_match(v) {
        bail!(
            "Versión inválida: {:?}. El formato debe ser X.Y.Z (ej: 3.12.4)",
            v
        );
    }
    Ok(())
}

/// Verifies that `child` is contained within `parent` (defense-in-depth against path traversal).
pub fn path_within(parent: &Path, child: &Path) -> Result<()> {
    if !child.starts_with(parent) {
        bail!("Ruta fuera del directorio de pvm. Operación denegada.");
    }
    Ok(())
}
