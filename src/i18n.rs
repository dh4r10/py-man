use std::sync::OnceLock;

static LANG: OnceLock<Language> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
    Spanish,
}

impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Spanish => "es",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Spanish => "Español",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "en" | "english" | "inglés" | "ingles" => Some(Language::English),
            "es" | "español" | "espanol" | "spanish" => Some(Language::Spanish),
            _ => None,
        }
    }

    pub fn all() -> &'static [Language] {
        &[Language::English, Language::Spanish]
    }
}

pub fn init(lang: Language) {
    let _ = LANG.set(lang);
}

pub fn current() -> Language {
    *LANG.get().unwrap_or(&Language::English)
}

pub fn load(path: &std::path::Path) -> Language {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| Language::from_str(s.trim()))
        .unwrap_or(Language::English)
}

pub fn save(path: &std::path::Path, lang: Language) -> anyhow::Result<()> {
    std::fs::write(path, lang.code())?;
    Ok(())
}

/// Returns the translated string. For static strings (no format args), use:
///   t!("English text", "Texto en español")
/// For formatted strings:
///   t!("Version {} installed", "Versión {} instalada", version)
#[macro_export]
macro_rules! t {
    ($en:literal, $es:literal) => {{
        match $crate::i18n::current() {
            $crate::i18n::Language::English => $en.to_string(),
            $crate::i18n::Language::Spanish => $es.to_string(),
        }
    }};
    ($en:literal, $es:literal, $($arg:tt)+) => {{
        match $crate::i18n::current() {
            $crate::i18n::Language::English => format!($en, $($arg)+),
            $crate::i18n::Language::Spanish => format!($es, $($arg)+),
        }
    }};
}
