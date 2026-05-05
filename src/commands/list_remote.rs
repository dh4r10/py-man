use anyhow::Result;
use crate::releases::fetch_remote_versions;

pub async fn run(filter: Option<String>) -> Result<()> {
    println!("Obteniendo versiones disponibles de python.org...");
    let versions = fetch_remote_versions().await?;

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

    Ok(())
}
