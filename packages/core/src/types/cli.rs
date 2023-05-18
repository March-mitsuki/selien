use clap::{arg, command, Args, Parser, Subcommand};

/// Selien a ssot-type-specification compiler.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generate code from your spec file.
    /// By default, Selien will generate code for all languages in your config file.
    Gen(GenArgs),
}

#[derive(Debug, Args)]
pub struct GenArgs {
    /// Output language. If not specified, Selien will generate code for all languages in your config file.
    #[arg(short, long)]
    pub output: Option<String>,

    /// Path to your config file. Default is current directory.
    #[arg(short, long, default_value = ".")]
    pub config: String,
}

impl Cli {
    pub fn get_parse() -> Self {
        Self::parse()
    }
}
