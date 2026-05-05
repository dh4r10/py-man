use anyhow::Result;
use regex::Regex;
use semver::Version;

const PYTHON_FTP: &str = "https://www.python.org/ftp/python/";
const NUGET_BASE: &str = "https://www.nuget.org/api/v2/package/python/";

pub async fn fetch_remote_versions() -> Result<Vec<Version>> {
    let body = reqwest::get(PYTHON_FTP).await?.text().await?;

    let re = Regex::new(r#"href="(\d+\.\d+\.\d+)/""#)?;

    let mut versions: Vec<Version> = re
        .captures_iter(&body)
        .filter_map(|cap| Version::parse(&cap[1]).ok())
        .collect();

    versions.sort_by(|a, b| b.cmp(a));

    Ok(versions)
}

/// Devuelve la URL del paquete NuGet oficial de Python.
///
/// Ejemplo:
/// https://www.nuget.org/api/v2/package/python/3.13.7
pub fn installer_url(version: &str) -> String {
    format!("{}{}/", NUGET_BASE, version)
}