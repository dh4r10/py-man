use anyhow::{bail, Context, Result};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

use crate::{dirs, releases, validate};

pub async fn run(version: &str) -> Result<()> {
    let version = resolve_version(version).await?;
    let version = version.as_str();

    validate::version(version)?;

    let dest = dirs::version_dir(version)?;

    if dest.exists() {
        bail!("La versión {} ya está instalada.", version);
    }

    #[cfg(windows)]
    run_windows(version, &dest).await?;

    #[cfg(not(windows))]
    run_linux(version, &dest).await?;

    Ok(())
}

async fn download_bytes(url: &str) -> Result<Vec<u8>> {
    let client = reqwest::Client::builder()
        .user_agent("pvm/1.0")
        .build()?;

    let response = client
        .get(url)
        .send()
        .await
        .context("Error iniciando la descarga")?;

    if !response.status().is_success() {
        bail!("HTTP {} al descargar {}", response.status(), url);
    }

    let total_size = response.content_length();

    let pb = ProgressBar::new(total_size.unwrap_or(0));
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
            )?
            .progress_chars("#>-"),
    );

    let mut bytes: Vec<u8> = Vec::new();
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("Error leyendo chunk de descarga")?;
        bytes.extend_from_slice(&chunk);
        pb.inc(chunk.len() as u64);
    }

    pb.finish_with_message("Descarga completa");

    Ok(bytes)
}

#[cfg(windows)]
async fn run_windows(version: &str, dest: &Path) -> Result<()> {
    let url = releases::installer_url(version);

    println!("Descargando Python {} ...", version);

    let bytes = download_bytes(&url).await?;

    println!("Extrayendo Python {} ...", version);

    install_nuget_package(&bytes, dest)?;

    let python = find_python_exe(dest)
        .context("No se encontró python.exe en el paquete NuGet")?;

    println!("Configurando pip ...");

    ensurepip(&python)?;

    println!("Python {} instalado correctamente.", version);
    println!("Usa `pvm use {}` para activarlo.", version);

    Ok(())
}

#[cfg(not(windows))]
async fn run_linux(version: &str, dest: &Path) -> Result<()> {
    println!("Buscando Python {} para Linux ...", version);

    let url = releases::standalone_url(version).await?;

    println!("Descargando Python {} ...", version);

    let bytes = download_bytes(&url).await?;

    println!("Extrayendo Python {} ...", version);

    extract_tarball(&bytes, dest)?;

    println!("Python {} instalado correctamente.", version);
    println!("Usa `pvm use {}` para activarlo.", version);

    Ok(())
}

#[cfg(windows)]
fn install_nuget_package(zip_bytes: &[u8], dest: &Path) -> Result<()> {
    use std::fs::File;
    use std::io::Cursor;
    use zip::ZipArchive;

    std::fs::create_dir_all(dest)?;

    let reader = Cursor::new(zip_bytes);
    let mut archive = ZipArchive::new(reader)
        .context("No se pudo abrir el paquete NuGet")?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = dest.join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

/// Extrae el tarball de python-build-standalone eliminando el componente raíz `python/`.
/// El archivo tiene la estructura `python/bin/python3`, `python/lib/`, etc.
/// Tras la extracción queda `dest/bin/python3`, `dest/lib/`, etc.
#[cfg(not(windows))]
fn extract_tarball(bytes: &[u8], dest: &Path) -> Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    std::fs::create_dir_all(dest)?;

    let gz = GzDecoder::new(bytes as &[u8]);
    let mut archive = Archive::new(gz);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let raw_path = entry.path()?.into_owned();

        // Elimina el componente raíz `python/` que incluye python-build-standalone.
        let stripped: std::path::PathBuf = raw_path.components().skip(1).collect();
        if stripped.as_os_str().is_empty() {
            continue;
        }

        let outpath = dest.join(&stripped);

        if entry.header().entry_type().is_dir() {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            entry.unpack(&outpath)
                .with_context(|| format!("No se pudo extraer {:?}", outpath))?;
        }
    }

    Ok(())
}

#[cfg(windows)]
fn find_python_exe(base: &Path) -> Result<std::path::PathBuf> {
    let candidates = [
        base.join("tools").join("python.exe"),
        base.join("python.exe"),
    ];

    for candidate in candidates {
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    bail!("No se encontró python.exe")
}

#[cfg(windows)]
fn ensurepip(python: &Path) -> Result<()> {
    let status = std::process::Command::new(python)
        .args(["-m", "ensurepip", "--default-pip"])
        .status()
        .context("Error ejecutando ensurepip")?;

    if !status.success() {
        bail!("ensurepip falló");
    }

    Ok(())
}

async fn resolve_version(version: &str) -> Result<String> {
    let minor_re = regex::Regex::new(r"^\d{1,3}\.\d{1,3}$").unwrap();
    if !minor_re.is_match(version) {
        return Ok(version.to_string());
    }

    #[cfg(windows)]
    {
        let versions = crate::releases::fetch_remote_versions().await?;
        let latest = versions
            .iter()
            .filter(|v| v.pre.is_empty())
            .find(|v| format!("{}.{}", v.major, v.minor) == version)
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow::anyhow!("No se encontró ninguna versión estable para {}", version))?;
        println!("Resolviendo {} → {}", version, latest);
        return Ok(latest);
    }

    #[cfg(not(windows))]
    {
        let versions = crate::releases::fetch_standalone_versions().await?;
        let latest = versions
            .iter()
            .filter(|v| v.pre.is_empty())
            .find(|v| format!("{}.{}", v.major, v.minor) == version)
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow::anyhow!("No se encontró ninguna versión estable para {}", version))?;
        println!("Resolviendo {} → {}", version, latest);
        return Ok(latest);
    }
}
