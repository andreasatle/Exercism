//! Find the difference between the square of the sum and the sum of the squares of the first N natural numbers.
//! 
//! # Example
//! The square of the sum of the first ten natural numbers is (1 + 2 + ... + 10)² = 55² = 3025.
//! 
//! The sum of the squares of the first ten natural numbers is 1² + 2² + ... + 10² = 385.
//! 
//! Hence the difference between the square of the sum of the first ten natural numbers and the sum of the squares of the first ten natural numbers is 3025 - 385 = 2640.
//! 
//! # Derivation
//! You are not expected to discover an efficient solution to this yourself from first principles; research is allowed, indeed, encouraged. Finding the best algorithm for the problem is a key skill in software engineering.
//! 
//! Following derivation comes from brilliant.org
//!
//! Sn = sum(k,1,n), Tn = sum(k^2,1,n)
//!
//! Start with Sn: sum((k-1)^2,1,n) = sum(k^2-2*k+1,1,n)
//!
//! Rearrange: n^2 = sum(k^2-(k-1)^2,1,n) = 2*Sn-n
//!
//! Finally: Sn = n*(n+1)/2
//!
//! Now Tn: sum(k^3-(k-1)^3,1,n) = 3*Tn-3*Sn+n.
//!
//! Finally: Tn = n*(2*n+1)*(n+1)/6

/// computes the square of the sum 1..=n
pub fn square_of_sum(n: u32) -> u32 {
    n * n * (n + 1) * (n + 1) / 4
}

/// computes the sum of the squares 1^2..=n^2
pub fn sum_of_squares(n: u32) -> u32 {
    n * (2 * n + 1) * (n + 1) / 6
}

/// computes the difference of the square of the sum and the sum of the squares
pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
