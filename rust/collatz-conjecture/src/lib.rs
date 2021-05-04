//! The Collatz Conjecture or 3x+1 problem can be summarized as follows:
//! 
//! Take any positive integer n. If n is even, divide n by 2 to get n / 2. If n is odd, multiply n by 3 and add 1 to get 3n + 1. Repeat the process indefinitely. The conjecture states that no matter which number you start with, you will always reach 1 eventually.
//! 
//! Given a number n, return the number of steps required to reach 1.
//! 
//! # Example
//! Starting with n = 12, the steps would be as follows:
//! 
//! ```comment
//! 12, 6, 3, 10, 5, 16, 8, 4, 2, 1
//! ```
//! Resulting in 9 steps. So for input n = 12, the return value would be 9.

/// returns the number of steps it takes to reach 1. A zero value returns None.
pub fn collatz(n: u64) -> Option<u64> {
    let mut cnt = 0;
    let mut k = n;
    loop {
        match &mut k {
            0 => return None,
            1 => return Some(cnt),
            k if *k & 1 == 0 => {
                cnt += 1;
                *k >>= 1;
            }
            k => {
                cnt += 1;
                *k = 3 * *k + 1;
            }
        }
    }
}
