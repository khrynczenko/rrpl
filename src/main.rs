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

use rrpl::{StdTextReplacer, TextReplacer};

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
}

fn main() {
    let args = CliArgs::parse();

    let content = io::read_file(&args.file);

    if args.backup {
        io::peform_backup(&args.file, &content);
    }

    let replacer = StdTextReplacer {};
    let new_content = replacer.replace(&args.from, &args.to, &content);

    io::write_file(&args.file, &new_content);
}
