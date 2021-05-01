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
