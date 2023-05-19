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
                items: Box::new(ast_type_alias::Node::new()),
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
            let v_uri = match def.body.get("uri") {
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
            if let serde_yaml::Value::String(uri) = v_uri {
                let name = uri.split('/').last().unwrap().to_string();
                let path = uri.split('#').next().unwrap().to_string();

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
        Some("$dyn") => {
            let v_name = match def.body.get("name") {
                Some(v) => v,
                None => {
                    error!(
                        "Syntax error: missing name in dyn definition: {}.",
                        def.identifier
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            let v_from = match def.body.get("from") {
                Some(v) => v,
                None => {
                    error!(
                        "Syntax error: missing from in dyn definition: {}.",
                        def.identifier
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            let name = match v_name {
                serde_yaml::Value::String(v) => v,
                _ => {
                    error!(
                        "Syntax error: invalid name in dyn definition: {}.",
                        def.identifier
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            let from = match v_from {
                serde_yaml::Value::String(v) => v,
                _ => {
                    error!(
                        "Syntax error: invalid from in dyn definition: {}.",
                        def.identifier
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            let mut body = ast_type_alias::Node::Dyn(ast_type_alias::DynNode {
                name: name.to_string(),
                from: from.to_string(),
            });
            body = change_body_if_split(body, split);

            ast_list.push(AST::TypeAlias(ast_type_alias::TypeAliasAst {
                identifier: def.identifier.clone(),
                body,
            }));
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
        Some("union") => {
            let mut union_node = ast_type_alias::UnionNode { types: vec![] };

            let v_types = match def.body.get("types") {
                Some(v) => v,
                None => {
                    error!(
                        "Syntax error: missing types in union definition: {}.",
                        def.identifier
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            if let serde_yaml::Value::Sequence(seq) = v_types {
                for v in seq {
                    if let serde_yaml::Value::Mapping(m) = v {
                        visit_union_types(m, &def.identifier, &mut union_node);
                    } else {
                        error!(
                            "Syntax error: invalid types in union definition: {}.",
                            def.identifier
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                }
            } else {
                error!(
                    "Syntax error: invalid types in union definition: {}.",
                    def.identifier
                );
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }

            let mut body = ast_type_alias::Node::Union(union_node);
            body = change_body_if_split(body, split);

            ast_list.push(AST::TypeAlias(ast_type_alias::TypeAliasAst {
                identifier: def.identifier.clone(),
                body,
            }));
        }
        _ => {
            error!(
                "Syntax error: invalid type {} in definition: {}.",
                body_type.as_str().expect("Syntax error in definition."),
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
                        error!(
                            "Syntax error: missing properties in definition: {:?}",
                            value
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
                        "Syntax error: invalid properties in definition: {:?}",
                        value
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }

                property.body = ast_type_alias::Node::Object(obj_node);
            }
            Some("array") => {
                let mut array_node = ast_type_alias::ArrayNode {
                    items: Box::new(ast_type_alias::Node::new()),
                };

                let items = match value.get("items") {
                    Some(r) => r,
                    None => {
                        error!(
                            "Syntax error: missing items in object array properties: {:?}",
                            value
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };

                if let serde_yaml::Value::Mapping(map) = items {
                    visit_items(map, key.as_str().unwrap(), &mut array_node);
                } else {
                    error!(
                        "Syntax error: invalid items in object array properties: {:?}",
                        value
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }

                property.body = ast_type_alias::Node::Array(array_node);
            }
            Some("literal") => {
                let literal_value = match value.get("value") {
                    Some(v) => v,
                    None => {
                        error!(
                            "Syntax error: missing value in object literal properties: {:?}.",
                            value
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };
                match literal_value {
                    serde_yaml::Value::String(v) => {
                        property.body = ast_type_alias::Node::StringLiteral(
                            ast_type_alias::StringLiteralNode { value: v.clone() },
                        );
                    }
                    serde_yaml::Value::Number(v) => {
                        property.body = ast_type_alias::Node::NumberLiteral(
                            ast_type_alias::NumberLiteralNode {
                                value: v.to_string(),
                            },
                        );
                    }
                    _ => {
                        error!(
                            "Syntax error: invalid value in object literal properties: {:?}.",
                            value
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
                let v_uri = match value.get("uri") {
                    Some(v) => v,
                    None => {
                        error!(
                            "Syntax error: missing uri in object ref proerties: {:?}.",
                            value
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };
                if let serde_yaml::Value::String(uri) = v_uri {
                    let name = uri.split('/').last().unwrap().to_string();
                    let path = uri.split('#').next().unwrap().to_string();

                    property.body =
                        ast_type_alias::Node::Ref(ast_type_alias::RefNode { name, path });
                } else {
                    error!(
                        "Syntax error: invalid uri in object ref proerties: {:?}.",
                        value
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            }
            Some("$dyn") => {
                let v_name = match value.get("name") {
                    Some(v) => v,
                    None => {
                        error!(
                            "Syntax error: missing name in object dyn properties: {:?}.",
                            value
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };
                let v_from = match value.get("from") {
                    Some(v) => v,
                    None => {
                        error!(
                            "Syntax error: missing from in object dyn properties: {:?}.",
                            value
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };

                let name = match v_name {
                    serde_yaml::Value::String(v) => v,
                    _ => {
                        error!(
                            "Syntax error: invalid name in object dyn properties: {:?}.",
                            value
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };
                let from = match v_from {
                    serde_yaml::Value::String(v) => v,
                    _ => {
                        error!(
                            "Syntax error: invalid from in object dyn properties: {:?}.",
                            value
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };

                property.body = ast_type_alias::Node::Dyn(ast_type_alias::DynNode {
                    name: name.to_string(),
                    from: from.to_string(),
                });
            }
            Some("union") => {
                let mut union_node = ast_type_alias::UnionNode { types: vec![] };

                let v_types = match value.get("types") {
                    Some(v) => v,
                    None => {
                        error!(
                            "Syntax error: missing types in object union properties: {:?}.",
                            value
                        );
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                };

                if let serde_yaml::Value::Sequence(types) = v_types {
                    for t in types {
                        if let serde_yaml::Value::Mapping(map) = t {
                            visit_union_types(map, key.as_str().unwrap(), &mut union_node);
                        } else {
                            error!(
                                "Syntax error: invalid types in object union properties: {:?}",
                                value
                            );
                            if crate::is_dev() {
                                panic!();
                            } else {
                                std::process::exit(1);
                            }
                        }
                    }
                } else {
                    error!(
                        "Syntax error: invalid types in object union properties: {:?}",
                        value
                    );
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }

                property.body = ast_type_alias::Node::Union(union_node);
            }
            Some("split") => {
                error!(
                    "Syntax error: split must used top-level. But you use it in a object {:?}.",
                    key
                );
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }
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

fn visit_items(i: &serde_yaml::Mapping, id: &str, node: &mut ast_type_alias::ArrayNode) {
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
            let mut arr_node = ast_type_alias::ArrayNode {
                items: Box::new(ast_type_alias::Node::new()),
            };

            let items = match i.get("items") {
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

            if let serde_yaml::Value::Mapping(map) = items {
                visit_items(map, id, &mut arr_node);
            } else {
                error!("Syntax error: invalid nested items in definition: {}.", id);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }

            node.items = Box::new(ast_type_alias::Node::Array(arr_node));
        }
        Some("lietral") => {
            let literal_value = match i.get("value") {
                Some(v) => v,
                None => {
                    error!("Syntax error: missing value in array literal item: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            match literal_value {
                serde_yaml::Value::String(v) => {
                    node.items = Box::new(ast_type_alias::Node::StringLiteral(
                        ast_type_alias::StringLiteralNode { value: v.clone() },
                    ));
                }
                serde_yaml::Value::Number(v) => {
                    node.items = Box::new(ast_type_alias::Node::NumberLiteral(
                        ast_type_alias::NumberLiteralNode {
                            value: v.to_string(),
                        },
                    ));
                }
                _ => {
                    error!("Syntax error: invalid value in array literal item: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            }
        }
        Some("$ref") => {
            let v_uri = match i.get("uri") {
                Some(v) => v,
                None => {
                    error!("Syntax error: missing uri in array ref items: {:?}.", i);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            if let serde_yaml::Value::String(uri) = v_uri {
                let name = uri.split('/').last().unwrap().to_string();
                let path = uri.split('#').next().unwrap().to_string();

                node.items = Box::new(ast_type_alias::Node::Ref(ast_type_alias::RefNode {
                    name,
                    path,
                }));
            } else {
                error!("Syntax error: invalid uri in array ref items: {:?}.", i);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }
        }
        Some("$dyn") => {
            let v_name = match i.get("name") {
                Some(v) => v,
                None => {
                    error!("Syntax error: missing name in array dyn items: {:?}.", i);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            let v_from = match i.get("from") {
                Some(v) => v,
                None => {
                    error!("Syntax error: missing from in array dyn items: {:?}.", i);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            let name = match v_name {
                serde_yaml::Value::String(v) => v,
                _ => {
                    error!("Syntax error: invalid name in array dyn items: {:?}.", i);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            let from = match v_from {
                serde_yaml::Value::String(v) => v,
                _ => {
                    error!("Syntax error: invalid from in array dyn items: {:?}.", i);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            node.items = Box::new(ast_type_alias::Node::Dyn(ast_type_alias::DynNode {
                name: name.to_string(),
                from: from.to_string(),
            }));
        }
        Some("union") => {
            let mut union_node = ast_type_alias::UnionNode { types: vec![] };

            let v_types = match i.get("types") {
                Some(r) => r,
                None => {
                    error!("Syntax error: missing types in union definition: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            if let serde_yaml::Value::Sequence(seq) = v_types {
                for t in seq {
                    if let serde_yaml::Value::Mapping(map) = t {
                        visit_union_types(map, id, &mut union_node);
                    } else {
                        error!("Syntax error: invalid types in union definition: {}.", id);
                        if crate::is_dev() {
                            panic!();
                        } else {
                            std::process::exit(1);
                        }
                    }
                }
            } else {
                error!("Syntax error: invalid types in union definition: {}.", id);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }

            node.items = Box::new(ast_type_alias::Node::Union(union_node));
        }
        Some("split") => {
            error!(
                "Syntax error: split must used top-level. But you use it in a array {}.",
                id
            );
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
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

fn visit_union_types(t: &serde_yaml::Mapping, id: &str, node: &mut ast_type_alias::UnionNode) {
    let tp = match t.get("type") {
        Some(v) => v,
        None => {
            error!("Syntax error: missing type in union definition: {}.", id);
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
    };

    match tp.as_str() {
        Some("string") => {
            node.types
                .push(ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                    value: ast_type_alias::Keywords::String,
                }));
        }
        Some("number") => {
            node.types
                .push(ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                    value: ast_type_alias::Keywords::Number,
                }));
        }
        Some("boolean") => {
            node.types
                .push(ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                    value: ast_type_alias::Keywords::Boolean,
                }));
        }
        Some("any") => {
            node.types
                .push(ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                    value: ast_type_alias::Keywords::Any,
                }));
        }
        Some("object") => {
            let mut obj_node = ast_type_alias::ObjectNode { values: vec![] };
            let properties = match t.get("properties") {
                Some(v) => v,
                None => {
                    error!(
                        "Syntax error: missing properties in object definition: {}.",
                        id
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
                    "Syntax error: invalid properties in object definition: {}.",
                    id
                );
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }

            node.types.push(ast_type_alias::Node::Object(obj_node));
        }
        Some("array") => {
            let mut arr_node = ast_type_alias::ArrayNode {
                items: Box::new(ast_type_alias::Node::new()),
            };
            let items = match t.get("items") {
                Some(v) => v,
                None => {
                    error!("Syntax error: missing items in union.array: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            if let serde_yaml::Value::Mapping(map) = items {
                visit_items(map, id, &mut arr_node);
            } else {
                error!("Syntax error: invalid items in array definition: {}.", id);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }

            node.types.push(ast_type_alias::Node::Array(arr_node));
        }
        Some("literal") => {
            let literal_value = match t.get("value") {
                Some(v) => v,
                None => {
                    error!("Syntax error: missing value in union.literal: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            match literal_value {
                serde_yaml::Value::String(v) => {
                    node.types.push(ast_type_alias::Node::StringLiteral(
                        ast_type_alias::StringLiteralNode { value: v.clone() },
                    ));
                }
                serde_yaml::Value::Number(v) => {
                    node.types.push(ast_type_alias::Node::NumberLiteral(
                        ast_type_alias::NumberLiteralNode {
                            value: v.to_string(),
                        },
                    ));
                }
                _ => {
                    error!("Syntax error: invalid value in union.literal: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            }
        }
        Some("$ref") => {
            let v_uri = match t.get("uri") {
                Some(v) => v,
                None => {
                    error!("Syntax error: missing uri in union.$ref: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            if let serde_yaml::Value::String(uri) = v_uri {
                let name = uri.split('/').last().unwrap().to_string();
                let path = uri.split('#').next().unwrap().to_string();

                node.types
                    .push(ast_type_alias::Node::Ref(ast_type_alias::RefNode {
                        name,
                        path,
                    }));
            } else {
                error!("Syntax error: invalid uri in union.$ref: {}.", id);
                if crate::is_dev() {
                    panic!();
                } else {
                    std::process::exit(1);
                }
            }
        }
        Some("$dyn") => {
            let v_name = match t.get("name") {
                Some(v) => v,
                None => {
                    error!("Syntax error: missing name in union.$dyn: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            let v_from = match t.get("from") {
                Some(v) => v,
                None => {
                    error!("Syntax error: missing from in union.$dyn: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            let name = match v_name {
                serde_yaml::Value::String(v) => v,
                _ => {
                    error!("Syntax error: invalid name in union.$dyn: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };
            let from = match v_from {
                serde_yaml::Value::String(v) => v,
                _ => {
                    error!("Syntax error: invalid from in union.$dyn: {}.", id);
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            };

            node.types
                .push(ast_type_alias::Node::Dyn(ast_type_alias::DynNode {
                    name: name.to_string(),
                    from: from.to_string(),
                }));
        }
        Some("uniton") => {
            error!("Syntax error: use uniton nested in a union: {}.", id);
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
        Some("split") => {
            error!(
                "Syntax error: split must used top-level. But you use it in a union: {}.",
                id
            );
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
        _ => {
            error!("Syntax error: invalid type in union definition: {}.", id);
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
    }
}
