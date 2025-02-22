use std::fs::read_to_string;

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    // Envlang files must have the .envl suffix to work
    if !path.ends_with(".envl") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "File must have .envl extension",
        ));
    }
    
    return Ok(read_to_string(path)?);
}

// Unit tests for io.rs
#[cfg(test)]
mod tests {
    use super::*;
    // Test that read_file returns an error when the file does not have the .envl extension
    #[test]
    fn invalid_file_extension() {
        let result = read_file("io_invalidextension.txt");
        assert!(result.is_err());
    }
}