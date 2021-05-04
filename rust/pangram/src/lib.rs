//! Determine if a sentence is a pangram. A pangram (Greek: παν γράμμα, 
//! pan gramma, "every letter") is a sentence using every letter of 
//! the alphabet at least once. The best known English pangram is:
//! 
//! The quick brown fox jumps over the lazy dog.
//! 
//! The alphabet used consists of ASCII letters a to z, inclusive, 
//! and is case insensitive. Input will not contain non-ASCII symbols.
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
