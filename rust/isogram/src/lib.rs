//! Determine if a word or phrase is an isogram.
//! 
//! An isogram (also known as a "nonpattern word") is a word or phrase
//! without a repeating letter, however spaces and hyphens are allowed
//! to appear multiple times.
//! 
//! Examples of isograms:
//! lumberjacks
//! background
//! downstream
//! six-year-old
//! 
//! The word isograms, however, is not an isogram, because the s repeats.

/// Check if the input is an isogram.
pub fn check(candidate: &str) -> bool {
    let mut letters = [false; 26];
    let offset = 'a' as usize;

    for ch in candidate.chars().filter(|c| c.is_alphabetic()) {
        // Somewhat weird way of getting the ch on the correct form.
        // There must be a better way to do this!
        for ch in ch.to_lowercase() {
            let idx = (ch as usize) - offset;
            if letters[idx] {
                return false;
            }
            letters[idx] = true;
        }
    }
    true
}
