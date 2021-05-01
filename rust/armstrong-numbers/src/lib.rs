/// is_armstrong_number returns true is nume is an armstrong number.
pub fn is_armstrong_number(num: u32) -> bool {
    num == compute_sum_digit_pow(&num, &count_digits(&num))
}

/// count_digits computes the number of digits of num.
fn count_digits(num: &u32) -> u32 {
    let mut left = *num;
    let mut n_digits = 0;
    while left > 0 {
        n_digits+=1;
        left /= 10;
    }
    n_digits
}

/// compute_sum_digit_pow computes the sum of the digits to the 
/// n_digit power.
fn compute_sum_digit_pow(num: &u32, n_digits: &u32) -> u32 {
    let mut left = *num;
    let mut sum = 0;
    while left > 0 {
        sum += (left%10).pow(*n_digits);
        left /= 10;
    }
    sum
}
