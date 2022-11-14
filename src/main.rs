use dotfiles_rs::cli::command;
use dotfiles_rs::path::home_path;
use dotfiles_rs::{install, read_yaml, save};
use log::info;

/// Wrapper function to run the workflow
fn run() -> Result<u8, String> {
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
            _ => Err("Unsupported subcommand".to_string()),
        },
        _ => Err("No subcommand provided".to_string()),
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let result = run();
    match result {
        Ok(_) => (),
        Err(err_string) => println!("{}", err_string),
    };
}
