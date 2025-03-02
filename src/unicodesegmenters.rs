//! Wrappers for segmenting input strings into Unicode graphemes
//! 
//! This module contains functions for converting `String` data into Unicode-segmented `String` vectors.
//! 
//! It uses the external crate [`unicode_segmentation`] heavily.
//! 
//! [`unicode_segmentation`]: https://crates.io/crates/unicode-segmentation

use unicode_segmentation::UnicodeSegmentation;

/// Segment a string slice into Unicode graphemes
/// 
/// # Arguments
/// * `input` - The string slice to segment
/// 
/// # Returns
/// A vector of strings representing single extended grapheme clusters
/// 
/// # Performance
/// The vector allocates new strings for each grapheme cluster.
/// 
/// # Examples
/// ```
/// use envlang::unicodesegmenters::segment_graphemes;
/// 
/// let input = "Hello, 世界!";
/// let segments = segment_graphemes(input);
/// assert_eq!(segments.len(), 9);
/// ```
pub fn segment_graphemes(input: &str) -> Vec<String> {
    UnicodeSegmentation::graphemes(input, true)
        .map(String::from)
        .collect()
}

// Unit tests for unicodesegmenters.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_crlf_as_single_grapheme() {
        let input = "line1\r\nline2\nline3\r\nline4";
        let segments = segment_graphemes(input);
        
        assert!(segments.contains(&"\r\n".to_string()));
        
        // Count the actual segments to verify no splitting
        let newlines: Vec<&String> = segments.iter()
            .filter(|&s| s == "\r\n" || s == "\n")
            .collect();
        assert_eq!(newlines.len(), 3);
    }

    #[test]
    fn handles_empty_string() {
        let input = "";
        let segments = segment_graphemes(input);
        assert!(segments.is_empty());
    }

    #[test]
    fn handles_unicode_combining_characters() {
        let input = "e\u{301}";
        let segments = segment_graphemes(input);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0], "e\u{301}");
    }

    #[test]
    fn handles_emoji_sequences() {
        let input = "😺";
        let segments = segment_graphemes(input);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0], "😺");
    }

    #[test]
    fn preserves_input_sequence() {
        let input = "Hello, 世界!";
        let segments = segment_graphemes(input);
        assert_eq!(segments.join(""), input);
    }
}