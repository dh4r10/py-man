use colored::Colorize;

// Paleta morada
const P1: (u8, u8, u8) = (138,  43, 226); // violeta profundo  — líneas 1-2
const P2: (u8, u8, u8) = (160,  90, 255); // morado medio      — líneas 3-4
const P3: (u8, u8, u8) = (185, 140, 255); // lila claro        — líneas 5-6
const CMD: (u8, u8, u8) = (200, 160, 255); // nombres de comando
const DSC: (u8, u8, u8) = (160, 140, 185); // descripciones
const DIM: (u8, u8, u8) = (110,  90, 140); // texto sutil

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
        "Gestiona múltiples versiones de Python desde la terminal."
            .truecolor(DIM.0, DIM.1, DIM.2)
    );
    println!();

    section("USO");
    println!(
        "  {}",
        "pvm <comando> [opciones]".truecolor(CMD.0, CMD.1, CMD.2)
    );
    println!();

    section("COMANDOS");

    let commands: &[(&str, &str)] = &[
        ("install <version>",   "Descarga e instala una versión de Python"),
        ("use <version>",       "Cambia la versión activa"),
        ("list",                "Lista las versiones instaladas"),
        ("list-remote",         "Lista versiones disponibles en python.org"),
        ("uninstall <version>", "Elimina una versión instalada"),
        ("default <version>",   "Establece la versión global por defecto"),
        ("env",                 "Configura el PATH del shell actual"),
        ("venv <dir>",          "Crea un entorno virtual con la versión activa"),
    ];

    for (name, desc) in commands {
        // Padding sobre el texto plano para que las ANSI no rompan el ancho
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
        "Crea el venv con una versión específica sin cambiar el use activo"
            .truecolor(DSC.0, DSC.1, DSC.2),
    );
    println!(
        "  {}",
        "Ejemplo: pvm -3.12.4 venv .venv".truecolor(DIM.0, DIM.1, DIM.2)
    );

    println!();
    println!(
        "  {}",
        format!(
            "Ejecuta {} para más información sobre un comando.",
            "pvm <comando> --help".truecolor(CMD.0, CMD.1, CMD.2)
        )
        .truecolor(DIM.0, DIM.1, DIM.2)
    );
    println!();
}

fn section(title: &str) {
    println!("  {}", title.truecolor(P2.0, P2.1, P2.2).bold());
}
