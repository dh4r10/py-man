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

/// Devuelve la URL del paquete NuGet oficial de Python (Windows).
pub fn installer_url(version: &str) -> String {
    format!("{}{}/", NUGET_BASE, version)
}

/// Busca en las releases de python-build-standalone (astral-sh) la URL de descarga
/// del tarball `install_only` para la versión y arquitectura actuales (Linux).
#[cfg(not(windows))]
pub async fn standalone_url(version: &str) -> Result<String> {
    let arch = match std::env::consts::ARCH {
        "x86_64"  => "x86_64",
        "aarch64" => "aarch64",
        other     => anyhow::bail!("Arquitectura no soportada: {}", other),
    };

    let prefix = format!("cpython-{}+", version);
    let suffix = format!("{}-unknown-linux-gnu-install_only.tar.gz", arch);

    let client = reqwest::Client::builder()
        .user_agent("pvm/1.0")
        .build()?;

    for page in 1..=10u32 {
        let api_url = format!(
            "https://api.github.com/repos/astral-sh/python-build-standalone/releases?per_page=50&page={}",
            page
        );

        let releases: Vec<serde_json::Value> = client
            .get(&api_url)
            .send()
            .await?
            .json()
            .await?;

        if releases.is_empty() {
            break;
        }

        for release in &releases {
            if let Some(assets) = release["assets"].as_array() {
                for asset in assets {
                    let name = asset["name"].as_str().unwrap_or("");
                    if name.starts_with(&prefix) && name.ends_with(&suffix) {
                        if let Some(url) = asset["browser_download_url"].as_str() {
                            return Ok(url.to_string());
                        }
                    }
                }
            }
        }
    }

    anyhow::bail!(
        "No se encontró Python {} para Linux ({}).\n\
         Versiones disponibles en: https://github.com/astral-sh/python-build-standalone/releases",
        version,
        arch
    )
}