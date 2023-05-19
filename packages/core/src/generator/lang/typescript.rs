use log::error;

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

pub fn generate_typescript(ast: &AST, imports: &mut Imports, tabsize: usize) -> String {
    let mut result = String::new();

    match ast {
        AST::Enum(enum_ast) => {
            let s = iterate_members(&enum_ast.members, &enum_ast.r#type, tabsize);
            result += &format!(
                "export enum {} {{\n{}\n}};\n",
                capitalize(&enum_ast.identifier),
                s
            );
        }
        AST::TypeAlias(type_alias_ast) => match &type_alias_ast.body {
            Node::StringLiteral(node) => {
                let s = format!(
                    "export type {} = \"{}\";\n",
                    capitalize(&type_alias_ast.identifier),
                    node.value
                );
                result += &s;
            }
            Node::NumberLiteral(node) => {
                let s = format!(
                    "export type {} = {};\n",
                    capitalize(&type_alias_ast.identifier),
                    node.value
                );
                result += &s;
            }
            Node::Keyword(node) => {
                let s = format!(
                    "export type {} = {};\n",
                    capitalize(&type_alias_ast.identifier),
                    node.value.to_string(SupportedLang::TypeScript)
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
                    "export type {} = {{\n{}\n}};\n",
                    capitalize(&type_alias_ast.identifier),
                    r
                );
            }
            Node::Array(node) => {
                let r = iterate_array(imports, &node.items, tabsize);
                result += &format!(
                    "export type {} = {}[];\n",
                    capitalize(&type_alias_ast.identifier),
                    r
                );
            }
            Node::Ref(node) => {
                let s = format!(
                    "export type {} = {};\n",
                    capitalize(&type_alias_ast.identifier),
                    capitalize(&node.name)
                );
                if !node.path.is_empty() {
                    imports.push(Import::Ref(RefImport {
                        name: capitalize(&node.name),
                        from: node.path.clone(),
                    }));
                }
                result += &s;
            }
            Node::Dyn(node) => {
                let s = format!(
                    "export type {} = {}\n",
                    capitalize(&type_alias_ast.identifier),
                    capitalize(&node.name)
                );
                imports.push(Import::Dyn(DynImport {
                    name: node.name.clone(),
                    from: node.from.clone(),
                }));

                result += &s;
            }
            Node::Split(split) => {
                if let Some(node) = &split.typescript {
                    let s = generate_typescript(
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
        },
    }

    result
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
        Node::StringLiteral(node) => {
            let mut s = format!("{}{}: \"{}\";", indent, p.identifier, node.value);
            if !is_last {
                s += "\n"
            }

            result += &s;
        }
        Node::NumberLiteral(node) => {
            let mut s = format!("{}{}: {};", indent, p.identifier, node.value);
            if !is_last {
                s += "\n"
            }

            result += &s;
        }
        Node::Keyword(node) => {
            let mut s = format!(
                "{}{}: {};",
                indent,
                p.identifier,
                node.value.to_string(SupportedLang::TypeScript)
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
                "{i}{id}: {{\n{re}\n{ii}}};",
                i = indent,
                id = p.identifier,
                ii = indent,
                re = r
            );
            if !is_last {
                s += "\n"
            }

            result += &s
        }
        Node::Array(node) => {
            let r = &iterate_array(imports, &node.items, tabsize);
            let mut s = format!("{}{}: {}[];", indent, p.identifier, r);
            if !is_last {
                s += "\n"
            }

            result += &s
        }
        Node::Ref(node) => {
            let mut s = format!("{}{}: {};", indent, p.identifier, capitalize(&node.name));
            if !is_last {
                s += "\n"
            }
            if !node.path.is_empty() {
                imports.push(Import::Ref(RefImport {
                    name: capitalize(&node.name),
                    from: node.path.clone(),
                }));
            }
            result += &s
        }
        Node::Dyn(node) => {
            let mut s = format!("{}{}: {};", indent, &p.identifier, &node.name,);
            if !is_last {
                s += "\n"
            }
            imports.push(Import::Dyn(DynImport {
                name: node.name.clone(),
                from: node.from.clone(),
            }));

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

fn iterate_array(imports: &mut Imports, n: &Node, tabsize: usize) -> String {
    let mut result = String::new();
    match n {
        Node::StringLiteral(node) => {
            let s = format!("\"{}\"", node.value);
            result += &s;
        }
        Node::NumberLiteral(node) => {
            let s = node.value.to_string();
            result += &s;
        }
        Node::Keyword(node) => {
            let s = node.value.to_string(SupportedLang::TypeScript);
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
            result += &format!("{{\n{}\n}}", r);
        }
        Node::Array(node) => {
            let r = &iterate_array(imports, &node.items, tabsize);
            result += &format!("{}[]", r);
        }
        Node::Ref(node) => {
            if !node.path.is_empty() {
                imports.push(Import::Ref(RefImport {
                    name: capitalize(&node.name),
                    from: node.path.clone(),
                }));
            }
            result += &capitalize(&node.name);
        }
        Node::Dyn(node) => {
            imports.push(Import::Dyn(DynImport {
                name: node.name.clone(),
                from: node.from.clone(),
            }));

            result += &node.name;
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
    tabsize: usize,
) -> String {
    let indent = " ".repeat(tabsize);
    let mut result = String::new();

    match m_type {
        ast_enum::MembersType::String => {
            for (idx, m) in members.iter().enumerate() {
                let is_last = idx == members.len() - 1;
                let mut s = format!("{}{} = \"{}\",", indent, m.identifier, m.value);
                if !is_last {
                    s += "\n"
                }

                result += &s
            }
        }
        ast_enum::MembersType::Number => {
            for (idx, m) in members.iter().enumerate() {
                let is_last = idx == members.len() - 1;
                let mut s = format!("{}{} = {},", indent, m.identifier, m.value);
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
        let result = ast.generator(&SupportedLang::TypeScript, &mut vec![], &DEFAULT_TABSIZE);
        insta::assert_yaml_snapshot!(result);
    }

    #[test]
    fn array() {
        let ast = for_test::array_ast();
        let result = ast.generator(&SupportedLang::TypeScript, &mut vec![], &DEFAULT_TABSIZE);
        insta::assert_yaml_snapshot!(result);
    }

    #[test]
    fn nested_array() {
        let ast = for_test::nested_array_ast();
        let result = ast.generator(&SupportedLang::TypeScript, &mut vec![], &DEFAULT_TABSIZE);
        insta::assert_yaml_snapshot!(result);
    }

    #[test]
    fn object_array() {
        let ast = for_test::object_array_ast();
        let result = ast.generator(&SupportedLang::TypeScript, &mut vec![], &DEFAULT_TABSIZE);
        insta::assert_yaml_snapshot!(result);
    }

    #[test]
    fn string_enum() {
        let ast = for_test::string_enum_ast();
        let result = ast.generator(&SupportedLang::TypeScript, &mut vec![], &DEFAULT_TABSIZE);
        insta::assert_yaml_snapshot!(result);
    }
}
