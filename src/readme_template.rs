use log::info;
use std::fs;

const README_TEXT: &str = "# Dotfiles\n
This is a repo to save all the config files
from backing up with [dotfiles-rs](https://github.com/wckdouglas/dotfiles-rs) \n\n
:warning: this repo should be private!!
";

/// Writing template readme to a file
///
/// # Args
/// - `readme_fn`: file path to the output readme markdown file
///
/// # Return
/// - return code or error
///
/// # Example
/// ```
/// use dotfiles_rs::readme_template::write_template_readme;
///
/// let filename = "test_readme.md";
/// write_template_readme(filename.to_string());
/// ```
pub fn write_template_readme(readme_fn: String) -> Result<u8, String> {
    fs::write(&readme_fn, README_TEXT).map_err(|e| e.to_string())?;
    info!("Written {}", readme_fn);
    Ok(0)
}
