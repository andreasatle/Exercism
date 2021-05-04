//! Given a number, find the sum of all the unique multiples of particular 
//! numbers up to but not including that number.
//! 
//! If we list all the natural numbers below 20 that are multiples of 
//! 3 or 5, we get 3, 5, 6, 9, 10, 12, 15, and 18.
//! 
//! The sum of these multiples is 78.

use std::cmp;

/// Returns the sum of all multiples of the given factors.
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut num = next_multiple(0, limit, factors);
    let mut sum = 0;
    while num < limit {
        sum += num;
        num = next_multiple(num, limit, factors);
    }
    sum
}

/// Compute the next multiple.
fn next_multiple(num: u32, limit: u32, factors: &[u32]) -> u32 {
    let mut next = num + limit; // Make next too big...
    for f in factors.iter() {
        if *f == 0 {
            continue;
        }
        next = cmp::min(next, num+f-num%f);
    }
    next
}

