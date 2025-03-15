//! Envlang internal input/output functions
//! 
//! These functions are used by the Envlang interpreter to read and write `.envl` files

use std::fs::read_to_string;

/// Read an `.envl` file
/// 
/// Returns a `Result<String, std::io::Error>`
/// 
/// # Errors
/// Possible errors are all the usual `std::io::Error`s.
/// 
/// It also errors if the file does not end in the `.envl` file extension (`std::io::ErrorKind::InvalidInput`)
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
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

    #[test]
    fn invalid_file_extension() {
        let result = read_file("tests/data/io_invalidextension.txt");
        assert!(result.is_err());
    }

    #[test]
    fn file_does_not_exist() {
        let result = read_file("tests/data/doesnotexist.envl");
        assert!(result.is_err());
    }

    #[test]
    fn file_is_read() {
        let result = read_file("tests/data/io_validextension.envl").unwrap();
        assert_eq!(result, "here is some text\nwith\nwhite\nspace".to_string())
    }
}