
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
