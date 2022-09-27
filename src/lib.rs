pub mod cli;
pub mod path;

use cli::command;
use log::info;
use path::home_path;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{copy, create_dir_all, metadata};
use std::path::Path;
use std::string::String;

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
                let destination_dir: &str = sub_m
                    .value_of::<&str>("dest_dir")
                    .ok_or("No dest-dir provided")?;
                save(dotfile_list, destination_dir.to_string())?;
                info!(
                    "You can now go to {} and create a github repo!",
                    destination_dir
                );
                Ok(0)
            }
            _ => Err(String::from("Unsupported subcommand".to_string())),
        },
        _ => Err("No subcommand provided".to_string()),
    }
}

/// Saving the provided list of dotfiles to the provided output folder
///
/// # Arguments
/// - *dotfile_list*: a list of filenames relative to home directory
/// - *destination_dir*: the folder to store the copies
///
/// # Return
/// - Result<Vec<u8>, String>: return code for each copy
fn save(dotfile_list: Vec<String>, destination_dir: String) -> Result<Vec<u8>, String> {
    let home_dir = home_path()?;
    dotfile_list
        .into_par_iter()
        .map(|dotfile| {
            let orig_file = format!("{}/{}", home_dir, dotfile);
            let dest_file = format!("{}/{}", destination_dir, dotfile);
            let orig_path = Path::new(&orig_file);
            let dest_path = Path::new(&dest_file);
            metadata(orig_path)
                .or_else(|_| Err(format!("{} does not exist", &orig_path.display())))?; //check whether the file exists
            let prefix = &dest_path.parent();
            match prefix {
                Some(prefix_path) => create_dir_all(prefix_path).map_err(|e| e.to_string())?,
                _ => (),
            };
            copy(orig_path, &dest_path).map_err(|e| e.to_string())?;
            info!(
                "Copied {} to {}",
                &orig_path.display(),
                &dest_path.display()
            );
            Ok(0)
        })
        .collect()
}

/// reading the dotfile yaml
///
/// # Arguments
/// - *yaml_fn*: the input yaml file path to be read
///
/// # Return
/// - list of dotfiles to be copied
fn read_yaml(yaml_fn: &str) -> Result<Vec<String>, String> {
    let f = std::fs::File::open(yaml_fn).map_err(|e| e.to_string())?;
    let data: DotFiles = serde_yaml::from_reader(f).map_err(|e| e.to_string())?;
    Ok(data.dotfiles)
}
