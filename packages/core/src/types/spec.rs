use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TypeAliasDef {
    pub identifier: String,
    pub body: serde_yaml::Mapping,
}

#[derive(Debug, Deserialize)]
pub struct EnumDef {
    pub identifier: String,
    pub r#type: String,
    pub members: serde_yaml::Sequence,
}

#[derive(Debug, Deserialize)]
pub enum Def {
    TypeAlias(TypeAliasDef),
    Enum(EnumDef),
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub version: String,
    pub path: String,
    pub def: Vec<Def>,
}

impl Spec {
    pub fn new() -> Self {
        Self {
            version: String::new(),
            path: String::new(),
            def: vec![],
        }
    }
}

pub type SpecList = Vec<Spec>;
