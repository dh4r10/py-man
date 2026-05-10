use anyhow::Result;
use colored::Colorize;
use crate::i18n::{self, Language};
use crate::t;

const CMD: (u8, u8, u8) = (200, 160, 255);
const DIM: (u8, u8, u8) = (110, 90, 140);

pub fn run_list() -> Result<()> {
    let current = i18n::current();

    println!();
    println!(
        "  {}",
        t!("Available languages:", "Idiomas disponibles:")
            .truecolor(CMD.0, CMD.1, CMD.2)
            .bold()
    );
    println!();

    for lang in Language::all() {
        let is_current = *lang == current;
        let name = lang.display_name();
        let code = lang.code();

        if is_current {
            println!(
                "  {} {} {}",
                "*".truecolor(CMD.0, CMD.1, CMD.2).bold(),
                name.truecolor(CMD.0, CMD.1, CMD.2).bold(),
                format!("({})", code).truecolor(DIM.0, DIM.1, DIM.2),
            );
        } else {
            println!(
                "    {} {}",
                name.truecolor(DIM.0, DIM.1, DIM.2),
                format!("({})", code).truecolor(DIM.0, DIM.1, DIM.2),
            );
        }
    }

    println!();
    println!(
        "  {}",
        t!(
            "Use `pvm lang change <lang>` or `pvm lang -c <lang>` to switch.",
            "Usa `pvm lang change <lang>` o `pvm lang -c <lang>` para cambiar."
        )
        .truecolor(DIM.0, DIM.1, DIM.2)
    );
    println!();

    Ok(())
}

pub fn run_change(lang_str: &str) -> Result<()> {
    let lang = Language::from_str(lang_str).ok_or_else(|| {
        anyhow::anyhow!(
            "{}",
            t!(
                "Unknown language: '{}'. Available: english, español",
                "Idioma desconocido: '{}'. Disponibles: english, español",
                lang_str
            )
        )
    })?;

    let path = crate::dirs::language_file()?;
    i18n::save(&path, lang)?;

    // Show confirmation in the newly selected language
    let msg = match lang {
        Language::English => format!("Language set to {}", lang.display_name()),
        Language::Spanish => format!("Idioma establecido: {}", lang.display_name()),
    };
    println!("{}", msg);

    Ok(())
}
