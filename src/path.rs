use dirs::home_dir;
use std::path::{Path, PathBuf};
use std::string::String;

pub fn home_path() -> Result<String, String> {
    let home = home_dir();
    match home {
        Some(dir) => {
            let home_dir = dir.into_os_string().into_string();
            match home_dir {
                Ok(home_dir_str) => Ok(home_dir_str),
                _ => Err(String::from("Cannot parse home directory")),
            }
        }
        _ => Err(String::from("Cannot get home directory")),
    }
}

pub fn file_to_path(filename: String) -> Result<PathBuf, String> {
    let file_path = Path::new(&filename);
    match file_path.is_file() {
        true => Ok(file_path.to_path_buf()),
        _ => Err(format!("File not found: {}", filename)),
    }
}
