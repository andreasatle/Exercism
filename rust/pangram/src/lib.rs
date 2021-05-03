use std::collections::HashSet;
use std::ascii::AsciiExt;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut letter_set = HashSet::new();
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        letter_set.insert(c);
    }
    for c in sentence.chars() {
        letter_set.remove(&AsciiExt::to_ascii_lowercase(&c));
    }
    letter_set.len() == 0
}
