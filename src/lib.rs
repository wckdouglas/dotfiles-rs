pub mod cli;
pub mod path;
pub mod readme_template;

use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use log::info;
use path::{file_to_path, home_path};
use rayon::prelude::*;
use readme_template::write_template_readme;
use serde::{Deserialize, Serialize};
use std::fs::{copy, create_dir_all, File};
use std::path::{Path, PathBuf};
use std::string::String;

/// Dotfile validation schema
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DotFiles {
    // a list of dotfile file names
    dotfiles: Vec<String>,
}

/// A function to copy file (essentially mkdir -p and cp)
///
/// # Args
/// - `dest_file`: a file path pointing to the output file name
/// - `orig_file`: the file path to the original file
/// - `dry_run`: only print out what will be done
///
/// # Return
/// - Result<u8, String>: return code for the mkdir/copy operation
fn copy_file(dest_file: String, orig_file: String, dry_run: bool) -> Result<u8, String> {
    //check whether the source file exists
    let source_file_pathbuf: PathBuf = file_to_path(&orig_file, true)?;
    let dest_file_pathbuf: PathBuf = file_to_path(&dest_file, false)?;
    // mkdir with parents
    let prefix: &Option<&Path> = &dest_file_pathbuf.parent();
    if let Some(prefix_path) = prefix {
        create_dir_all(prefix_path).map_err(|e| e.to_string())?
    }

    // copy over the file
    let mut label: &str = "Dry run";
    if !dry_run {
        copy(source_file_pathbuf, dest_file_pathbuf).map_err(|e| e.to_string())?;
        label = "Shell";
    }
    info!("[{}] Copied {} to {}", label, orig_file, dest_file,);
    Ok(0)
}

/// Saving the provided list of dotfiles to the provided output folder
///
/// # Arguments
/// - `dotfile_list`: a list of filenames relative to home directory
/// - `destination_dir`: the folder to store the copies
///
/// # Return
/// - Result<Vec<u8>, String>: return code for each copy
pub fn save(
    dotfile_list: Vec<String>,
    destination_dir: String,
    dry_run: bool,
) -> Result<Vec<u8>, String> {
    let home_dir: String = home_path()?;
    // writing a readme for the new dotfile repo

    if !dry_run {
        write_template_readme(format!("{}/README.md", &destination_dir))?;
    }

    // copy over the dotfiles
    dotfile_list
        .into_par_iter()
        .map(|dotfile| {
            let orig_file: String = format!("{}/{}", home_dir, dotfile);
            let dest_file: String = format!("{}/{}", destination_dir, dotfile);
            copy_file(dest_file, orig_file, dry_run)
        })
        .collect()
}

/// Apply the dotfiles from a given directory
///
/// # Args
/// - `dotfile_list`: a list of dot files to be installed from the dotfiles directory
/// - `dotfiles_dir`: a give directory storing the dotfiles
/// - `dry_run`: if true, only print, not do copy
pub fn apply(
    dotfile_list: Vec<String>,
    dotfiles_dir: String,
    dry_run: bool,
) -> Result<Vec<u8>, String> {
    info!("Applying dotfiles from: {}", dotfiles_dir);
    let home_dir: String = home_path()?;
    // copy over the files to the desinated folders
    // from the cloned repo
    dotfile_list
        .into_par_iter()
        .map(|dotfile| {
            let orig_file: String = format!("{}/{}", dotfiles_dir, dotfile);
            let dest_file: String = format!("{}/{}", home_dir, dotfile);
            copy_file(dest_file, orig_file, dry_run)
        })
        .collect()
}

/// Installing the dotfiles from a github repo
///
/// # Args
/// - `dotfile_list`: a list of dot files to be installed from the github repo
/// - `github_url`: a valid github url for the repo (e.g. git@github.com:wckdouglas/dotfiles.git, must starts with git@github.com)
/// - `ssh_key_file`: a ssh key file for github authentication (e.g. ~/.ssh/id_rsa)
/// - `dry_run`: if true, only print, not do copy
pub fn install(
    dotfile_list: Vec<String>,
    github_url: String,
    ssh_key_file: String,
    dry_run: bool,
) -> Result<Vec<u8>, String> {
    let home_dir: String = home_path()?;

    // define where to clone the dotfile repo
    let git_dotfiles_dir: String = format!("{}/dotfiles", &home_dir);
    let git_dotfiles_path: &Path = Path::new(&git_dotfiles_dir);

    // cloning the repo
    let _repo = match git_dotfiles_path.exists() {
        true => Repository::open(git_dotfiles_path)
            .map_err(|_| format!("Folder not exists: {}", git_dotfiles_dir)),
        _ => {
            let repo: Result<Repository, String> =
                git_clone(github_url, &git_dotfiles_dir, ssh_key_file);
            info!("Clone complete");
            repo
        }
    }?;
    // applying dotfiles
    apply(dotfile_list, git_dotfiles_dir, dry_run)
}

/// Cloning a github repo with a given ssh key file
///
/// # Args
/// - `github_url`: a git repo url from github starting with git@github.com
/// - `git_dorfiles_dir`: a directory name for cloning the repo locally
/// - `ssh_private_key_fn`: ~/.ssh/id_rsa file, ~/.ssh/id_ecds (the ~/.ssh/id_ecds.pub should also exists!)
fn git_clone(
    github_url: String,
    git_dotfiles_dir: &String,
    ssh_private_key_fn: String,
) -> Result<Repository, String> {
    match github_url.starts_with("git@github.com") {
        true => {
            info!("Cloning {} into {}", github_url, git_dotfiles_dir);
            let git_dotfiles_path: &Path = Path::new(&git_dotfiles_dir);
            // make them to pathbuf
            let ssh_pub_key_fn: String = format!("{}.pub", &ssh_private_key_fn);
            let ssh_pub_key_file_path: PathBuf = file_to_path(&ssh_pub_key_fn, true)?;
            let ssh_private_key_file_path: PathBuf = file_to_path(&ssh_private_key_fn, true)?;

            // cloning the repo
            let mut builder: RepoBuilder = RepoBuilder::new();
            let mut callbacks: RemoteCallbacks = RemoteCallbacks::new();
            let mut fetch_options: FetchOptions = FetchOptions::new();

            callbacks.credentials(|_, _, _| {
                let credentials: Cred = Cred::ssh_key(
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
                .map_err(|e| e.to_string())
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
///
/// # Example
///
/// ```
/// use dotfiles_rs::read_yaml;
///
/// let dotfile_list = read_yaml("data/dotfiles.yaml").unwrap();
/// assert_eq!(dotfile_list.len(), 9);
/// ```
pub fn read_yaml(yaml_fn: &str) -> Result<Vec<String>, String> {
    let f: File = File::open(yaml_fn).map_err(|e| e.to_string())?;
    let data: DotFiles = serde_yaml::from_reader(f).map_err(|e| e.to_string())?;
    Ok(data.dotfiles)
}
