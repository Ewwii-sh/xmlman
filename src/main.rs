mod error;
mod opts;
mod parser;
mod transpiler;
mod checks;

use opts::XmlManArgs;
use parser::parse_xml;
use transpiler::convert_node;
use checks::run_all_checks;

use clap::Parser as ClapParser;
use log::{Level, error, info};
use std::fs;

/// [`FileInfo`] is structure for holding both the
/// file_path and xml content. It is used to send
/// info to the transpiler so that it can print pretty
/// errors whenever it need to.
pub struct FileInfo<'a> {
    file_path: &'a str,
    script: &'a str
}

fn main() {
    let args = XmlManArgs::parse();

    set_debug_levels(args.debug);

    // This is where we start transpiling to rhai.
    // Full transpile process:
    //
    // -----------      ---------------------------------
    // | Read fs | ---> | Parse with xml-rs & xmlparser | ---->
    // -----------      ---------------------------------     |
    //                              ---------------------------------------------------
    //              <-------------- | Convert XML AST to internal tree representation |
    //              |               ---------------------------------------------------
    //              |                                                 |
    //     ----------------------------------------                   |
    //     | Internal tree then converted to Rhai |    ----------------------------------
    //     ----------------------------------------    | Run checks to prevent mistakes |
    //                                                 ----------------------------------
    //
    for file in args.files {
        if !fs::exists(&file).expect("Could not check file existence") {
            error!("The file '{}' does not exist.", &file);
            return;
        }

        let xml_content = fs::read_to_string(&file).expect("Failed to read file");

        let file_info = FileInfo {
            file_path: &file,
            script: &xml_content
        };

        match parse_xml(&file_info) {
            Ok(ast) => {
                info!("Xml ast: {:#?}", ast);

                // convert to internal tree
                // the internal tree is a tree that
                // stands between xml and rhai.
                let internal_tree = convert_node(&ast, &file_info);
                info!("Internal AST: {:#?}", internal_tree);

                // If any check failed, return;
                if let Err(_) = run_all_checks(&internal_tree) {
                    return;
                }
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
