// What is the default string encoding of Rust?
// What's the difference between a character, a code point, a glyph and a grapheme?
// What is the optimal crate for grapheme manipulation?

//
// cargo test --features grapheme
// exercism submit absolutePath/lib.rs

use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
