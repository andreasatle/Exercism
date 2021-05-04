//! Compute the prime factors of a given natural number.
//! 
//! A prime number is only evenly divisible by itself and 1.
//! 
//! Note that 1 is not a prime number.

/// Returns a vector with all primes up to n.
pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();
    if n < 2 { // check whether there are any factors.
        return prime_factors;
    }

    let mut next = n;
    let mut prime = 2;
    let mut primes = vec![];

    while next > 1 && prime*prime <= n {
        // update with current prime
        while prime*prime <= n && next%prime == 0 {
            prime_factors.push(prime);
            next /= prime;
        }

        // save and get next prime
        primes.push(prime);
        prime = next_prime(prime, &primes[..]);
    }

    // store laast factor (if any...)
    if next > 1 {
        prime_factors.push(next);
    }
    prime_factors
}

/// Computes the next prime, given a vector of primes.
fn next_prime(prime: u64, primes: &[u64]) -> u64 {
    let mut p = prime + 1;
    loop {
        if is_prime(p, &primes) {
            return p;
        }
        p += 1;
    }
}

/// Checks if a number is prime, given all primes smaller than or equal to the number.
fn is_prime(p: u64, primes: &[u64]) -> bool {
    for prime in primes {
        if p % prime == 0 {
            return false;
        }
    }
    true
}