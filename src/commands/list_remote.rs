use anyhow::Result;
use colored::Colorize;
use semver::Version;
use std::collections::BTreeMap;
use std::io::{IsTerminal, Write};

#[cfg(windows)]
use crate::releases::fetch_remote_versions;

const MIN_MINOR: u64 = 8;
const CMD: (u8, u8, u8) = (200, 160, 255);
const DIM: (u8, u8, u8) = (110, 90, 140);

pub async fn run(filter: Option<String>) -> Result<()> {
    #[cfg(not(windows))]
    return run_linux(filter).await;

    #[cfg(windows)]
    return run_windows(filter).await;
}

#[cfg(windows)]
async fn run_windows(filter: Option<String>) -> Result<()> {
    println!(
        "{}",
        "Obteniendo versiones disponibles de python.org..."
            .truecolor(DIM.0, DIM.1, DIM.2)
    );
    let versions = fetch_remote_versions().await?;
    let installed = installed_versions();
    let active = active_version();
    print_versions(&versions, filter, &installed, active.as_deref());
    Ok(())
}

#[cfg(not(windows))]
async fn run_linux(filter: Option<String>) -> Result<()> {
    use crate::releases::fetch_standalone_versions;
    println!(
        "{}",
        "Obteniendo versiones disponibles para Linux..."
            .truecolor(DIM.0, DIM.1, DIM.2)
    );
    let versions = fetch_standalone_versions().await?;
    let installed = installed_versions();
    let active = active_version();
    print_versions(&versions, filter, &installed, active.as_deref());
    Ok(())
}

fn installed_versions() -> Vec<String> {
    let Ok(dir) = crate::dirs::versions_dir() else {
        return vec![];
    };
    let Ok(entries) = std::fs::read_dir(dir) else {
        return vec![];
    };
    entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect()
}

fn active_version() -> Option<String> {
    let current_path = crate::dirs::current_alias_dir().ok()?;
    if !current_path.exists() {
        return None;
    }
    let target = std::fs::read_link(&current_path).unwrap_or(current_path);
    target.file_name()?.to_str().map(|s| s.to_string())
}

fn print_version_line(v: &Version, installed: &[String], active: Option<&str>, is_latest: bool) {
    let ver_str = v.to_string();
    let is_installed = installed.contains(&ver_str);
    let is_active = active == Some(ver_str.as_str());
    let latest_tag = if is_latest {
        format!("  {}", "(Latest)".truecolor(255, 200, 50))
    } else {
        String::new()
    };

    if is_installed {
        let label = if is_active { " (activa)" } else { " (instalada)" };
        println!(
            "    {} {}{}{}",
            "*".green().bold(),
            ver_str.green().bold(),
            label.green(),
            latest_tag,
        );
    } else {
        println!("    {}{}", ver_str.truecolor(DIM.0, DIM.1, DIM.2), latest_tag);
    }
}

fn wait_space() -> bool {
    use crossterm::event::{poll, read, Event, KeyCode};
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
    use std::time::Duration;

    let _ = enable_raw_mode();

    // Vaciar eventos pendientes en el buffer (ej: el Enter del comando anterior)
    while poll(Duration::ZERO).unwrap_or(false) {
        let _ = read();
    }

    let pressed_space = loop {
        match read() {
            Ok(Event::Key(key)) => match key.code {
                KeyCode::Char(' ') => break true,
                _ => break false,
            },
            _ => {}
        }
    };
    let _ = disable_raw_mode();
    pressed_space
}

fn print_versions(
    versions: &[Version],
    filter: Option<String>,
    installed: &[String],
    active: Option<&str>,
) {
    let interactive = std::io::stdout().is_terminal() && filter.is_none();

    let filtered: Vec<&Version> = versions
        .iter()
        .filter(|v| v.pre.is_empty())
        .filter(|v| v.major > 3 || (v.major == 3 && v.minor >= MIN_MINOR))
        .filter(|v| {
            filter
                .as_deref()
                .map(|f| v.to_string().starts_with(f))
                .unwrap_or(true)
        })
        .collect();

    if filtered.is_empty() {
        println!("No se encontraron versiones para el filtro dado.");
        return;
    }

    println!(
        "\n  {}  {}",
        format!("{} versiones disponibles.", filtered.len())
            .truecolor(CMD.0, CMD.1, CMD.2)
            .bold(),
        "Usa --filter <prefijo> para filtrar  (ej: --filter 3.12)"
            .truecolor(DIM.0, DIM.1, DIM.2)
    );

    let mut groups: BTreeMap<(u64, u64), Vec<&Version>> = BTreeMap::new();
    for v in &filtered {
        groups.entry((v.major, v.minor)).or_default().push(v);
    }

    let mut keys: Vec<(u64, u64)> = groups.keys().cloned().collect();
    keys.sort_by(|a, b| b.cmp(a));

    for (i, key) in keys.iter().enumerate() {
        if interactive && i >= 3 {
            print!(
                "  {}\n",
                "Presiona [SPACE] para ver más versiones  [otra tecla] salir"
                    .truecolor(DIM.0, DIM.1, DIM.2)
            );
            let _ = std::io::stdout().flush();

            if !wait_space() {
                println!();
                break;
            }
            println!();
        }

        let group = &groups[key];

        println!(
            "\n  {}",
            format!("{}.{}:", key.0, key.1)
                .truecolor(CMD.0, CMD.1, CMD.2)
                .bold()
        );
        println!();

        for (j, v) in group.iter().enumerate() {
            print_version_line(v, installed, active, j == 0);
        }
        println!();
    }
}
