//! Calculate the Hamming Distance between two DNA strands.
//! 
//! When cells divide, their DNA replicates too. Sometimes during
//! this process mistakes happen and single pieces of DNA get
//! encoded with the incorrect information. If we compare two
//! strands of DNA and count the differences between them we can
//! see how many mistakes occurred. This is known as the
//! "Hamming Distance".

/// Return the Hamming distance between two words.
/// This distance is defined as the count of pair-wise characters
/// that differ.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    Some(s1.chars()
        .zip(s2.chars())
        .filter(|(a, b)| a != b)
        .count())
}
