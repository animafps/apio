use std::fs::File;
use apio_lib::{PluginManager, parse_filtergraph};
use clap::Parser;
use walkdir::WalkDir;
use y4m::decode;

mod cli;
fn main() -> Result<(),&'static str> {
    let cli = cli::Args::parse();
    let file = &cli.input[0];
    let filtergraph = parse_filtergraph(&cli.filter).unwrap();
    if file.extension().unwrap() != "y4m" {
        return Err("input not y4m");
    }
    let input = File::open(file).unwrap();
    let mut decoder = decode(input).unwrap();
    let mut plugin_manager = PluginManager::new();
    let lib_path = option_env!("APIOLIBPATH").or_else(|| {
        Some("/usr/lib/apio")
    }).unwrap();
    for entry in WalkDir::new(lib_path).follow_links(true).min_depth(1) {
        let entry = entry.unwrap();
        let extension = entry.path().extension().unwrap();
        if extension == "so" ||  extension == "dll" {
            // need to only load plugins being used
            unsafe { plugin_manager.load_plugin(entry.path()) }.unwrap();
        }
    }
    for (_num, frame) in decoder.read_frame().into_iter().enumerate() {
        for chain in filtergraph.clone() {
            let mut current_frames = vec![&frame];
            for current_frame in current_frames {
                for (filter, args) in &chain {
                    let plugin = plugin_manager.get_plugin(filter).unwrap();
                    current_frames = plugin.get_frame(&current_frame, args);
                    
                }
            }
        }
    }
    Ok(())
}
