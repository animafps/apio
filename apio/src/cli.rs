use std::{path::PathBuf, error::Error};
use apio_plugin::ArgType;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short, long, value_name = "FILE(S)", required=true)]
    pub input: Vec<PathBuf>,
    #[arg(short,long, value_name = "FILTERGRAPH")]
    pub filter: String,
}