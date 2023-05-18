use log::error;

use super::config::Config;

#[derive(Debug)]
pub enum SupportedLang {
    TypeScript,
    Go,
}

impl SupportedLang {
    pub fn all() -> Vec<Self> {
        vec![Self::TypeScript, Self::Go]
    }
    pub fn from(s: &str) -> Self {
        let ts_alias = Self::get_alias(&Self::TypeScript);
        let go_alias = Self::get_alias(&Self::Go);
        if ts_alias.contains(&s) {
            return Self::TypeScript;
        }
        if go_alias.contains(&s) {
            return Self::Go;
        }

        error!("Unsupported language or alias: {}.", s);
        if crate::is_dev() {
            panic!();
        } else {
            std::process::exit(1);
        }
    }
    pub fn is_defined_in_config(&self, config: &Config) -> bool {
        match self {
            Self::TypeScript => config.output.typescript.is_some(),
            Self::Go => config.output.go.is_some(),
        }
    }
    pub fn get_alias<'a>(lang: &Self) -> Vec<&'a str> {
        match lang {
            Self::TypeScript => vec!["ts", "typescript"],
            Self::Go => vec!["go", "golang"],
        }
    }
}
