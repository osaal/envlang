use unicode_segmentation::UnicodeSegmentation;

/// Parse a `String` input into Unicode graphemes
/// 
/// -  input: The input as a `String` type
/// 
/// The return vector will contain whitespace and newline characters
/// The return will not attempt any other character combinations such as Unicode word boundaries
pub fn parse_into_graphemes(input: String) -> Vec<String> {
    let collected = UnicodeSegmentation::graphemes(
        input.as_str(),
        true
    )
        .map(|val| val.to_string())
        .collect::<Vec<String>>();

    return collected;
}

/// Drop whitespace characters from given `String` vector
/// 
/// -  input: Vector of `String` types
/// 
/// See https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt for a full list of dropped characters
pub fn drop_whitespace(input: Vec<String>) -> Vec<String> {
    let mut cleaned: Vec<String> = Vec::new();

    for el in &input {
        let val: Vec<char> = el.chars().collect();
        if val[0].is_whitespace() {
            continue;
        }
        cleaned.push(el.clone());
    }

    return cleaned;
}

/// Converts a vector of integers into a String
/// 
/// -  ints: Vector of i32 integers
/// 
/// Used internally by `concatenate_ints()`
fn convert_ints(ints: &Vec<i32>) -> String {
    ints
        .iter()
        .map(|num| num.to_string())
        .collect::<String>()
        .as_str()
        .to_owned()
}

/// Take a vector of collected Unicode graphemes and 'concatenate' numbers together.
/// 
/// # Example
/// `["1", "2", "+", "3"] -> ["12", "+", "3"]`
pub fn concatenate_ints(input: Vec<String>) -> Vec<String> {
    let mut constructed: Vec<String> = Vec::new();
    let mut i = 0;

    // Main loop options:
    // A) Push char into constructed
    // B) Start int loop
    // Int loop options:
    // A) Push i32 onto temp
    // B) Convert temp through String to new i32 and push i32 onto constructed

    while i < input.len() {
        let el = &input[i];
        let parse = el.parse::<i32>();

        match parse {
            Ok(_) => {
                // Value conforms to i32, start constructing string
                let mut temp: Vec<i32> = Vec::new();
                let mut j = i;

                while j < input.len() {
                    let inner_el = &input[j];
                    match inner_el.parse::<i32>() {
                        Ok(_) => {
                            // Value conforms to i32, add to temp
                            temp.push(inner_el.parse::<i32>().unwrap());
                            j += 1;
                            if j == input.len() {
                                let converted_number = convert_ints(&temp);
                                constructed.push(converted_number);
                            }
                        },
                        Err(_) => {
                            // No more numbers that conform to i32
                            // Convert temp to string and push
                            // Ignore whitespace char
                            if inner_el
                                .chars()
                                .collect::<Vec<char>>()[0]
                                .is_whitespace() {
                                    j += 1;
                                    continue;
                                }
                            
                            // TODO: Currently requires that string does not end in whitespace!
                            let converted_number = convert_ints(&temp);
                            constructed.push(converted_number);

                            break;
                        },
                    }
                }
                // Outer loop starts from where inner loop left off
                i = j;
            },
            Err(_) => {
                // Value does not conform to i32
                // Push value as String instead
                // TODO: Value could be a decimal marker, which would imply a float
                constructed.push(el.to_string());
                i += 1;
            },
        }
    }

    return constructed;
}