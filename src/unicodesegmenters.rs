use unicode_segmentation::UnicodeSegmentation;

/// Segment a `String` input into Unicode graphemes
/// 
/// -  input: The input as a `String` type
/// 
/// # Notes
/// The return vector will contain whitespace and newline characters
/// 
/// The return will not attempt any other character combinations such as Unicode word boundaries
/// 
/// Uses the extended Unicode grapheme boundaries
pub fn segment_graphemes(input: String) -> Vec<String> {
    let collected: Vec<String> = UnicodeSegmentation::graphemes(
        input.as_str(),
        true
    )
        .map(|val: &str| val.to_string())
        .collect::<Vec<String>>();
    return collected;
}

/// Segment a `String` input into Unicode words based on grapheme and word boundaries
/// 
/// -  input: The input as a `String` type
/// 
/// # Notes
/// The return vector will not contain whitespace
/// 
/// Note some quirks with the word segmenter (decimals come through in their entirety, English apostrophes are retained, other symbols are removed)
pub fn segment_words(input: String) -> Vec<String> {
    let collected: Vec<String> = UnicodeSegmentation::unicode_words(
        input.as_str()
    )
        .map(|val: &str| val.to_string())
        .collect::<Vec<String>>();
    return collected;
}

// Unit tests for unicodesegmenters.rs
#[cfg(test)]
mod tests {
    use super::*;
}