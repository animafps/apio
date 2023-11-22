use std::fs::File;

use apio_plugin::PluginManager;
use clap::Parser;
use walkdir::WalkDir;
use y4m::decode;

mod cli;
fn main() -> Result<(),&'static str> {
    let cli = cli::Args::parse();
    if cli.input.extension().unwrap() != "y4m" {
        return Err("input not y4m");
    }
    let input = File::open(cli.input).unwrap();
    let indecoder = decode(input).unwrap();
    let mut plugin_manager = PluginManager::new();
    let lib_path = option_env!("APIOLIBPATH").or_else(|| {
        return Some("/usr/lib/apio")
    }).unwrap();
    for entry in WalkDir::new(lib_path).follow_links(true).min_depth(1) {
        let entry = entry.unwrap();
        let extension = entry.path().extension().unwrap();
        if extension == "so" ||  extension == "dll" {
            unsafe { plugin_manager.load_plugin(entry.path()) }.unwrap();
        }
    }
    let example = &plugin_manager.plugins[0];
    Ok(())
}
