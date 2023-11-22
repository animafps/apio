use std::path::PathBuf;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf
}