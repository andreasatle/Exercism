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
    let mut distance: usize = 0;
    let mut iter1 = s1.chars();
    let mut iter2 = s2.chars();

    let mut ch1 = iter1.next();
    let mut ch2 = iter2.next();

    while ch1 != None || ch2 != None {
        match ch1.zip(ch2) {
            Some((x,y)) => {
                if x != y {
                    distance += 1;
                }
            },
            None => {return None;},
        }
        ch1 = iter1.next();
        ch2 = iter2.next();
    }
    Some(distance)
}
