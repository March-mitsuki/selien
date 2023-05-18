use log::error;

use super::special::with_special;
use super::types::{self, Output, OutputFile};
use crate::compiler::importer::with_import;
use crate::generator::types::{Imports, TabSize};
use crate::path::process_path;
use crate::transformer;
use crate::types::lang::SupportedLang;
use crate::types::{config::Config, spec::SpecList};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

fn with_notice(content: &String) -> String {
    let mut result = String::new();
    result += &format!(
        "// This file is created automatically by Selien.\n// Do NOT edit.\n\n{}",
        content
    );
    result
}

impl SupportedLang {
    pub fn compiler(&self, config: &Config, spec_list: &SpecList) {
        let mut output = types::Output {
            config,
            files: Vec::new(),
        };
        for spec in spec_list {
            let ast_list = transformer::main::transformer(spec);
            let mut content: String = String::new();
            let mut imports: Imports = vec![];
            let tabsize = TabSize {
                go: config.output.go.as_ref().unwrap().tabsize,
                typescript: config.output.typescript.as_ref().unwrap().tabsize,
            };

            for ast in ast_list {
                content += &ast.generator(self, &mut imports, &tabsize);
            }

            if !imports.is_empty() {
                content = with_import(self, &spec.path, &content, &imports, config);
            }

            content = with_special(self, &content, spec, config);
            content = with_notice(&content);

            let file = OutputFile {
                path: spec.path.clone(),
                content,
            };
            output.files.push(file);
        }

        file_creater(&output, self);
    }
}

fn file_creater(output: &Output, lang: &SupportedLang) {
    for file in output.files.iter() {
        let mut p = PathBuf::from(&file.path);
        let config = &output.config;

        p = match p.strip_prefix(&config.spec.root) {
            Ok(r) => r.to_path_buf(),
            Err(err) => {
                error!(
                    "File path {} is not start with spec root in config: {}. Error: {}",
                    file.path, config.spec.root, err
                );
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }
        };
        process_output(&mut p, config, lang);
        p = process_path(p);

        let dir = match p.parent() {
            Some(r) => r,
            None => {
                error!("Can not get parent dir of {}", file.path);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }
        };

        match create_dir_all(dir) {
            Ok(_) => {}
            Err(err) => {
                error!("Can not create file {} with err: {}", file.path, err);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }
        };

        let mut f = match File::create(&p) {
            Ok(r) => r,
            Err(err) => {
                error!("Can not create file {} with err: {}", file.path, err);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }
        };

        match f.write(file.content.as_bytes()) {
            Ok(_) => {}
            Err(err) => {
                error!("Can not write file {} with err: {}", file.path, err);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }
        };
    }
}

fn process_output(p: &mut PathBuf, config: &Config, lang: &SupportedLang) {
    match lang {
        SupportedLang::Go => {
            let output_path = match &config.output.go {
                Some(o) => o.output.clone(),
                None => {
                    error!("You may want to generate Go code but forget to defiend it in config.");
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            *p = PathBuf::from(output_path).join(&p);
            p.set_extension("go");
        }
        SupportedLang::TypeScript => {
            let output_path = match &config.output.typescript {
                Some(o) => o.output.clone(),
                None => {
                    error!("You may want to generate TypeScript code but forget to defiend it in config.");
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            *p = PathBuf::from(output_path).join(&p);
            p.set_extension("ts");
        }
    }
}
