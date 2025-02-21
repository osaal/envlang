# Graphemes

Iterator over the grapheme clusters.

Consuming it will return each grapheme cluster.

Implements traits:
- Clone
- Debug
- DoubleEndedIterator
- Iterator

Can be ran from a String into a Vec\<String\> using the following:

``` {rust}
UnicodeSegmentation::graphemes(input.as_str(), true)
    .map(|val| val.to_string())
    .collect::<Vec<String>>();
```

# UnicodeWords

Iterator over words separated on word boundaries.

Words are substrings that contain alphanumeric characters.

Alphanumerics are defined as at least one character with either the Alphabetic or Number properties.

The Alphabetic property is defined as Lowercase+Uppercase+Lt+Lm+Lo+Nl+Other_Alphabetic, where Lowercase includes Ll+Other_Lowercase and Uppercase includes Lu+Other_Uppercase.

- Lt = Titlecase_letter (digraph with first part uppercase)
- Lm = Modifier letter
- Lo = Other letter (syllables, ideographs)
- Nl = Letter number (letterlike numeric character)
- Ll = Lowercase letter
- Lu = Uppercase letter
- Other_Alphabetic = not defined in the standard
- Other_Lowercase = not defined in the standard
- Other_Uppercase = not defined in the standard

The Number property is defined as Nd + Nl + No.

- Nd = Decimal digit
- Nl = Letter number (letterlike numeric character)
- No = Other numeric character type

Note some quirks from the example documentation:

- English-language genitive apostrophies and/or short-hands will be a single word, e.g., "can't"
- Decimal numbers will be a single word, e.g., "32.3"
- Any other symbols seem to be ignored, e.g., no "(", "\\", "," or "?" 