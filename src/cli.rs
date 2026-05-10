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
    /// Download and install a Python version from NuGet / python-build-standalone
    Install {
        /// Version to install (e.g. 3.12.4)
        version: String,
    },

    /// Switch the active version and install shims in ~/.pvm/bin
    Use {
        /// Version to activate (e.g. 3.12.4)
        version: String,
    },

    /// List locally installed versions (* marks the active one)
    List,

    /// List versions available on python.org
    ListRemote {
        /// Filter by prefix (e.g. 3.12)
        #[arg(short, long, value_name = "PREFIX")]
        filter: Option<String>,
    },

    /// Remove an installed version (cannot remove the active version)
    Uninstall {
        /// Version to remove (e.g. 3.12.4)
        version: String,
    },

    /// Set the global default version
    Default {
        /// Version to set as default (e.g. 3.12.4)
        version: String,
    },

    /// Print the command to add ~/.pvm/bin to the current shell's PATH
    Env {
        /// Target shell (auto-detected if omitted)
        #[arg(long, value_enum, value_name = "SHELL")]
        shell: Option<Shell>,
    },

    /// Create a virtual environment pinned to an exact Python version
    ///
    /// Uses the active version by default. To use a specific version:
    ///   pvm -3.12.4 venv .venv
    Venv {
        /// Directory where the virtual environment will be created
        dir: String,
        /// Extra options passed to `python -m venv` (e.g. --system-site-packages)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Uninstall PVM from the system (removes ~/.pvm/, PATH entries and shell profile)
    UninstallSelf,

    /// Manage the display language
    ///
    /// Lists available languages when called without arguments.
    /// Use 'change' (or '-c') to switch language:
    ///   pvm lang change english
    ///   pvm lang -c español
    #[command(name = "lang")]
    Language {
        #[command(subcommand)]
        action: Option<LanguageAction>,
    },
}

#[derive(Subcommand)]
pub enum LanguageAction {
    /// Change the display language  (alias: -c)
    ///
    /// Accepted values: english, español, en, es
    ///
    /// Examples:
    ///   pvm lang change english
    ///   pvm lang -c español
    #[command(alias = "c")]
    Change {
        /// Language name: english, español (or en, es)
        language: String,
    },
}
