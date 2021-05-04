//! Use the Sieve of Eratosthenes to find all the primes from 2 up to a given number.
//! 
//! The Sieve of Eratosthenes is a simple, ancient algorithm for finding all 
//! prime numbers up to any given limit. It does so by iteratively marking as 
//! composite (i.e. not prime) the multiples of each prime, starting with the 
//! multiples of 2. It does not use any division or remainder operation.
//! 
//! Create your range, starting at two and continuing up to and including the 
//! given limit. (i.e. [2, limit])
//! 
//! The algorithm consists of repeating the following over and over:
//! 
//! take the next available unmarked number in your list (it is prime)
//! mark all the multiples of that number (they are not prime)
//! Repeat until you have processed each number in your range.
//! 
//! When the algorithm terminates, all the numbers in the list that have not 
//! been marked are prime.
//! 
//! The wikipedia article has a useful graphic that explains the 
//! algorithm: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
//! 
//! Notice that this is a very specific algorithm, and the tests don't check 
//! that you've implemented the algorithm, only that you've come up with the 
//! correct list of primes. A good first test is to check that you do not use 
//! division or remainder operations (div, /, mod or % depending on the language).

/// Returns all primes up to upper_bound.
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut sieve = vec![true; (upper_bound + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..=upper_bound as usize {
        if !sieve[i] {
            continue;
        }
        primes.push(i as u64);
        for j in (2*i..=upper_bound as usize).step_by(i as usize) {
            sieve[j] = false;
        }
    }
    primes
}
