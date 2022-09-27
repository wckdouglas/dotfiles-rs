pub mod cli;
pub mod path;

use cli::command;
use log::info;
use path::home_path;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{copy, create_dir_all};
use std::path::Path;
use std::string::String;

const DOTFILE_YAML: &str = "";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DotFiles {
    dotfiles: Vec<String>,
}

pub fn run() -> Result<u8, String> {
    let command_args = command()?;
    let yaml_fn: &str = command_args
        .value_of::<&str>("dotfile_yaml")
        .ok_or("Cannot find the provided dotfile yaml file")?;

    let dotfile_list: Vec<String> = read_yaml(yaml_fn)?;

    match command_args.subcommand() {
        Some((subcommand, sub_m)) => match subcommand {
            "save" => {
                let at_id: &str = sub_m
                    .value_of::<&str>("dest_dir")
                    .ok_or("No dest-dir provided")?;
                save(dotfile_list, destination_dir.to_string());
            }
        },
        _ => (),
    }
    Ok(0)
}

fn save(dotfile_list: Vec<String>, destination_dir: String) -> Result<Vec<u8>, String> {
    dotfile_list
        .into_par_iter()
        .map(|dotfile| {
            let orig_file = format!("{}/{}", home_path, dotfile);
            let dest_file = format!("{}/{}", destination_dir, dotfile);
            let orig_path = path::new(orig_file);
            let dest_path = path::new(dest_file);
            let prefix = path.parent().map_err(|e| e.to_string())?;
            create_dir_all(prefix).map_err(|e| e.to_string())?;
            copy(orig_path, dest_path).map_err(|e| e.to_string())?;
            info!("Copied {} to {}", orig_path, dest_path);
            Ok(0)
        })
        .collect()
}

fn read_yaml(yaml_fn: &str) -> Result<Vec<String>, String> {
    let f = std::fs::File::open(yaml_fn).map_err(|e| e.to_string())?;
    let data: DotFiles = serde_yaml::from_reader(f)?;
    Ok(data.dotfiles)
}
