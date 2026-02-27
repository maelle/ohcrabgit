use clap::ValueEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Lang {
    En,
    Fr,
    Es,
}

impl Lang {
    pub fn code(&self) -> &str {
        match self {
            Lang::En => "en",
            Lang::Fr => "fr",
            Lang::Es => "es",
        }
    }

    fn parse(s: &str) -> Option<Lang> {
        match s {
            "en" => Some(Lang::En),
            "fr" => Some(Lang::Fr),
            "es" => Some(Lang::Es),
            _ => None,
        }
    }

    fn strip_region(s: &str) -> &str {
        s.splitn(2, ['-', '_']).next().unwrap_or(s)
    }

    /// Resolution order: --lang flag > ZUT_LANG env var > system locale > En
    pub fn resolve(flag: Option<Lang>) -> Lang {
        if let Some(lang) = flag {
            return lang;
        }
        if let Ok(val) = std::env::var("ZUT_LANG") {
            // ZUT_LANG is set: use it, or fall back to En for unknown values.
            // Do NOT fall through to system locale.
            let base = Self::strip_region(&val);
            return Self::parse(base).unwrap_or(Lang::En);
        }
        if let Some(locale) = sys_locale::get_locale() {
            let base = Self::strip_region(&locale);
            if let Some(lang) = Self::parse(base) {
                return lang;
            }
        }
        Lang::En
    }
}
