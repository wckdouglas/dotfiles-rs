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

/// Convert string type filename to pathbuf (and perform check)
///
/// # Args
/// - `filename`: the filename for check and convert
/// - `check`: whether we whould check the existence of the file
///
/// # Return
/// - file path of the input filename
///
/// # Example
///
/// ```
/// use dotfiles_rs::path::file_to_path;
///
/// let filename = "blahblah".to_string();
/// let result1 = file_to_path(&filename, true);
/// assert!(result1.is_err());
///
/// let filename2 = "blahblah".to_string();
/// let result2 = file_to_path(&filename2, false);
/// assert!(result2.is_ok());
/// ```
pub fn file_to_path(filename: &String, check: bool) -> Result<PathBuf, String> {
    let file_path = Path::new(&filename);

    if check {
        match file_path.is_file() {
            true => Ok(file_path.to_path_buf()),
            _ => Err(format!("File not found: {}", filename)),
        }
    } else {
        Ok(file_path.to_path_buf())
    }
}
