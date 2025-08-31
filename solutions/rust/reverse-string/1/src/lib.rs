extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let grapheme_string = UnicodeSegmentation::graphemes(input, true);

    if grapheme_string.as_str().is_empty() {
        input.chars().rev().collect()
    } else {
        grapheme_string.rev().collect()
    }
}
