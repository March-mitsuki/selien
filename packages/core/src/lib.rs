mod compiler;
mod generator;
mod logger;
mod path;
mod prepare;
mod transformer;
mod types;

use log::{info, LevelFilter};
use types::cli::{Cli, Commands};
use wasm_bindgen::prelude::*;

use crate::types::lang::SupportedLang;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn is_dev() -> bool {
    match std::env::var("SELIEN_ENV") {
        Ok(value) => value == "dev",
        Err(_) => false,
    }
}

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    logger::main::init(LevelFilter::Debug).expect("Can not init logger.");

    let cli = Cli::get_parse();

    match cli.command {
        Commands::Gen(args) => {
            let (config, spec_list) = prepare::prepare(&args.config);

            match &args.output {
                Some(output) => {
                    let lang = SupportedLang::from(output);
                    lang.compiler(&config, &spec_list);
                }
                None => {
                    SupportedLang::all().iter().for_each(|lang| {
                        if lang.is_defined_in_config(&config) {
                            lang.compiler(&config, &spec_list);
                        }
                    });
                }
            }

            info!("Done.");
        }
    }
}
