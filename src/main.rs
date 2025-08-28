mod error;
mod opts;
mod parser;
mod transpiler;

use opts::AppArgs;
use parser::parse_xml;

use clap::Parser as ClapParser;
use log::{Level, error, info};
use std::fs;

fn main() {
    let args = AppArgs::parse();

    set_debug_levels(args.debug);

    for file in args.files {
        if !fs::exists(&file).expect("Could not check file existence") {
            error!("The file '{}' does not exist.", &file);
            return;
        }

        let xml_content = fs::read_to_string(&file).expect("Failed to read file");

        match parse_xml(&xml_content, &file) {
            Ok(ast) => {
                info!("{:#?}", ast);
            }
            Err(_) => return,
        }
    }
}

fn set_debug_levels(debug_mode: bool) {
    let mut builder = env_logger::Builder::from_default_env();

    if debug_mode {
        builder
            .filter_level(log::LevelFilter::Debug)
            .format_timestamp_secs()
            .format_module_path(true)
            .format_level(true);
    } else {
        builder
            .format(|buf, record| {
                use std::io::Write;

                match record.level() {
                    Level::Warn => writeln!(buf, "[WARN] {}", record.args()),
                    Level::Error => writeln!(buf, "[ERROR] {}", record.args()),
                    _ => writeln!(buf, "{}", record.args()),
                }
            })
            .filter_level(log::LevelFilter::Info);
    }

    builder.init();
}
