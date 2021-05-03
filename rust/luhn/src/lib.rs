/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut checksum = 0;
    let mut cnt = 0;
    for ch in code.chars().rev() {
        if ch == ' ' {
            continue
        }
        cnt += 1;

        let opt_d = ch.to_digit(10);
        match opt_d {
            None => return false,
            Some(mut d) => {
                if cnt%2 == 0 {
                    d *= 2;
                    if d > 9 {
                        d -= 9;
                    }
                }
                checksum += d;        
            }
        }
    }
    cnt > 1 && checksum%10 == 0
}
