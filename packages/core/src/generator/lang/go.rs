use std::path::PathBuf;

use log::{error, warn};

use super::super::types::{
    ast_enum,
    ast_type_alias::{Node, Property, TypeAliasAst},
    AST,
};
use crate::{
    generator::{
        types::{DynImport, Import, Imports, RefImport},
        utils::capitalize,
    },
    types::lang::SupportedLang,
};

/// Make sure to due with golang imports. (when $ref used)
///
/// golang imports used like this:
/// ```go
/// import (
///    "github.com/yourname/yourpackage"
/// )
///
/// // when use import, must use package name.
/// type YourType struct {
///   YourField yourpackage.YourType
/// }
/// ```
///
/// So we add a **token** when $ref used. Syntax is `[selien-ref]path/to/ref/yourpackage/file.yaml[selien-ref]`
///
/// This token must be replace to actual package name when compile phase.
pub fn generate_go(ast: &AST, imports: &mut Imports, tabsize: usize) -> String {
    match ast {
        AST::Enum(enum_ast) => {
            let mut result = String::from("\n");

            match &enum_ast.r#type {
                ast_enum::MembersType::String => {
                    result += &format!("type {} string\n", capitalize(&enum_ast.identifier));
                }
                ast_enum::MembersType::Number => {
                    result += &format!("type {} int\n", capitalize(&enum_ast.identifier));
                }
            }

            let s = iterate_members(
                &enum_ast.members,
                &enum_ast.r#type,
                &enum_ast.identifier,
                tabsize,
            );
            result += &format!("const (\n{}\n)\n\n", s);

            result
        }
        AST::TypeAlias(type_alias_ast) => {
            let mut result = String::new();
            match &type_alias_ast.body {
                Node::StringLiteral(_) => {
                    let s = format!("type {} string\n", capitalize(&type_alias_ast.identifier));
                    result += &s;
                }
                Node::NumberLiteral(_) => {
                    let s = format!("type {} int\n", capitalize(&type_alias_ast.identifier));
                    result += &s;
                }
                Node::Keyword(node) => {
                    let s = format!(
                        "type {} {}\n",
                        capitalize(&type_alias_ast.identifier),
                        node.value.to_string(SupportedLang::Go)
                    );
                    result += &s;
                }
                Node::Object(node) => {
                    let r: String = node
                        .values
                        .iter()
                        .enumerate()
                        .map(|(idx, v)| {
                            let is_last = idx == node.values.len() - 1;
                            iterate_properties(imports, v, 1, tabsize, is_last)
                        })
                        .collect();
                    result += &format!(
                        "type {} struct {{\n{}\n}}\n",
                        capitalize(&type_alias_ast.identifier),
                        r
                    );
                }
                Node::Array(node) => {
                    let r = iterate_array(imports, &node.items, tabsize);
                    result += &format!("type {} []{}\n", capitalize(&type_alias_ast.identifier), r);
                }
                Node::Ref(node) => {
                    let mut s = format!(
                        "type {} {}\n",
                        capitalize(&type_alias_ast.identifier),
                        capitalize(&node.name)
                    );

                    if !node.path.is_empty() {
                        let ref_token = format!("[selien-ref]{}[selien-ref]", node.path);
                        s = format!(
                            "type {} {}{}\n",
                            capitalize(&type_alias_ast.identifier),
                            ref_token,
                            capitalize(&node.name)
                        );

                        imports.push(Import::Ref(RefImport {
                            name: capitalize(&node.name),
                            from: PathBuf::from(&node.path),
                        }));
                    }

                    result += &s;
                }
                Node::Dyn(node) => {
                    let fp = PathBuf::from(&node.from);
                    let s = format!(
                        "type {} {}.{}\n",
                        capitalize(&type_alias_ast.identifier),
                        fp.file_name().unwrap().to_str().unwrap(),
                        capitalize(&node.name)
                    );
                    imports.push(Import::Dyn(DynImport {
                        name: node.name.clone(),
                        from: node.from.clone(),
                    }));

                    result += &s;
                }
                Node::Union(_) => {
                    let id = capitalize(&type_alias_ast.identifier);
                    warn!(
                        "Union type '{}' using in golang. interface{{}} type will be generated.",
                        &id
                    );
                    let s = format!("type {} interface{{}}\n", id);
                    result += &s;
                }
                Node::Split(split) => {
                    if let Some(node) = &split.go {
                        let s = generate_go(
                            &AST::TypeAlias(TypeAliasAst {
                                identifier: type_alias_ast.identifier.clone(),
                                body: node.as_ref().clone(),
                            }),
                            imports,
                            tabsize,
                        );
                        result += &s;
                    }
                }
                Node::Empty => {
                    error!("Empty node.");
                    if crate::is_dev() {
                        panic!();
                    } else {
                        std::process::exit(1);
                    }
                }
            }
            result
        }
    }
}

fn iterate_properties(
    imports: &mut Imports,
    p: &Property,
    deepth: usize,
    tabsize: usize,
    is_last: bool,
) -> String {
    let indent = " ".repeat(deepth * tabsize);
    let mut result = String::new();
    match &p.body {
        Node::StringLiteral(_) => {
            let mut s = format!(
                "{}{} string `json:\"{}\"`",
                indent,
                capitalize(&p.identifier),
                &p.identifier
            );
            if !is_last {
                s += "\n"
            }
            result += &s;
        }
        Node::NumberLiteral(_) => {
            let mut s = format!(
                "{}{} int `json:\"{}\"`",
                indent,
                capitalize(&p.identifier),
                &p.identifier
            );
            if !is_last {
                s += "\n"
            }
            result += &s;
        }
        Node::Keyword(node) => {
            let mut s = format!(
                "{}{} {} `json:\"{}\"`",
                indent,
                capitalize(&p.identifier),
                node.value.to_string(SupportedLang::Go),
                &p.identifier
            );
            if !is_last {
                s += "\n"
            }
            result += &s;
        }
        Node::Object(node) => {
            let r: String = node
                .values
                .iter()
                .enumerate()
                .map(|(idx, v)| {
                    let is_last = idx == node.values.len() - 1;
                    iterate_properties(imports, v, deepth + 1, tabsize, is_last)
                })
                .collect();
            let mut s = format!(
                "{i}{id} struct {{\n{re}\n{ii}}} `json:\"{j}\"`",
                i = indent,
                id = capitalize(&p.identifier),
                ii = indent,
                re = r,
                j = &p.identifier
            );
            if !is_last {
                s += "\n"
            }
            result += &s
        }
        Node::Array(node) => {
            let r = iterate_array(imports, &node.items, tabsize + 1);
            let mut s = format!(
                "{}{} []{} `json:\"{}\"`",
                indent,
                capitalize(&p.identifier),
                r,
                &p.identifier
            );
            if !is_last {
                s += "\n"
            }
            result += &s
        }
        Node::Ref(node) => {
            let mut s = format!(
                "{}{} {} `json:\"{}\"`",
                indent,
                capitalize(&p.identifier),
                capitalize(&node.name),
                &p.identifier
            );
            if !node.path.is_empty() {
                let ref_token = format!("[selien-ref]{}[selien-ref]", node.path);
                s = format!(
                    "{}{} {}{} `json:\"{}\"`",
                    indent,
                    capitalize(&p.identifier),
                    ref_token,
                    capitalize(&node.name),
                    &p.identifier
                );

                imports.push(Import::Ref(RefImport {
                    name: capitalize(&node.name),
                    from: PathBuf::from(&node.path),
                }));
            }

            if !is_last {
                s += "\n"
            }
            result += &s
        }
        Node::Dyn(node) => {
            let fp = PathBuf::from(&node.from);
            let mut s = format!(
                "{}{} {}.{} `json:\"{}\"`",
                indent,
                capitalize(&p.identifier),
                fp.file_name().unwrap().to_str().unwrap(),
                capitalize(&node.name),
                &p.identifier,
            );
            if !is_last {
                s += "\n"
            }
            imports.push(Import::Dyn(DynImport {
                name: node.name.clone(),
                from: node.from.clone(),
            }));

            result += &s;
        }
        Node::Union(_) => {
            let id = capitalize(&p.identifier);
            warn!(
                "Union type '{}' using in golang object type. interface{{}} type will be generated.",
                &id
            );
            let mut s = format!(
                "{}{} interface{{}} `json:\"{}\"`",
                indent, id, &p.identifier
            );
            if !is_last {
                s += "\n"
            }
            result += &s;
        }
        Node::Split(_) => {
            error!("Split-type can only use on top-level.");
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
        Node::Empty => {
            error!("Empty node.");
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
    }
    result
}

fn iterate_array(imports: &mut Imports, node: &Node, tabsize: usize) -> String {
    let mut result = String::new();
    match node {
        Node::StringLiteral(_) => result += "string",
        Node::NumberLiteral(_) => result += "int",
        Node::Keyword(node) => result += &node.value.to_string(SupportedLang::Go),
        Node::Object(node) => {
            let r: String = node
                .values
                .iter()
                .enumerate()
                .map(|(idx, v)| {
                    let is_last = idx == node.values.len() - 1;
                    iterate_properties(imports, v, 1, tabsize, is_last)
                })
                .collect();
            result += &format!("struct {{\n{}\n}}", r);
        }
        Node::Array(node) => {
            let r = iterate_array(imports, &node.items, tabsize);
            result += &format!("[]{}", r);
        }
        Node::Ref(node) => {
            let mut s = capitalize(&node.name);
            if !node.path.is_empty() {
                let ref_token = format!("[selien-ref]{}[selien-ref]", node.path);
                s = format!("{}{}", ref_token, capitalize(&node.name));

                imports.push(Import::Ref(RefImport {
                    name: capitalize(&node.name),
                    from: PathBuf::from(&node.path),
                }));
            }

            result += &s
        }
        Node::Dyn(node) => {
            let fp = PathBuf::from(&node.from);
            let s = format!(
                "{}.{}",
                fp.file_name().unwrap().to_str().unwrap(),
                capitalize(&node.name),
            );
            imports.push(Import::Dyn(DynImport {
                name: node.name.clone(),
                from: node.from.clone(),
            }));

            result += &s;
        }
        Node::Union(_) => {
            warn!("Union type using in golang array type. interface{{}} type will be generated.");
            result += "interface{}"
        }
        Node::Split(_) => {
            error!("Split-type can only use on top-level.");
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
        Node::Empty => {
            error!("Empty node.");
            if crate::is_dev() {
                panic!();
            } else {
                std::process::exit(1);
            }
        }
    }
    result
}

fn iterate_members(
    members: &ast_enum::Members,
    m_type: &ast_enum::MembersType,
    type_id: &str,
    tabsize: usize,
) -> String {
    let indent = " ".repeat(tabsize);
    let mut result = String::new();

    match m_type {
        ast_enum::MembersType::String => {
            for (idx, m) in members.iter().enumerate() {
                let is_last = idx == members.len() - 1;
                let mut s = format!(
                    "{}{} {} = \"{}\"",
                    indent,
                    capitalize(&m.identifier),
                    capitalize(type_id),
                    m.value
                );
                if !is_last {
                    s += "\n"
                }

                result += &s
            }
        }
        ast_enum::MembersType::Number => {
            for (idx, m) in members.iter().enumerate() {
                let is_last = idx == members.len() - 1;
                let mut s = format!(
                    "{}{} {} = {}",
                    indent,
                    capitalize(&m.identifier),
                    capitalize(type_id),
                    m.value
                );
                if !is_last {
                    s += "\n"
                }

                result += &s
            }
        }
    }

    result
}

// unit test here
#[cfg(test)]
mod test {
    use crate::{
        generator::{lang::for_test, types::DEFAULT_TABSIZE},
        types::lang::SupportedLang,
    };

    #[test]
    fn nested_object() {
        let ast = for_test::nested_object_ast();
        let result = ast.generator(&SupportedLang::Go, &mut vec![], &DEFAULT_TABSIZE);
        insta::assert_yaml_snapshot!(result);
    }

    #[test]
    fn nested_array() {
        let ast = for_test::nested_array_ast();
        let result = ast.generator(&SupportedLang::Go, &mut vec![], &DEFAULT_TABSIZE);
        insta::assert_yaml_snapshot!(result);
    }

    #[test]
    fn object_array() {
        let ast = for_test::object_array_ast();
        let result = ast.generator(&SupportedLang::Go, &mut vec![], &DEFAULT_TABSIZE);
        insta::assert_yaml_snapshot!(result);
    }

    #[test]
    fn string_enum() {
        let ast = for_test::string_enum_ast();
        let result = ast.generator(&SupportedLang::Go, &mut vec![], &DEFAULT_TABSIZE);
        insta::assert_yaml_snapshot!(result);
    }
}
