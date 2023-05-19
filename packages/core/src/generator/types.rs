pub mod ast_type_alias {
    #[derive(Debug, Clone)]
    pub struct StringLiteralNode {
        pub value: String,
    }

    #[derive(Debug, Clone)]
    pub struct NumberLiteralNode {
        pub value: String,
    }

    #[derive(Debug, Clone)]
    pub enum Keywords {
        String,
        Number,
        Boolean,
        Any,
    }

    #[derive(Debug, Clone)]
    pub struct KeywordNode {
        pub value: Keywords,
    }

    #[derive(Debug, Clone)]
    pub struct ArrayNode {
        pub items: Box<Node>,
    }

    #[derive(Debug, Clone)]
    pub struct Property {
        pub identifier: String,
        pub body: Node,
    }

    #[derive(Debug, Clone)]
    pub struct ObjectNode {
        pub values: Vec<Property>,
    }

    #[derive(Debug, Clone)]
    pub struct RefNode {
        pub name: String,
        pub path: String,
    }

    #[derive(Debug, Clone)]
    pub struct DynNode {
        pub name: String,
        pub from: String,
    }

    #[derive(Debug, Clone)]
    pub struct SplitNode {
        pub typescript: Option<Box<Node>>,
        pub go: Option<Box<Node>>,
    }

    #[derive(Debug, Clone)]
    pub enum Node {
        StringLiteral(StringLiteralNode),
        NumberLiteral(NumberLiteralNode),
        Keyword(KeywordNode),
        Array(ArrayNode),
        Object(ObjectNode),
        Ref(RefNode),
        Dyn(DynNode),
        Split(SplitNode),
        Empty,
    }

    impl Node {
        pub fn new() -> Self {
            Self::Empty
        }
    }

    #[derive(Debug)]
    pub struct TypeAliasAst {
        pub identifier: String,
        pub body: Node,
    }
}

pub mod ast_enum {
    #[derive(Debug)]
    pub enum MembersType {
        String,
        Number,
    }

    #[derive(Debug)]
    pub struct Member {
        pub identifier: String,
        pub value: String,
    }

    pub type Members = Vec<Member>;

    #[derive(Debug)]
    pub struct EnumAst {
        pub identifier: String,
        pub r#type: MembersType,
        pub members: Members,
    }
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum AST {
    TypeAlias(ast_type_alias::TypeAliasAst),
    Enum(ast_enum::EnumAst),
}

#[derive(Debug)]
pub struct RefImport {
    pub name: String,
    pub from: String,
}

#[derive(Debug)]
pub struct DynImport {
    pub name: String,
    pub from: String,
}

#[derive(Debug)]
pub enum Import {
    Ref(RefImport),
    Dyn(DynImport),
}

pub type Imports = Vec<Import>;

pub struct TabSize {
    pub go: usize,
    pub typescript: usize,
}

pub const DEFAULT_TABSIZE: TabSize = TabSize {
    go: 4,
    typescript: 2,
};
