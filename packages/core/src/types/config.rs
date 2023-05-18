use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub root: String,
}

#[derive(Debug, Deserialize)]
pub struct OutputGo {
    #[serde(rename = "modName")]
    pub mod_name: String,
    // #[serde(rename = "modRoot")]
    // pub mod_root: String,
    pub output: String,
    pub tabsize: usize,
}

#[derive(Debug, Deserialize)]
pub struct OutputTypescript {
    pub output: String,
    pub tabsize: usize,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub go: Option<OutputGo>,
    pub typescript: Option<OutputTypescript>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub spec: Spec,
    pub output: Output,
}
