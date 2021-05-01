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

fn compute_next_prime(mut prime: u32, primes: &Vec<u32>) -> u32 {
    loop {
        prime += 1;
        if !divisible_by(prime, &primes) {
            return prime;
        }
    }
}

fn divisible_by(val: u32, primes: &Vec<u32>) -> bool {
    for prime in primes.iter() {
        if val % prime == 0 {
            return true;
        }
    }
    false
}
