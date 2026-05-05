use anyhow::{Context, Result};
use regex::Regex;

/// Downloads the official SHA256 checksum file from python.org FTP for a given installer URL.
/// Returns None if the checksum file is not available (older releases).
pub async fn fetch_expected(installer_url: &str) -> Result<Option<String>> {
    let sha256_url = format!("{}.sha256", installer_url);

    let response = reqwest::get(&sha256_url)
        .await
        .context("Error al conectar con python.org para obtener el checksum")?;

    if !response.status().is_success() {
        return Ok(None);
    }

    let content = response
        .text()
        .await
        .context("Error al leer el archivo de checksum")?;

    let re = Regex::new(r"[a-f0-9]{64}").unwrap();
    Ok(re.find(&content).map(|m| m.as_str().to_string()))
}
