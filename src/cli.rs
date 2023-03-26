use clap::{Arg, ArgAction, ArgMatches, Command, SubCommand};
use std::string::String;

const PROGRAM_DESC: &str = "Convenient utils for saving and install dotfiles";
const PROGRAM_NAME: &str = "dotfiles";

pub fn command() -> Result<ArgMatches, String> {
    let cli: Command = Command::new(PROGRAM_NAME)
        .version("0.1.0")
        .author("Douglas Wu <wckdouglas@gmail.com>")
        .about(PROGRAM_DESC)
        .arg(
            Arg::with_name("dotfile_yaml")
                .help("A yaml file containing the dotfile names (see data/dotfiles.yaml)")
                .long("dotfile-yaml")
                .short('y')
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            // the first subcommand is to save the 
            // needed config files into a new dir
            SubCommand::with_name("save")
                .about("Save dotfiles into a folder")
                .arg(
                    Arg::with_name("dest_dir")
                        .help("The destination directory to save for copying the dotfiles to")
                        .short('d')
                        .long("dest-dir")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("dry")
                        .help("Dry run")
                        .long("dry-run")
                        .takes_value(false)
                        .required(false)
                        .action(ArgAction::SetTrue),
                    )
        )
        .subcommand(
            SubCommand::with_name("apply")
                // the second subcommand is to clone a github dotfile repo
                // and apply the files
                .about("Applying dotfiles from a github url")
                .arg(
                    Arg::with_name("url")
                        .help("The github url")
                        .short('u')
                        .long("url")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("ssh_key")
                        .help("The ssh key to use, the private key file, should have a .pub file in the same folder too (default: ~/.ssh/id_rsa)")
                        .short('s')
                        .long("ssh-key")
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::with_name("dry")
                        .help("Dry run")
                        .long("dry-run")
                        .takes_value(false)
                        .required(false)
                        .action(ArgAction::SetTrue)
                ),
        );

    Ok(cli.get_matches())
}
