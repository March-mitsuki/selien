use super::types::ast_type_alias::Keywords;
use crate::types::lang::SupportedLang;

pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

impl Keywords {
    pub fn to_string(&self, lang: SupportedLang) -> String {
        match lang {
            SupportedLang::TypeScript => match &self {
                Keywords::Any => String::from("any"),
                Keywords::Boolean => String::from("boolean"),
                Keywords::Number => String::from("number"),
                Keywords::String => String::from("string"),
            },
            SupportedLang::Go => match &self {
                Keywords::Any => String::from("interface{}"),
                Keywords::Boolean => String::from("bool"),
                Keywords::Number => String::from("int"),
                Keywords::String => String::from("string"),
            },
        }
    }
}
