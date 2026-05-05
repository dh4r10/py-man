use clap::{Parser, Subcommand};
use clap::builder::styling::{AnsiColor, Effects, Styles};
use crate::commands::env::Shell;

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Magenta.on_default()        | Effects::BOLD)
        .usage(AnsiColor::Magenta.on_default()         | Effects::BOLD)
        .literal(AnsiColor::BrightMagenta.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Cyan.on_default())
        .error(AnsiColor::Red.on_default()             | Effects::BOLD)
        .valid(AnsiColor::BrightGreen.on_default())
        .invalid(AnsiColor::Yellow.on_default())
}

#[derive(Parser)]
#[command(
    name    = "pvm",
    about   = "Python Version Manager",
    version,
    styles  = styles(),
    disable_help_subcommand = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Descarga e instala una versión de Python desde NuGet / python-build-standalone
    Install {
        /// Versión a instalar  (ej: 3.12.4)
        version: String,
    },

    /// Cambia la versión activa e instala los shims en ~/.pvm/bin
    Use {
        /// Versión a activar  (ej: 3.12.4)
        version: String,
    },

    /// Lista las versiones instaladas localmente  (* marca la activa)
    List,

    /// Lista las versiones disponibles en python.org
    ListRemote {
        /// Filtrar por prefijo  (ej: 3.12)
        #[arg(short, long, value_name = "PREFIX")]
        filter: Option<String>,
    },

    /// Elimina una versión instalada (no permite eliminar la versión activa)
    Uninstall {
        /// Versión a eliminar  (ej: 3.12.4)
        version: String,
    },

    /// Establece la versión global por defecto
    Default {
        /// Versión a marcar como default  (ej: 3.12.4)
        version: String,
    },

    /// Imprime el comando para añadir ~/.pvm/bin al PATH del shell actual
    Env {
        /// Shell de destino (detectado automáticamente si se omite)
        #[arg(long, value_enum, value_name = "SHELL")]
        shell: Option<Shell>,
    },

    /// Crea un entorno virtual anclado a una versión exacta de Python
    ///
    /// Usa la versión activa por defecto. Para especificar otra:
    ///   pvm -3.12.4 venv .venv
    Venv {
        /// Directorio donde crear el entorno virtual
        dir: String,
        /// Opciones adicionales para `python -m venv`  (ej: --system-site-packages)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Desinstala PVM del sistema (elimina ~/.pvm/, PATH y perfil de PowerShell)
    UninstallSelf,
}
