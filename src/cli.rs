use clap::{Parser, Subcommand};
use crate::commands::env::Shell;

#[derive(Parser)]
#[command(name = "pvm", about = "Python Version Manager", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Instala una versión de Python
    Install {
        /// Versión a instalar (ej: 3.12.4)
        version: String,
    },
    /// Cambia la versión activa de Python
    Use {
        /// Versión a activar (ej: 3.12.4)
        version: String,
    },
    /// Lista las versiones instaladas localmente
    List,
    /// Lista las versiones disponibles en python.org
    ListRemote {
        /// Filtrar por prefijo (ej: 3.12)
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Elimina una versión instalada
    Uninstall {
        /// Versión a eliminar (ej: 3.12.4)
        version: String,
    },
    /// Establece la versión global por defecto
    Default {
        /// Versión a usar como default (ej: 3.12.4)
        version: String,
    },
    /// Imprime el comando para añadir pvm al PATH del shell actual
    Env {
        /// Shell de destino (detectado automáticamente si se omite)
        #[arg(long, value_enum)]
        shell: Option<Shell>,
    },
    /// Crea un entorno virtual usando la versión activa (pinned al directorio exacto)
    Venv {
        /// Directorio donde crear el entorno virtual
        dir: String,
        /// Opciones adicionales pasadas a `python -m venv` (ej: --system-site-packages)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}
