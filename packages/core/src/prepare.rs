use crate::generator::types::DEFAULT_TABSIZE;
use crate::path::process_path;
use crate::types::lang::SupportedLang;
use crate::types::{config, spec};
use log::error;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

pub fn prepare(input: &String) -> (config::Config, spec::SpecList) {
    let mut path = process_path(input);

    if path.is_dir() {
        path = path.join("selien.config.yaml");
    }

    let config: config::Config = parse_config_file(&path);

    let spec_list = parse_selien_file(config.spec.root.as_str());

    (config, spec_list)
}

fn parse_config_file<P: AsRef<Path>>(path: P) -> config::Config {
    let data: serde_yaml::Value = parse_yaml_from_file(path);

    let spec_data = data.get("spec").expect("Can not find spec in config file.");
    let output_data = data
        .get("output")
        .expect("Can not find output in config file.");

    let spec = config::Spec {
        root: spec_data
            .get("root")
            .expect("Can not find spec.root in config file.")
            .as_str()
            .expect("Can not parse spec.root to string.")
            .to_string(),
    };

    let mut output_go: Option<config::OutputGo> = None;
    let go_aliases = SupportedLang::get_alias(&SupportedLang::Go);
    for ga in go_aliases {
        if output_data.get(ga).is_some() {
            if let Some(ga_value) = output_data.get(ga) {
                let mod_name = ga_value
                    .get("mod_name")
                    .expect("selien golang config is missing in modName.")
                    .as_str()
                    .expect("Can not parse modName to string.")
                    .to_string();

                let output = ga_value
                    .get("output")
                    .expect("selien golang config is missing in output.")
                    .as_str()
                    .expect("Can not parse output to string.")
                    .to_string();

                let go_root = ga_value
                    .get("root")
                    .expect("selien golang config is missing in output.")
                    .as_str()
                    .expect("Can not parse output to string.")
                    .to_string();

                let tabsize = match ga_value.get("tabsize") {
                    Some(r) => r.as_u64().expect("Can not parse tabsize to u64.") as usize,
                    None => DEFAULT_TABSIZE.go,
                };

                output_go = Some(config::OutputGo {
                    mod_name,
                    root: go_root,
                    output,
                    tabsize,
                });
            }
        }
    }
    let mut output_ts: Option<config::OutputTypescript> = None;
    let ts_aliases = SupportedLang::get_alias(&SupportedLang::TypeScript);
    for ta in ts_aliases {
        if output_data.get(ta).is_some() {
            if let Some(ta_value) = output_data.get(ta) {
                let output = ta_value
                    .get("output")
                    .expect("selien typescript config is missing in output.")
                    .as_str()
                    .expect("Can not parse output to string.")
                    .to_string();

                let tabsize = match ta_value.get("tabsize") {
                    Some(r) => r.as_u64().expect("Can not parse tabsize to u64.") as usize,
                    None => DEFAULT_TABSIZE.typescript,
                };

                output_ts = Some(config::OutputTypescript { output, tabsize });
            }
        }
    }

    config::Config {
        spec,
        output: config::Output {
            go: output_go,
            typescript: output_ts,
        },
    }
}

fn parse_selien_file<D: AsRef<Path>>(dir: D) -> spec::SpecList {
    let mut result: Vec<spec::Spec> = Vec::new();

    visit_dirs(dir.as_ref(), &mut result);
    result
}

fn parse_yaml_from_file<T: DeserializeOwned, P: AsRef<Path>>(p: P) -> T {
    let string = match fs::read_to_string(&p) {
        Ok(result) => result,
        Err(err) => {
            error!(
                "Can not read file from given path: {:?}, {}",
                p.as_ref(),
                err
            );
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1)
            }
        }
    };

    match serde_yaml::from_str(&string) {
        Ok(result) => result,
        Err(err) => {
            error!("Can not parse yaml to given type: {}", err);
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1)
            }
        }
    }
}

fn visit_dirs(path: &Path, result: &mut Vec<spec::Spec>) {
    if !path.is_dir() {
        error!("{:?} is not a directory.", path);
        if crate::is_dev() {
            panic!();
        } else {
            std::process::exit(1);
        }
    }

    let root_dir = match fs::read_dir(path) {
        Ok(r) => r,
        Err(err) => {
            error!("Can not read directory: {}", err);
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
    };

    for entry in root_dir {
        let entry = entry.expect("Can not read entry.");
        let _path = entry.path();
        if _path.is_dir() {
            visit_dirs(&_path, result);
        } else {
            let mut spec = spec::Spec::new();
            let data: serde_yaml::Value = parse_yaml_from_file(&_path);

            if let serde_yaml::Value::Mapping(map) = data {
                for (key, value) in map {
                    match key.as_str() {
                        Some("selien-version") => {
                            spec.version = value.as_str().unwrap().to_string();
                            spec.path = _path.to_str().unwrap().to_string();
                        }
                        Some("definition") => {
                            spec.def = parse_def(&value);
                        }
                        _ => {
                            error!("Syntax error in selien file: {:?}", path);
                            if crate::is_dev() {
                                panic!();
                            } else {
                                std::process::exit(1);
                            }
                        }
                    }
                }
            } else {
                error!("Selien file must be a object (mapping): {:?}", path);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }
            result.push(spec);
        }
    }
}

fn parse_def(def: &serde_yaml::Value) -> Vec<spec::Def> {
    let mut result: Vec<spec::Def> = Vec::new();

    let mut parser = |m: &serde_yaml::Mapping| {
        for (key, value) in m {
            let declaration = match value.get("declaration") {
                Some(r) => r,
                None => {
                    error!(
                        "Syntax error: declaration not found in: {:?}",
                        key.as_str().unwrap()
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            match declaration.as_str() {
                Some("type-alias") => {
                    let body = value.get("body").expect("Syntax error: body not found");

                    result.push(spec::Def::TypeAlias(spec::TypeAliasDef {
                        identifier: match key.as_str() {
                            Some(v) => v.to_string(),
                            None => {
                                error!("type alias identifier err: {:?}", value);
                                if crate::is_dev() {
                                    panic!();
                                } else {
                                    std::process::exit(1);
                                }
                            }
                        },
                        body: match body {
                            serde_yaml::Value::Mapping(v) => v.to_owned(),
                            _ => {
                                error!(
                                    "type-alias body must be a object (mapping): {}",
                                    key.as_str().unwrap()
                                );
                                if crate::is_dev() {
                                    panic!();
                                } else {
                                    std::process::exit(1);
                                }
                            }
                        },
                    }))
                }
                Some("enum") => {
                    let members = value
                        .get("members")
                        .expect("Syntax error: members not found");
                    result.push(spec::Def::Enum(spec::EnumDef {
                        identifier: match key.as_str() {
                            Some(v) => v.to_string(),
                            None => {
                                error!("enum identifier err: {:?}", value);
                                if crate::is_dev() {
                                    panic!();
                                } else {
                                    std::process::exit(1);
                                }
                            }
                        },
                        r#type: match value.get("type") {
                            Some(v) => v.as_str().unwrap().to_string(),
                            None => {
                                error!("enum type err: {:?}", value);
                                if crate::is_dev() {
                                    panic!();
                                } else {
                                    std::process::exit(1);
                                }
                            }
                        },
                        members: match members {
                            serde_yaml::Value::Sequence(v) => v.to_owned(),
                            _ => {
                                error!(
                                    "enum member must be a array (list): {}",
                                    key.as_str().unwrap()
                                );
                                if crate::is_dev() {
                                    panic!();
                                } else {
                                    std::process::exit(1);
                                }
                            }
                        },
                    }))
                }
                _ => {
                    error!("Syntax error in selien file: {:?}", value);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            }
        }
    };

    match def {
        serde_yaml::Value::Mapping(map) => {
            parser(map);
        }
        _ => {
            error!("Selien file must be a object (mapping): {:?}", def);
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
    }

    result
}
