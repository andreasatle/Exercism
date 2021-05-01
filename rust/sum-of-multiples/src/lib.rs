use std::cmp;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut num = next_number(0, limit, factors);
    let mut sum = 0;
    while num < limit {
        sum += num;
        num = next_number(num, limit, factors);
    }
    sum
}

fn next_number(num: u32, limit: u32, factors: &[u32]) -> u32 {
    let mut next = num + limit; // Make next too big...
    for f in factors.iter() {
        if *f == 0 {
            continue;
        }
        next = cmp::min(next, num+f-num%f);
    }
    next
}

