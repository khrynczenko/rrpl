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

use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

#[allow(clippy::struct_excessive_bools)]
// ^ This is one-to-one repr of CLI arguments so it is okay to do that
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    // REQUIRED ARGUMENTS
    #[clap()]
    from: String,

    #[clap()]
    to: String,

    #[clap(name = "FILE")]
    files: Vec<PathBuf>,

    // OPTIONS
    /// Rename original file to file~ before replacing
    #[clap(short, long)]
    backup: bool,

    /// Match case-insensitively
    #[clap(short, long)]
    ignore_case: bool,

    /// Prompt confirmation before changing the file
    #[clap(short, long)]
    prompt: bool,

    /// Disable logging to stdout/stderr
    #[clap(short, long)]
    quiet: bool,

    /// Match on word boundaries only
    #[clap(short, long)]
    whole_words: bool,
}

fn main() {
    let args = CliArgs::parse();

    initialize_logger(args.quiet);

    for path in &args.files {
        run_replacement(path, &args);
    }
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

fn run_replacement(path: &Path, args: &CliArgs) {
    let content = io::read_file(&path);
    if args.backup {
        io::peform_backup(&path, &content);
    }

    let replacer = rrpl::make_text_replacer(args.ignore_case.into(), args.whole_words.into());
    let (new_content, occurences) = replacer.replace(&args.from, &args.to, &content);
    log::info!("Found {} matches in {:#?}", occurences, path);

    let write_to_file_confirmed = if args.prompt {
        ask_for_confirmation()
    } else {
        true
    };

    if write_to_file_confirmed {
        io::write_file(&path, &new_content);
    }
}

fn ask_for_confirmation() -> bool {
    println!("Do you want to replace matches in this file? (y/n)");
    let mut answer = String::new();
    while !["y\n", "n\n"].contains(&answer.to_lowercase().as_str()) {
        answer.clear();
        std::io::stdin().read_line(&mut answer).unwrap_or_else(|e| {
            log::error!("{}", e);
            std::process::exit(1);
        });
    }

    match answer.as_ref() {
        "y\n" => true,
        "n\n" => false,
        _ => panic!("impossible answer"),
    }
}
