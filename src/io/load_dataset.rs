use std::error::Error;

/// Load file by path
pub fn load_file(path: &str) -> Result<String, Box<dyn Error>> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}
