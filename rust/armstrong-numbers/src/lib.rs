//! An Armstrong number is a number that is the sum of its own digits each raised to the power of the number of digits.
//! 
//! # Example
//! 
//! 9 is an Armstrong number, because 9 = 9^1 = 9
//! 10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
//! 153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
//! 154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190

/// returns true is num is an armstrong number.
pub fn is_armstrong_number(num: u32) -> bool {
    num == compute_sum_digit_pow(&num, &count_digits(&num))
}

/// computes the number of digits of num.
fn count_digits(num: &u32) -> u32 {
    let mut left = *num;
    let mut n_digits = 0;
    while left > 0 {
        n_digits+=1;
        left /= 10;
    }
    n_digits
}

/// computes the sum of the digits to the n_digit power.
fn compute_sum_digit_pow(num: &u32, n_digits: &u32) -> u32 {
    let mut left = *num;
    let mut sum = 0;
    while left > 0 {
        sum += (left%10).pow(*n_digits);
        left /= 10;
    }
    sum
}
