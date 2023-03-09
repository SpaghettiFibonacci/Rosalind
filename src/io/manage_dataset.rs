use std::error::Error;

/// Load file to String by path
pub fn load_file(path: &str) -> Result<String, Box<dyn Error>> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

/// Save str output to file
pub fn save_file(path: &str, contents: &str) -> Result<(), Box<dyn Error>> {
    std::fs::write(path, contents)?;
    Ok(())
}
