pub mod cli;
pub mod path;

use cli::command;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use log::info;
use path::{file_to_path, home_path};
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
    let home_dir = home_path()?;
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
            "install" => {
                let github_url: &str =
                    sub_m.value_of::<&str>("url").ok_or("No git url provided")?;
                let ssh_key_file: String = sub_m
                    .value_of::<&str>("ssh_key")
                    .unwrap_or(format!("{}/.ssh/id_rsa", &home_dir).as_str())
                    .to_string();
                install(dotfile_list, github_url.to_string(), ssh_key_file)?;
                Ok(0)
            }
            _ => Err(String::from("Unsupported subcommand".to_string())),
        },
        _ => Err("No subcommand provided".to_string()),
    }
}

fn copy_file(dest_file_path: &Path, orig_file_path: &Path) -> Result<u8, String> {
    metadata(orig_file_path)
        .or_else(|_| Err(format!("{} does not exist", &orig_file_path.display())))?; //check whether the file exists
    let prefix = &dest_file_path.parent();
    match prefix {
        Some(prefix_path) => create_dir_all(prefix_path).map_err(|e| e.to_string())?,
        _ => (),
    };
    copy(orig_file_path, &dest_file_path).map_err(|e| e.to_string())?;
    info!(
        "Copied {} to {}",
        &orig_file_path.display(),
        &dest_file_path.display()
    );
    Ok(0)
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
            let orig_path = file_to_path(orig_file)?;
            let dest_path = Path::new(&dest_file);
            copy_file(dest_path, &orig_path)
        })
        .collect()
}

fn install(
    dotfile_list: Vec<String>,
    github_url: String,
    ssh_key_file: String,
) -> Result<Vec<u8>, String> {
    let home_dir = home_path()?;
    let git_dotfiles_dir = format!("{}/dotfiles", &home_dir);
    let git_dotfiles_path: &Path = Path::new(&git_dotfiles_dir);

    let _repo = match git_dotfiles_path.exists() {
        true => Repository::open(&git_dotfiles_path)
            .or_else(|_| Err(format!("Folder not exists: {}", git_dotfiles_dir))),
        _ => {
            let repo = git_clone(&github_url, &git_dotfiles_dir, ssh_key_file);
            info!("Clone complete");
            repo
        }
    }?;
    dotfile_list
        .into_par_iter()
        .map(|dotfile| {
            let orig_file = format!("{}/{}", &git_dotfiles_dir, dotfile);
            let dest_file = format!("{}/{}", home_dir, dotfile);
            let orig_path = Path::new(&orig_file);
            let dest_path = Path::new(&dest_file);
            copy_file(dest_path, orig_path)
        })
        .collect()
}

fn git_clone(
    github_url: &String,
    git_dotfiles_dir: &String,
    ssh_private_key_fn: String,
) -> Result<Repository, String> {
    match github_url.starts_with("git@github.com") {
        true => {
            info!("Cloning {} into {}", github_url, git_dotfiles_dir);
            let git_dotfiles_path: &Path = Path::new(&git_dotfiles_dir);
            let ssh_pub_key_fn = format!("{}.pub", &ssh_private_key_fn);
            let ssh_pub_key_file_path = file_to_path(ssh_pub_key_fn)?;
            let ssh_private_key_file_path = file_to_path(ssh_private_key_fn)?;

            let mut builder = RepoBuilder::new();
            let mut callbacks = RemoteCallbacks::new();
            let mut fetch_options = FetchOptions::new();

            callbacks.credentials(|_, _, _| {
                let credentials = Cred::ssh_key(
                    "git",
                    Some(&ssh_pub_key_file_path),
                    &ssh_private_key_file_path,
                    None,
                ).expect("Credential problem");
                Ok(credentials)
            });

            fetch_options.remote_callbacks(callbacks);

            builder.fetch_options(fetch_options);

            builder
                .clone(&github_url, git_dotfiles_path)
                .or_else(|e| Err(e.to_string()))
        },
        _ => Err(String::from("We only support ssh-key cloning, which the github url should start with git@github.com prefix"))
    }
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
