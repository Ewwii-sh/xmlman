use clap::Parser;

/// XmlMan: An elegant xml to rhai transpiler for ewwii.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct XmlManArgs {
    /// Files to transpile.
    pub files: Vec<String>,

    /// Path to output the transpiled files.
    #[clap(short, long)]
    pub out: Option<String>,

    /// Output pretty and formatted Rhai code.
    #[clap(short, long)]
    pub format: bool,

    /// Show debug logs.
    #[arg(long)]
    pub debug: bool,
}
