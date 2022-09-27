use clap::{Arg, ArgMatches, Command, SubCommand};
use std::string::String;

const PROGRAM_DESC: &str = "Convenient utils for saving and install dotfiles";
const PROGRAM_NAME: &str = "dotfiles";

pub fn command() -> Result<ArgMatches, String> {
    let cli = Command::new(PROGRAM_NAME)
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
            SubCommand::with_name("save")
                .about("Save dotfiles into a folder")
                .arg(
                    Arg::with_name("dest_dir")
                        .help("The destination directory to save for copying the dotfiles to")
                        .short('d')
                        .long("dest-dir")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Installing dotfiles from a github url")
                .arg(
                    Arg::with_name("url")
                        .help("The github url")
                        .short('u')
                        .long("url")
                        .takes_value(true)
                        .required(true),
                ),
        );
    Ok(cli.get_matches())
}
