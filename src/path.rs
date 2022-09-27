use dirs::home_dir;

fn home_path() -> Result<String, String> {
    let home = home_dir();
    let home_path = match home {
        Some(dir) => dir.into_os_string().into_string()?,
        _ => Err(String::from("Cannot get home directory")),
    };
    Ok(home_path)
}
