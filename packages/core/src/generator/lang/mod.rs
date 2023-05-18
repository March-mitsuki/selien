pub mod go;
pub mod typescript;

#[cfg(test)]
pub(self) mod for_test {
    use crate::generator::types::{
        ast_enum,
        ast_type_alias::{self, Keywords},
        AST,
    };

    pub fn nested_object_ast() -> AST {
        /*
        ts:
        export type TestType = {
            head: {
                cmd: string;
            };
            body: boolean;
        };

        go:
        type TestType struct {
            Head struct {
                cmd string `json:"cmd"`
            } `json:"head"`
            Body bool `json:"body"`
        }
        */

        let head = ast_type_alias::Property {
            identifier: String::from("head"),
            body: ast_type_alias::Node::Object(ast_type_alias::ObjectNode {
                values: vec![ast_type_alias::Property {
                    identifier: String::from("cmd"),
                    body: ast_type_alias::Node::StringLiteral(ast_type_alias::StringLiteralNode {
                        value: String::from("hello"),
                    }),
                }],
            }),
        };
        let body = ast_type_alias::Property {
            identifier: String::from("body"),
            body: ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                value: Keywords::Boolean,
            }),
        };
        let obj = ast_type_alias::ObjectNode {
            values: vec![head, body],
        };
        let ast = AST::TypeAlias(ast_type_alias::TypeAliasAst {
            identifier: String::from("TestAst"),
            body: ast_type_alias::Node::Object(obj),
        });

        ast
    }

    pub fn array_ast() -> AST {
        /*
        ts:
        export type TestArray = number[];

        go:
        type TestArray []int
        */

        let ast = AST::TypeAlias(ast_type_alias::TypeAliasAst {
            identifier: String::from("TestArray"),
            body: ast_type_alias::Node::Array(ast_type_alias::ArrayNode {
                items: Box::new(ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                    value: Keywords::Number,
                })),
            }),
        });

        ast
    }

    pub fn nested_array_ast() -> AST {
        /*
        ts:
        export type NestedArray = number[][];

        go:
        type NestedArray [][]int
        */

        let ast = AST::TypeAlias(ast_type_alias::TypeAliasAst {
            identifier: String::from("NestedArray"),
            body: ast_type_alias::Node::Array(ast_type_alias::ArrayNode {
                items: Box::new(ast_type_alias::Node::Array(ast_type_alias::ArrayNode {
                    items: Box::new(ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                        value: Keywords::Number,
                    })),
                })),
            }),
        });

        ast
    }

    pub fn object_array_ast() -> AST {
        /*
        ts:
        export type ObjectArray = {
            head: {
                cmd: string;
            };
            body: {
                count: number;
            };
        }[];

        go:
        type ObjectArray []struct {
            Head struct {
                Cmd string `json:"cmd"`
            } `json:"head"`
            Body struct {
                Count int `json:"count"`
            } `json:"body"`
        }
        */

        let head = ast_type_alias::Property {
            identifier: String::from("head"),
            body: ast_type_alias::Node::Object(ast_type_alias::ObjectNode {
                values: vec![ast_type_alias::Property {
                    identifier: String::from("cmd"),
                    body: ast_type_alias::Node::StringLiteral(ast_type_alias::StringLiteralNode {
                        value: String::from("hello"),
                    }),
                }],
            }),
        };
        let body = ast_type_alias::Property {
            identifier: String::from("body"),
            body: ast_type_alias::Node::Object(ast_type_alias::ObjectNode {
                values: vec![ast_type_alias::Property {
                    identifier: String::from("count"),
                    body: ast_type_alias::Node::Keyword(ast_type_alias::KeywordNode {
                        value: Keywords::Number,
                    }),
                }],
            }),
        };
        let obj = ast_type_alias::ObjectNode {
            values: vec![head, body],
        };
        let ast = AST::TypeAlias(ast_type_alias::TypeAliasAst {
            identifier: String::from("ObjectArray"),
            body: ast_type_alias::Node::Array(ast_type_alias::ArrayNode {
                items: Box::new(ast_type_alias::Node::Object(obj)),
            }),
        });

        ast
    }

    pub fn string_enum_ast() -> AST {
        /*
        ts:
        export enum StringEnum {
            Hello = "hello",
            World = "world",
        }

        go:
        type StringEnum string
        const (
            StringEnumHello StringEnum = "hello"
            StringEnumWorld StringEnum = "world"
        )
        */

        let ast = AST::Enum(ast_enum::EnumAst {
            identifier: String::from("StringEnum"),
            r#type: ast_enum::MembersType::String,
            members: vec![
                ast_enum::Member {
                    identifier: String::from("Hello"),
                    value: String::from("hello"),
                },
                ast_enum::Member {
                    identifier: String::from("World"),
                    value: String::from("world"),
                },
            ],
        });

        ast
    }
}
