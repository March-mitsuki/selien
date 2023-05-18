use log::error;

use super::utils::go::get_root_pkg_name;
use crate::types::{config::Config, lang::SupportedLang, spec::Spec};

use std::path::PathBuf;

pub fn with_special(lang: &SupportedLang, content: &str, spec: &Spec, config: &Config) -> String {
    let mut result = String::new();
    let mut _content = content.to_owned();

    #[allow(clippy::single_match)]
    match lang {
        SupportedLang::Go => {
            let p = PathBuf::from(&spec.path);
            let parent = p.parent().unwrap();

            let mut package_name = match parent.file_name() {
                Some(n) => n.to_str().unwrap(),
                None => {
                    error!("Can not get file stem from path {}", spec.path);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            if parent == PathBuf::from(&config.spec.root) {
                package_name = get_root_pkg_name(config);
            }

            result += &format!("package {}\n\n", package_name);

            let tokens = go::find_match_tokens(&_content);
            if !tokens.is_empty() {
                for token in tokens {
                    let pkg_name = go::get_ref_pkg_name(&token.path, &spec.path, config);
                    _content = _content.replace(&token.token, &pkg_name);
                }
            }
        }
        _ => {}
    }

    result += &_content;
    result
}

mod go {
    use regex::Regex;
    use std::path::PathBuf;

    use crate::{
        compiler::utils::go::get_root_pkg_name, path::normalize_path, types::config::Config,
    };

    #[derive(Debug)]
    pub(super) struct MatchToken {
        pub(super) token: String,
        pub(super) path: String,
    }

    pub(super) fn find_match_tokens(text: &str) -> Vec<MatchToken> {
        let re = Regex::new(r"\[selien-ref\](.*?)\[selien-ref\]").unwrap();
        re.captures_iter(text)
            .map(|cap| MatchToken {
                token: cap[0].to_string(),
                path: cap[1].to_string(),
            })
            .collect()
    }

    pub(super) fn get_ref_pkg_name(path: &String, current: &String, config: &Config) -> String {
        let p = PathBuf::from(path);
        let mut result: String;

        if p.is_absolute() {
            result = p
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
        } else {
            let cp = PathBuf::from(current);
            let parent = cp.parent().unwrap();
            let joined = parent.join(p);
            let normalized = normalize_path(&joined);
            result = normalized
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
        }

        if result == config.spec.root {
            result = get_root_pkg_name(config).to_string();
        }

        result
    }
}
