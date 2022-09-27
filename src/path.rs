use dirs::home_dir;

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
