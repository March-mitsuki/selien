use std::path::PathBuf;

use log::error;

use crate::{
    compiler::utils::go,
    generator::types::Imports,
    path::{add_dot, diff_paths, normalize_path, to_relative},
    types::{config::Config, lang::SupportedLang},
};

pub fn with_import(
    lang: &SupportedLang,
    current: &String,
    content: &String,
    imports: &Imports,
    config: &Config,
) -> String {
    let mut result = String::new();

    match lang {
        SupportedLang::Go => {
            let mut froms = String::new();
            imports.iter().enumerate().for_each(|(idx, import)| {
                let mut f = process_from(lang, current, &import.from, config);
                let is_last = idx == imports.len() - 1;
                if !is_last {
                    f += "\n"
                }

                froms += &f;
            });
            let s = format!("import (\n{}\n)\n\n", froms);
            result += &s;
        }
        SupportedLang::TypeScript => {
            imports.iter().enumerate().for_each(|(idx, import)| {
                let mut s = format!(
                    "import {{ {} }} from \"{}\";\n",
                    import.name,
                    process_from(lang, current, &import.from, config)
                );
                let is_last = idx == imports.len() - 1;
                if is_last {
                    s += "\n"
                }

                result += &s;
            });
        }
    }

    result += content;
    result
}

fn process_from(lang: &SupportedLang, current: &String, from: &String, config: &Config) -> String {
    let result: String;

    let f = PathBuf::from(from);
    if f.is_absolute() {
        match lang {
            SupportedLang::Go => {
                let o = config.output.go.as_ref().expect(
                    "Maybe you want to generate Go code but forget to set it in config file.",
                );

                let indent = " ".repeat(o.tabsize);

                // because f is absolute path from <selien-root>
                // so just add with `mod_name/pkg_name`
                let result_path = PathBuf::from(&o.mod_name)
                    .join(go::get_root_pkg_name(config))
                    .join(to_relative(f.parent().unwrap()));

                result = format!("{}\"{}\"", indent, result_path.display());
            }
            SupportedLang::TypeScript => {
                let cp = PathBuf::from(current);
                let cp_parent = cp.parent().unwrap();

                // because f is absolute path from <selien-root>
                // like /foo/bar, so add `<selien-root>/` to head
                let f_parent = f.parent().unwrap();
                let f_relative = PathBuf::from(&config.spec.root).join(to_relative(f_parent));

                let diff = match diff_paths(&f_relative, cp_parent) {
                    Some(r) => r,
                    None => {
                        error!(
                            "Can not diff path {:?} to {:?}",
                            &cp_parent.display(),
                            &f_relative.display()
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };

                let result_path = add_dot(&diff.join(f.file_stem().unwrap()));

                result = format!("{}", result_path.display());
            }
        }
    } else {
        match lang {
            SupportedLang::Go => {
                let o = config.output.go.as_ref().expect(
                    "Maybe you want to generate Go code but forget to set it in config file.",
                );

                let indent = " ".repeat(o.tabsize);
                let mut result_path = PathBuf::new();

                let cp = PathBuf::from(current);
                let joined = cp.parent().unwrap().join(&f);
                let normalized = normalize_path(&joined);

                let replaced = match go::replace_selien_root(&normalized, config) {
                    Ok(r) => r,
                    Err(err) => {
                        error!("relative path {} out of selien-root: {}", &f.display(), err);
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };

                result_path.push(replaced.parent().unwrap());

                result = format!("{}\"{}\"", indent, result_path.display());
            }
            SupportedLang::TypeScript => {
                result = format!("{}", f.display());
            }
        }
    }

    result
}
