//! Given a number determine whether or not it is valid per the Luhn formula.
//! 
//! The Luhn algorithm is a simple checksum formula used to validate a variety of identification numbers, such as credit card numbers and Canadian Social Insurance Numbers.
//! 
//! The task is to check if a given string is valid.
//! 
//! Validating a Number
//! Strings of length 1 or less are not valid. Spaces are allowed in the input, but they should be stripped before checking. All other non-digit characters are disallowed.
//! 
//! # Example 1: valid credit card number
//! 4539 3195 0343 6467
//! The first step of the Luhn algorithm is to double every second digit, starting from the right. We will be doubling
//! 
//! ```comment
//! 4_3_ 3_9_ 0_4_ 6_6_
//! ```
//! If doubling the number results in a number greater than 9 then subtract 9 from the product. The results of our doubling:
//! 
//! ```comment
//! 8569 6195 0383 3437
//! ```
//! Then sum all of the digits:
//! 
//! ```comment
//! 8+5+6+9+6+1+9+5+0+3+8+3+3+4+3+7 = 80
//! ```
//! If the sum is evenly divisible by 10, then the number is valid. This number is valid!
//! 
//! # Example 2: invalid credit card number
//! ```comment
//! 8273 1232 7352 0569
//! ```
//! Double the second digits, starting from the right
//! 
//! ```comment
//! 7253 2262 5312 0539
//! ```
//! Sum the digits
//! 
//! ```comment
//! 7+2+5+3+2+2+6+2+5+3+1+2+0+5+3+9 = 57
//! ```
//! 57 is not evenly divisible by 10, so this number is not valid.

/// Check whether a code has a valid luhn checksum.
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
