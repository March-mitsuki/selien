use std::path::PathBuf;

use log::{debug, error};

use crate::{
    compiler::utils::{go, ts},
    generator::types::{Import, Imports},
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
                match import {
                    Import::Dyn(di) => {
                        let o = config.output.go.as_ref().expect(
                            "Maybe you want to generate Go code but forget to set it in config file.",
                        );
                        let indent = " ".repeat(o.tabsize);
                        let mut f = format!("{}\"{}\"", indent, di.from);
                        let is_last = idx == imports.len() - 1;
                        if !is_last {
                            f += "\n"
                        }

                        froms += &f;
                    }
                    Import::Ref(ri) => {
                        let mut f = process_from(lang, current, &ri.from, config);
                        if !f.is_empty() {
                            let is_last = idx == imports.len() - 1;
                            if !is_last {
                                f += "\n"
                            }
                            froms += &f;
                        }
                    }
                };
            });

            let s = format!("import (\n{}\n)\n\n", go::remove_duplicate_import(&froms));
            result += &s;
        }
        SupportedLang::TypeScript => {
            debug!("before reduce imports: {:#?}", imports);
            let r_imports = ts::reduce_imports(imports);

            r_imports.iter().enumerate().for_each(|(idx, import)| {
                match import {
                    Import::Dyn(di) => {
                        let mut s = format!("import {{ {} }} from \"{}\";\n", di.name, di.from,);
                        let is_last = idx == r_imports.len() - 1;
                        if is_last {
                            s += "\n"
                        }

                        result += &s;
                    }
                    Import::Ref(ri) => {
                        let mut s = format!(
                            "import {{ {} }} from \"{}\";\n",
                            ri.name,
                            process_from(lang, current, &ri.from, config)
                        );
                        let is_last = idx == r_imports.len() - 1;
                        if is_last {
                            s += "\n"
                        }

                        result += &s;
                    }
                };
            });
        }
    }

    result += content;
    result
}

/// `return`
/// - go
///     - if self import, will return empty string
fn process_from(lang: &SupportedLang, current: &String, from: &PathBuf, config: &Config) -> String {
    let result: String;

    // selien $ref abs path must be start with `/`, even windows
    if from.starts_with("/") {
        match lang {
            SupportedLang::Go => {
                let o = config.output.go.as_ref().expect(
                    "Maybe you want to generate Go code but forget to set it in config file.",
                );

                let indent = " ".repeat(o.tabsize);

                let goroot_to_output = diff_paths(&o.output, &o.root)
                    .expect("Can not diff path from go-root to output");

                // because from is absolute path from <selien-root>
                // so just add `mod_name/middle/path/to/pkg_name` to head
                let result_path = PathBuf::from(&o.mod_name)
                    .join(goroot_to_output)
                    .join(to_relative(from.parent().unwrap()));

                let ref_pkg = result_path.file_name().unwrap();
                let cp = PathBuf::from(current);
                let cuurent_pkg = cp.parent().unwrap().file_name().unwrap().to_str().unwrap();
                if ref_pkg == cuurent_pkg {
                    result = String::new();
                } else {
                    result = format!(
                        "{}\"{}\"",
                        indent,
                        result_path.to_str().unwrap().replace('\\', "/")
                    );
                }
            }
            SupportedLang::TypeScript => {
                let cp = PathBuf::from(current);
                let cp_parent = cp.parent().unwrap();

                // because from is absolute path from <selien-root>
                // like /foo/bar, so add `<selien-root>/` to head
                let f_parent = from.parent().unwrap();
                let f_relative = PathBuf::from(&config.spec.root).join(to_relative(f_parent));

                let diff = match diff_paths(&f_relative, cp_parent) {
                    Some(r) => r,
                    None => {
                        error!(
                            "Can not diff path {:?} to {:?}",
                            &cp_parent.to_str().unwrap(),
                            &f_relative.to_str().unwrap()
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };

                let result_path = add_dot(&diff.join(from.file_stem().unwrap()));

                result = result_path.to_str().unwrap().replace('\\', "/");
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
                let joined = cp.parent().unwrap().join(from);
                let normalized = normalize_path(&joined);

                let replaced = match go::replace_selien_root(&normalized, config) {
                    Ok(r) => r,
                    Err(err) => {
                        error!(
                            "relative path {} out of selien-root: {}",
                            from.to_str().unwrap(),
                            err
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };
                result_path.push(replaced.parent().unwrap());

                let ref_pkg = result_path.file_name().unwrap();
                let cuurent_pkg = cp.parent().unwrap().file_name().unwrap().to_str().unwrap();
                if ref_pkg == cuurent_pkg {
                    result = String::new();
                } else {
                    result = format!(
                        "{}\"{}\"",
                        indent,
                        result_path.to_str().unwrap().replace('\\', "/")
                    );
                }
            }
            SupportedLang::TypeScript => {
                result = from.to_str().unwrap().replace('\\', "/");
            }
        }
    }

    result
}
