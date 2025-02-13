#![feature(extract_if)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::path::PathBuf;
use std::num::NonZero;

use clap::{Parser, Subcommand};

mod stamp;
mod info;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    quiet: u8,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// How many jobs to run in parallel.
    #[arg(short, long)]
    jobs: Option<NonZero<usize>>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Stamp(StampArgs),
    Verify(VerifyArgs),
    Info(InfoArgs),
}

#[derive(Parser, Debug)]
struct StampArgs {
    #[arg(name = "FILE", required = true, num_args = 1..)]
    files: Vec<PathBuf>,
}

#[derive(Parser, Debug)]
struct VerifyArgs {
    #[arg(short, long, group = "target")]
    file: Option<PathBuf>,

    #[arg(short, long, group = "target")]
    digest: Option<String>,

    #[arg(name = "TIMESTAMP", required = true, num_args(1..))]
    timestamps: Vec<PathBuf>,
}

#[derive(Parser, Debug)]
struct InfoArgs {
    #[arg(required = true)]
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let args = Cli::parse();

    let _verbosity: isize = dbg!((args.verbose as isize) - (args.quiet as isize));
    let jobs = args.jobs.unwrap_or_else(|| std::thread::available_parallelism().unwrap_or(NonZero::new(1).unwrap()));

    match args.command {
        Command::Stamp(args) => {
            stamp::stamp_command(args.files.into_iter(), jobs)?;
        },
        Command::Verify(args) => {
            todo!()
        }
        Command::Info(args) => {
            info::info_command(args.file)?;
        }
    }
    Ok(())
}
