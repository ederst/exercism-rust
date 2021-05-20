extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

// Found some useful information for this exercise here:
//   * https://docs.rs/unicode-reverse/1.0.8/unicode_reverse/

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
