use log::error;

use crate::generator::types::{ast_enum, ast_type_alias, AST};
use crate::types::lang::SupportedLang;
use crate::types::spec;

pub fn transformer(spec: &spec::Spec) -> Vec<AST> {
    let mut result: Vec<AST> = vec![];

    transform(spec, &mut result);

    result
}

fn transform(spec: &spec::Spec, ast_list: &mut Vec<AST>) {
    for def in spec.def.iter() {
        match def {
            spec::Def::TypeAlias(def) => {
                transfrom_type_alias(def, ast_list, None);
            }
            spec::Def::Enum(def) => {
                let members_type: ast_enum::MembersType = match def.r#type.as_str() {
                    "string" => ast_enum::MembersType::String,
                    "number" => ast_enum::MembersType::Number,
                    _ => {
                        error!(
                            "Syntax error: invalid type in enum definition: {}.",
                            def.identifier
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };

                let mut members: Vec<ast_enum::Member> = vec![];

                for member in def.members.iter() {
                    if let serde_yaml::Value::Mapping(m) = member {
                        for (key, value) in m {
                            match &members_type {
                                ast_enum::MembersType::String => {
                                    if !value.is_string() {
                                        error!(
                                            "Syntax error: string type enum member must be a string: {}.",
                                            def.identifier
                                        );
                                        if crate::is_dev() {
                                            panic!();
                                        } else {
                                            std::process::exit(1);
                                        }
                                    }

                                    members.push(ast_enum::Member {
                                        identifier: key.as_str().unwrap().to_string(),
                                        value: value.as_str().unwrap().to_string(),
                                    });
                                }
                                ast_enum::MembersType::Number => {
                                    if value.is_i64() {
                                        members.push(ast_enum::Member {
                                            identifier: key.as_str().unwrap().to_string(),
                                            value: format!("{}", value.as_i64().expect("Not i64.")),
                                        });
                                    } else if value.is_f64() {
                                        members.push(ast_enum::Member {
                                            identifier: key.as_str().unwrap().to_string(),
                                            value: format!("{}", value.as_f64().expect("Not f64.")),
                                        });
                                    } else {
                                        error!(
                                            "Syntax error: number type enum member must be a number: {}.",
                                            def.identifier
                                        );
                                        if crate::is_dev() {
                                            panic!();
                                        } else {
                                            std::process::exit(1);
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        error!(
                            "Syntax error: enum member must be a object (mapping): {}.",
                            def.identifier
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                }

                let enum_ast = ast_enum::EnumAst {
                    identifier: def.identifier.clone(),
                    r#type: members_type,
                    members,
                };

                ast_list.push(AST::Enum(enum_ast));
            }
        }
    }
}

fn transfrom_type_alias(
    def: &spec::TypeAliasDef,
    ast_list: &mut Vec<AST>,
    split: Option<&SupportedLang>,
) {
    let body_type = match def.body.get("type") {
        Some(r) => r,
        None => {
            error!(
                "Syntax error: missing type in definition: {}.",
                def.identifier
            );
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
    };

    match body_type.as_str() {
        Some("string") => {
            let mut body = ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                value: ast_type_alias::Keywords::String,
            });
            body = change_body_if_split(body, split);

            ast_list.push(AST::TypeAlias(ast_type_alias::TypeAliasAst {
                identifier: def.identifier.clone(),
                body,
            }))
        }
        Some("number") => {
            let mut body = ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                value: ast_type_alias::Keywords::Number,
            });
            body = change_body_if_split(body, split);

            ast_list.push(AST::TypeAlias(ast_type_alias::TypeAliasAst {
                identifier: def.identifier.clone(),
                body,
            }))
        }
        Some("boolean") => {
            let mut body = ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                value: ast_type_alias::Keywords::Boolean,
            });
            body = change_body_if_split(body, split);

            ast_list.push(AST::TypeAlias(ast_type_alias::TypeAliasAst {
                identifier: def.identifier.clone(),
                body,
            }))
        }
        Some("any") => {
            let mut body = ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                value: ast_type_alias::Keywords::Any,
            });
            body = change_body_if_split(body, split);

            ast_list.push(AST::TypeAlias(ast_type_alias::TypeAliasAst {
                identifier: def.identifier.clone(),
                body,
            }))
        }
        Some("object") => {
            let mut obj_node = ast_type_alias::ObjectNode { values: vec![] };
            let properties = match def.body.get("properties") {
                Some(r) => r,
                None => {
                    error!(
                        "Syntax error: missing properties in definition: {}.",
                        def.identifier
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            if let serde_yaml::Value::Mapping(map) = properties {
                visit_properties(map, &mut obj_node);
            } else {
                error!(
                    "Syntax error: invalid properties in definition: {}.",
                    def.identifier
                );
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }

            let mut body = ast_type_alias::Node::Object(obj_node);
            body = change_body_if_split(body, split);

            ast_list.push(AST::TypeAlias(ast_type_alias::TypeAliasAst {
                identifier: def.identifier.clone(),
                body,
            }));
        }
        Some("array") => {
            let mut array_node = ast_type_alias::ArrayNode {
                items: Box::new(ast_type_alias::Node::Array(ast_type_alias::ArrayNode {
                    items: Box::new(ast_type_alias::Node::new()),
                })),
            };

            let items = match def.body.get("items") {
                Some(r) => r,
                None => {
                    error!(
                        "Syntax error: missing items in array definition: {}.",
                        def.identifier
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            if let serde_yaml::Value::Mapping(map) = items {
                visit_items(map, &def.identifier, &mut array_node);
            } else {
                error!(
                    "Syntax error: invalid array items in definition: {}.",
                    def.identifier
                );
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }

            let mut body = ast_type_alias::Node::Array(array_node);
            body = change_body_if_split(body, split);

            ast_list.push(AST::TypeAlias(ast_type_alias::TypeAliasAst {
                identifier: def.identifier.clone(),
                body,
            }));
        }
        Some("literal") => {
            let value = match def.body.get("value") {
                Some(v) => v,
                None => {
                    error!(
                        "Syntax error: missing value in definition: {}.",
                        def.identifier
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            match value {
                serde_yaml::Value::String(v) => {
                    let mut body =
                        ast_type_alias::Node::StringLiteral(ast_type_alias::StringLiteralNode {
                            value: v.clone(),
                        });
                    body = change_body_if_split(body, split);

                    ast_list.push(AST::TypeAlias(ast_type_alias::TypeAliasAst {
                        identifier: def.identifier.clone(),
                        body,
                    }));
                }
                serde_yaml::Value::Number(v) => {
                    let mut body =
                        ast_type_alias::Node::NumberLiteral(ast_type_alias::NumberLiteralNode {
                            value: v.to_string(),
                        });
                    body = change_body_if_split(body, split);

                    ast_list.push(AST::TypeAlias(ast_type_alias::TypeAliasAst {
                        identifier: def.identifier.clone(),
                        body,
                    }));
                }
                _ => {
                    error!(
                        "Syntax error: invalid value in definition: {}.",
                        def.identifier
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            }
        }
        Some("$ref") => {
            let uri = match def.body.get("uri") {
                Some(v) => v,
                None => {
                    error!(
                        "Syntax error: missing uri in ref definition: {}.",
                        def.identifier
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            if let serde_yaml::Value::String(u) = uri {
                let name = u.split('/').last().unwrap().to_string();
                let path = u.split('#').next().unwrap().to_string();

                let mut body = ast_type_alias::Node::Ref(ast_type_alias::RefNode { name, path });
                body = change_body_if_split(body, split);

                ast_list.push(AST::TypeAlias(ast_type_alias::TypeAliasAst {
                    identifier: def.identifier.clone(),
                    body,
                }));
            } else {
                error!(
                    "Syntax error: invalid uri in definition: {}.",
                    def.identifier
                );
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }
        }
        Some("split") => {
            if split.is_some() {
                error!("Syntax error: split can only be used in root definition.");
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }

            let all_lang = SupportedLang::all();
            for lang in all_lang {
                let aliases = SupportedLang::get_alias(&lang);
                for alias in aliases {
                    if let Some(value) = def.body.get(alias) {
                        if let serde_yaml::Value::Mapping(map) = value {
                            let new_def = spec::TypeAliasDef {
                                identifier: def.identifier.to_string(),
                                body: map.clone(),
                            };
                            transfrom_type_alias(&new_def, ast_list, Some(&lang));
                        } else {
                            error!(
                                "Syntax error: invalid split in definition: {}.",
                                def.identifier
                            );
                            if crate::is_dev() {
                                panic!();
                            } else {
                                std::process::exit(1);
                            }
                        }
                    }
                }
            }
        }
        _ => {
            error!(
                "Syntax error: invalid type in definition: {}.",
                def.identifier
            );
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
    }
}

fn change_body_if_split(
    mut body: ast_type_alias::Node,
    split: Option<&SupportedLang>,
) -> ast_type_alias::Node {
    if let Some(lang) = split {
        match lang {
            SupportedLang::Go => {
                body = ast_type_alias::Node::Split(ast_type_alias::SplitNode {
                    go: Some(Box::new(body)),
                    typescript: None,
                });
            }
            SupportedLang::TypeScript => {
                body = ast_type_alias::Node::Split(ast_type_alias::SplitNode {
                    go: None,
                    typescript: Some(Box::new(body)),
                })
            }
        }
    }
    body
}

fn visit_properties(p: &serde_yaml::Mapping, node: &mut ast_type_alias::ObjectNode) {
    for (key, value) in p {
        let mut property = ast_type_alias::Property {
            identifier: key.as_str().unwrap().to_string(),
            body: ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                value: ast_type_alias::Keywords::String,
            }),
        };

        let property_type = match value.get("type") {
            Some(r) => r,
            None => {
                error!("Syntax error: missing type in definition: {:?}.", key);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }
        };
        match property_type.as_str() {
            Some("string") => {
                property.body = ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                    value: ast_type_alias::Keywords::String,
                });
            }
            Some("number") => {
                property.body = ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                    value: ast_type_alias::Keywords::Number,
                });
            }
            Some("boolean") => {
                property.body = ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                    value: ast_type_alias::Keywords::Boolean,
                });
            }
            Some("any") => {
                property.body = ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                    value: ast_type_alias::Keywords::Any,
                });
            }
            Some("object") => {
                let mut obj_node = ast_type_alias::ObjectNode { values: vec![] };
                let properties = match value.get("properties") {
                    Some(r) => r,
                    None => {
                        error!("Syntax error: missing properties in definition.");
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };

                if let serde_yaml::Value::Mapping(map) = properties {
                    visit_properties(map, &mut obj_node);
                } else {
                    error!("Syntax error: invalid properties in definition.");
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }

                property.body = ast_type_alias::Node::Object(obj_node);
            }
            Some("array") => {}
            _ => {
                error!(
                    "Syntax error: invalid type in object definition: {:?}.",
                    key
                );
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }
        }

        node.values.push(property);
    }
}

fn visit_items(i: &serde_yaml::Mapping, id: &String, node: &mut ast_type_alias::ArrayNode) {
    let items_type = match i.get("type") {
        Some(r) => r,
        None => {
            error!("Syntax error: missing type in array: {}.", id);
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
    };

    match items_type.as_str() {
        Some("string") => {
            node.items = Box::new(ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                value: ast_type_alias::Keywords::String,
            }));
        }
        Some("number") => {
            node.items = Box::new(ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                value: ast_type_alias::Keywords::Number,
            }));
        }
        Some("boolean") => {
            node.items = Box::new(ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                value: ast_type_alias::Keywords::Boolean,
            }));
        }
        Some("any") => {
            node.items = Box::new(ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                value: ast_type_alias::Keywords::Any,
            }));
        }
        Some("object") => {
            let mut obj_node = ast_type_alias::ObjectNode { values: vec![] };
            let properties = match i.get("properties") {
                Some(r) => r,
                None => {
                    error!("Syntax error: missing properties in definition: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            if let serde_yaml::Value::Mapping(map) = properties {
                visit_properties(map, &mut obj_node);
            } else {
                error!("Syntax error: invalid properties in definition: {}.", id);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }

            node.items = Box::new(ast_type_alias::Node::Object(obj_node));
        }
        Some("array") => {
            let mut _items_node = ast_type_alias::ArrayNode {
                items: Box::new(ast_type_alias::Node::new()),
            };

            let _items = match i.get("items") {
                Some(r) => r,
                None => {
                    error!("Syntax error: missing nested items in definition: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            if let serde_yaml::Value::Mapping(map) = _items {
                visit_items(map, id, &mut _items_node);
            } else {
                error!("Syntax error: invalid nested items in definition: {}.", id);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }

            node.items = Box::new(ast_type_alias::Node::Array(_items_node));
        }
        _ => {
            error!("Syntax error: invalid type in array definition: {}.", id);
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
    }
}
