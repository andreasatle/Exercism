//! Reverse a string.
//!
//! For example: input: "cool" output: "looc".

/// Return the string in reverse.
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
