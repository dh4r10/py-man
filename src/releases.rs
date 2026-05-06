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

/// Devuelve las versiones de Python disponibles en python-build-standalone para Linux.
#[cfg(not(windows))]
pub async fn fetch_standalone_versions() -> Result<Vec<Version>> {
    let arch = match std::env::consts::ARCH {
        "x86_64"  => "x86_64",
        "aarch64" => "aarch64",
        other     => anyhow::bail!("Arquitectura no soportada: {}", other),
    };

    let suffix = format!("{}-unknown-linux-gnu-install_only.tar.gz", arch);

    let client = reqwest::Client::builder()
        .user_agent("pvm/1.0")
        .build()?;

    let mut versions = std::collections::HashSet::new();

    for page in 1..=3u32 {
        let api_url = format!(
            "https://api.github.com/repos/astral-sh/python-build-standalone/releases?per_page=5&page={}",
            page
        );

        let response = client
            .get(&api_url)
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        if !response.status().is_success() {
            break;
        }

        let releases: Vec<serde_json::Value> = response.json().await?;

        if releases.is_empty() {
            break;
        }

        for release in &releases {
            if let Some(assets) = release["assets"].as_array() {
                for asset in assets {
                    let name = asset["name"].as_str().unwrap_or("");
                    if name.starts_with("cpython-") && name.ends_with(&suffix) {
                        // cpython-3.12.13+20260504-x86_64-unknown-linux-gnu-install_only.tar.gz
                        if let Some(ver_str) = name.strip_prefix("cpython-") {
                            if let Some(plus_pos) = ver_str.find('+') {
                                if let Ok(v) = Version::parse(&ver_str[..plus_pos]) {
                                    versions.insert(v);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut result: Vec<Version> = versions.into_iter().collect();
    result.sort_by(|a, b| b.cmp(a));
    Ok(result)
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
            "https://api.github.com/repos/astral-sh/python-build-standalone/releases?per_page=5&page={}",
            page
        );

        let response = client
            .get(&api_url)
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "GitHub API devolvió HTTP {} al buscar releases.",
                response.status()
            );
        }

        let releases: Vec<serde_json::Value> = response.json().await?;

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