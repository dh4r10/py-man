use anyhow::Result;
#[cfg(windows)]
use crate::releases::fetch_remote_versions;

pub async fn run(filter: Option<String>) -> Result<()> {
    #[cfg(not(windows))]
    return run_linux(filter).await;

    #[cfg(windows)]
    return run_windows(filter).await;
}

#[cfg(windows)]
async fn run_windows(filter: Option<String>) -> Result<()> {
    println!("Obteniendo versiones disponibles de python.org...");
    let versions = fetch_remote_versions().await?;
    print_versions(&versions, filter);
    Ok(())
}

#[cfg(not(windows))]
async fn run_linux(filter: Option<String>) -> Result<()> {
    use crate::releases::fetch_standalone_versions;
    println!("Obteniendo versiones disponibles para Linux...");
    let versions = fetch_standalone_versions().await?;
    print_versions(&versions, filter);
    Ok(())
}

fn print_versions(versions: &[semver::Version], filter: Option<String>) {
    let filtered: Vec<_> = versions
        .iter()
        .filter(|v| {
            if let Some(ref f) = filter {
                v.to_string().starts_with(f)
            } else {
                true
            }
        })
        .take(30)
        .collect();

    if filtered.is_empty() {
        println!("No se encontraron versiones para el filtro dado.");
    } else {
        for v in filtered {
            println!("  {}", v);
        }
    }
}
