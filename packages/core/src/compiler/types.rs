use crate::types::config::Config;

#[derive(Debug)]
pub struct OutputFile {
    pub path: String,
    pub content: String,
}

pub struct Output<'a> {
    pub config: &'a Config,
    pub files: Vec<OutputFile>,
}
