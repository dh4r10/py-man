use colored::Colorize;
use crate::i18n::Language;
use crate::t;

// Purple palette
const P1: (u8, u8, u8) = (138,  43, 226);
const P2: (u8, u8, u8) = (160,  90, 255);
const P3: (u8, u8, u8) = (185, 140, 255);
const CMD: (u8, u8, u8) = (200, 160, 255);
const DSC: (u8, u8, u8) = (160, 140, 185);
const DIM: (u8, u8, u8) = (110,  90, 140);

pub fn print_help() {
    println!();
    let logo = [
        (r"  ██████╗ ██╗   ██╗███╗   ███╗", P1),
        (r"  ██╔══██╗╚██╗ ██╔╝████╗ ████║", P1),
        (r"  ██████╔╝ ╚████╔╝ ██╔████╔██║", P2),
        (r"  ██╔═══╝   ╚██╔╝  ██║╚██╔╝██║", P2),
        (r"  ██║        ██║   ██║ ╚═╝ ██║", P3),
        (r"  ╚═╝        ╚═╝   ╚═╝     ╚═╝", P3),
    ];

    for (line, (r, g, b)) in logo {
        println!("{}", line.truecolor(r, g, b).bold());
    }

    println!();
    println!(
        "  {}  {}",
        "Python Version Manager".bold().bright_white(),
        format!("v{}", env!("CARGO_PKG_VERSION")).truecolor(140, 100, 210),
    );
    println!(
        "  {}",
        t!(
            "Manage multiple Python versions from the terminal.",
            "Gestiona múltiples versiones de Python desde la terminal."
        )
        .truecolor(DIM.0, DIM.1, DIM.2)
    );
    println!();

    section(&t!("LANGUAGE", "IDIOMA"));
    let current_lang = crate::i18n::current();
    println!(
        "  {}  {}",
        current_lang.display_name().truecolor(CMD.0, CMD.1, CMD.2).bold(),
        format!("({})", current_lang.code()).truecolor(DIM.0, DIM.1, DIM.2),
    );
    println!(
        "  {}",
        t!(
            "Change with `pvm lang change <lang>` or `-c <lang>`  (english / español)",
            "Cambia con `pvm lang change <lang>` o `-c <lang>`  (english / español)"
        )
        .truecolor(DIM.0, DIM.1, DIM.2)
    );
    println!();

    section(&t!("USAGE", "USO"));
    println!(
        "  {}",
        t!("pvm <command> [options]", "pvm <comando> [opciones]")
            .truecolor(CMD.0, CMD.1, CMD.2)
    );
    println!();

    section(&t!("COMMANDS", "COMANDOS"));

    let commands: &[(&str, &str)] = match crate::i18n::current() {
        Language::English => &[
            ("install <version>",   "Download and install a Python version"),
            ("use <version>",       "Switch the active version"),
            ("list",                "List installed versions"),
            ("list-remote",         "List available versions from python.org"),
            ("uninstall <version>", "Remove an installed version"),
            ("default <version>",   "Set the global default version"),
            ("env",                 "Configure the current shell's PATH"),
            ("venv <dir>",          "Create a virtual environment with the active version"),
        ],
        Language::Spanish => &[
            ("install <version>",   "Descarga e instala una versión de Python"),
            ("use <version>",       "Cambia la versión activa"),
            ("list",                "Lista las versiones instaladas"),
            ("list-remote",         "Lista versiones disponibles en python.org"),
            ("uninstall <version>", "Elimina una versión instalada"),
            ("default <version>",   "Establece la versión global por defecto"),
            ("env",                 "Configura el PATH del shell actual"),
            ("venv <dir>",          "Crea un entorno virtual con la versión activa"),
        ],
    };

    for (name, desc) in commands {
        let padded = format!("{:<28}", name);
        println!(
            "  {}{}",
            padded.truecolor(CMD.0, CMD.1, CMD.2).bold(),
            desc.truecolor(DSC.0, DSC.1, DSC.2),
        );
    }

    println!();
    println!(
        "  {}  {}",
        "pvm -<version> venv <dir>".truecolor(CMD.0, CMD.1, CMD.2).bold(),
        t!(
            "Create venv with a specific version without changing the active use",
            "Crea el venv con una versión específica sin cambiar el use activo"
        )
        .truecolor(DSC.0, DSC.1, DSC.2),
    );
    println!(
        "  {}",
        t!("Example: pvm -3.12.4 venv .venv", "Ejemplo: pvm -3.12.4 venv .venv")
            .truecolor(DIM.0, DIM.1, DIM.2)
    );

    println!();
    println!(
        "  {}",
        t!(
            "Run {} for more information about a command.",
            "Ejecuta {} para más información sobre un comando.",
            "pvm <command> --help".truecolor(CMD.0, CMD.1, CMD.2)
        )
        .truecolor(DIM.0, DIM.1, DIM.2)
    );
    println!();
}

fn section(title: &str) {
    println!("  {}", title.truecolor(P2.0, P2.1, P2.2).bold());
}
