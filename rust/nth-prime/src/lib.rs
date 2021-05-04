//! Given a number n, determine what the nth prime is.
//! 
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//! 
//! If your language provides methods in the standard library to deal with prime numbers, pretend they don't exist and implement them yourself.
//! 
//! Remember that while people commonly count with 1-based indexing (i.e. "the 6th prime is 13"), many programming languages, including Rust, 
//! use 0-based indexing (i.e. primes[5] == 13). Use 0-based indexing for your implementation.

/// Returns the nth prime.
pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();
    primes.push(2);
    let mut prime: u32 = 2;
    for _k in 0..n {
        prime = compute_next_prime(prime, &primes);
        primes.push(prime);
    }
    prime
}

/// Returns the next prime.
fn compute_next_prime(mut prime: u32, primes: &Vec<u32>) -> u32 {
    loop {
        prime += 1;
        if !divisible_by(prime, &primes) {
            return prime;
        }
    }
}

/// Checks whether val is divisible by any of the primes.
fn divisible_by(val: u32, primes: &Vec<u32>) -> bool {
    for prime in primes.iter() {
        if val % prime == 0 {
            return true;
        }
    }
    false
}
