#![deny(
    warnings,
    missing_debug_implementations,
    rust_2018_idioms,
    nonstandard_style,
    future_incompatible,
    clippy::all,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic
)]
#![forbid(unsafe_code)]

mod io;
mod rrpl;

use std::path::PathBuf;

use clap::Parser;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    #[clap()]
    from: String,

    #[clap()]
    to: String,

    #[clap()]
    file: PathBuf,

    /// Rename original file to file~ before replacing
    #[clap(short, long)]
    backup: bool,

    /// Match case-insensitively
    #[clap(short, long)]
    ignore_case: bool,

    /// Disable logging to stdout/stderr
    #[clap(short, long)]
    quiet: bool,
}

fn initialize_logger(quiet: bool) {
    let level_filter = if quiet {
        LevelFilter::Error
    } else {
        LevelFilter::Info
    };

    TermLogger::init(
        level_filter,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
}

fn main() {
    let args = CliArgs::parse();

    initialize_logger(args.quiet);

    let content = io::read_file(&args.file);

    if args.backup {
        io::peform_backup(&args.file, &content);
    }

    let replacer = rrpl::make_text_replacer(args.ignore_case.into());
    let new_content = replacer.replace(&args.from, &args.to, &content);

    io::write_file(&args.file, &new_content);
}
