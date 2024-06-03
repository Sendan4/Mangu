use std::path::Path;
use std::{env, fs};

// Crea y se mueve a un directorio
pub fn move_to_path_create_dir(path: &Path) -> Result<(), std::io::Error> {
    // Create directory if it doesn't exist

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    // Move to the specified directory
    env::set_current_dir(path)?;

    if !path.exists() {
        fs::create_dir_all("src")?;
    }

    Ok(())
}
