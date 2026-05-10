use anyhow::Result;
use crate::{dirs, t};

pub fn run() -> Result<()> {
    let versions_dir = dirs::versions_dir()?;

    if !versions_dir.exists() {
        println!("{}", t!("No versions installed.", "No hay versiones instaladas."));
        println!("  {}", t!(
            "Use `pvm install <version>` to install one.",
            "Usa `pvm install <version>` para instalar una."
        ));
        return Ok(());
    }

    let current = current_version()?;

    let mut versions: Vec<String> = std::fs::read_dir(&versions_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();

    versions.sort();

    if versions.is_empty() {
        println!("{}", t!("No versions installed.", "No hay versiones instaladas."));
        println!("  {}", t!(
            "Use `pvm install <version>` to install one.",
            "Usa `pvm install <version>` para instalar una."
        ));
        return Ok(());
    }

    println!("{}", t!("Installed versions:", "Versiones instaladas:"));
    for v in &versions {
        if Some(v.as_str()) == current.as_deref() {
            println!("  * {} {}", v, t!("(active)", "(activa)"));
        } else {
            println!("    {}", v);
        }
    }

    Ok(())
}

fn current_version() -> Result<Option<String>> {
    let current_path = dirs::current_alias_dir()?;

    if !current_path.exists() {
        return Ok(None);
    }

    let target = std::fs::read_link(&current_path).unwrap_or(current_path);
    let version = target
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string());

    Ok(version)
}
