use anyhow::{bail, Context, Result};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use crate::{dirs, releases, validate};

pub async fn run(version: &str) -> Result<()> {
    validate::version(version)?;

    let dest = dirs::version_dir(version)?;

    if dest.exists() {
        bail!("La versión {} ya está instalada.", version);
    }

    let url = releases::installer_url(version);

    println!("Descargando Python {} ...", version);

    let response = reqwest::get(&url)
        .await
        .context("Error descargando Python")?;

    if !response.status().is_success() {
        bail!(
            "No se pudo descargar Python {}. HTTP {}.",
            version,
            response.status()
        );
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

    println!("Extrayendo Python {} ...", version);

    install_nuget_package(&bytes, &dest)?;

    let python = find_python_exe(&dest)
        .context("No se encontró python.exe en el paquete NuGet")?;

    println!("Configurando pip ...");

    ensurepip(&python)?;

    println!("Python {} instalado correctamente.", version);
    println!("Usa `pvm use {}` para activarlo.", version);

    Ok(())
}

#[cfg(windows)]
fn install_nuget_package(
    zip_bytes: &[u8],
    dest: &Path,
) -> Result<()> {
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

#[cfg(not(windows))]
fn install_nuget_package(
    _zip_bytes: &[u8],
    _dest: &Path,
) -> Result<()> {
    bail!("Linux/macOS aún no implementado.")
}

/// Busca python.exe dentro del paquete NuGet.
///
/// Normalmente está en:
///
/// tools/python.exe
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

/// Ejecuta:
///
/// python -m ensurepip
///
/// para instalar pip automáticamente.
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