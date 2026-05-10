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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // --- version() ---

    #[test]
    fn version_valid_formats() {
        assert!(version("3.12.4").is_ok());
        assert!(version("3.9.0").is_ok());
        assert!(version("10.100.200").is_ok());
        assert!(version("0.0.0").is_ok());
    }

    #[test]
    fn version_rejects_incomplete() {
        assert!(version("3.12").is_err());
        assert!(version("3").is_err());
        assert!(version("").is_err());
        assert!(version("3.12.").is_err());
        assert!(version(".12.4").is_err());
    }

    #[test]
    fn version_rejects_non_digits() {
        assert!(version("3.12.4a").is_err());
        assert!(version("3.12.alpha").is_err());
        assert!(version("v3.12.4").is_err());
        assert!(version("3.12.4-rc1").is_err());
    }

    #[test]
    fn version_rejects_path_traversal() {
        assert!(version("../etc/passwd").is_err());
        assert!(version("3.12.4/../../etc").is_err());
        assert!(version("3.12.4;rm -rf /").is_err());
    }

    #[test]
    fn version_rejects_extra_segments() {
        assert!(version("3.12.4.5").is_err());
    }

    // --- path_within() ---

    #[test]
    fn path_within_allows_direct_child() {
        let parent = PathBuf::from("/home/user/.pvm/versions");
        let child = parent.join("3.12.4");
        assert!(path_within(&parent, &child).is_ok());
    }

    #[test]
    fn path_within_allows_nested_child() {
        let parent = PathBuf::from("/home/user/.pvm/versions");
        let child = parent.join("3.12.4").join("bin");
        assert!(path_within(&parent, &child).is_ok());
    }

    #[test]
    fn path_within_allows_same_path() {
        let parent = PathBuf::from("/home/user/.pvm/versions");
        assert!(path_within(&parent, &parent).is_ok());
    }

    #[test]
    fn path_within_rejects_sibling_dir() {
        let parent = PathBuf::from("/home/user/.pvm/versions");
        let sibling = PathBuf::from("/home/user/.pvm/aliases");
        assert!(path_within(&parent, &sibling).is_err());
    }

    #[test]
    fn path_within_rejects_parent_escape() {
        let parent = PathBuf::from("/home/user/.pvm/versions");
        let escaped = PathBuf::from("/home/user/.pvm");
        assert!(path_within(&parent, &escaped).is_err());
    }

    #[test]
    fn path_within_rejects_unrelated_path() {
        let parent = PathBuf::from("/home/user/.pvm/versions");
        let unrelated = PathBuf::from("/tmp/evil");
        assert!(path_within(&parent, &unrelated).is_err());
    }
}
