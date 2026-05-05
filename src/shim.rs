use anyhow::Result;
use std::path::PathBuf;
use crate::dirs;

/// Reenvía `python` / `python3` al ejecutable correcto según la plataforma.
pub fn forward_python() -> i32 {
    #[cfg(windows)]
    return forward("python.exe");
    #[cfg(not(windows))]
    return forward("python3");
}

/// Invocado como `python.exe` / `pythonw.exe`: reenvía al Python real de la versión activa.
pub fn forward(exe_name: &str) -> i32 {
    let python = match resolve(exe_name) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[pvm] {}", e);
            return 1;
        }
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    exec(python, args)
}

/// Invocado como `pip.exe` / `pip3.exe`: ejecuta `python -m pip <args>`.
pub fn forward_pip() -> i32 {
    let python = match resolve("python.exe") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[pvm] {}", e);
            return 1;
        }
    };
    let mut args: Vec<String> = vec!["-m".into(), "pip".into()];
    args.extend(std::env::args().skip(1));
    exec(python, args)
}

fn exec(exe: PathBuf, args: Vec<String>) -> i32 {
    match std::process::Command::new(&exe).args(&args).status() {
        Ok(s) => s.code().unwrap_or(1),
        Err(e) => {
            eprintln!("[pvm] No se pudo ejecutar {:?}: {}", exe, e);
            1
        }
    }
}

/// Resuelve el Python real de la versión activa a través del junction aliases/current.
/// El proceso hijo que se lanza tendrá sys.executable = path real de la versión,
/// no ~/.pvm/bin — lo que garantiza que los pyvenv.cfg queden fijados a la versión exacta.
fn resolve(exe_name: &str) -> Result<PathBuf> {
    let current = dirs::current_alias_dir()?;

    if !current.exists() {
        anyhow::bail!("No hay ninguna versión activa. Ejecuta `pvm use <version>` primero.");
    }

    // canonicalize() sigue el junction aliases/current → ~/.pvm/versions/X.Y.Z
    let resolved = std::fs::canonicalize(&current)?;

    #[cfg(windows)]
    {
        let s = resolved.to_string_lossy();
        let clean = PathBuf::from(s.trim_start_matches(r"\\?\"));
        let python = clean.join("tools").join(exe_name);
        if !python.exists() {
            anyhow::bail!(
                "No se encontró {} en la versión activa. Reinstala con `pvm install`.",
                exe_name
            );
        }
        return Ok(python);
    }

    #[cfg(not(windows))]
    {
        let python = resolved.join("bin").join("python3");
        if !python.exists() {
            anyhow::bail!("No se encontró python3 en la versión activa.");
        }
        return Ok(python);
    }
}
