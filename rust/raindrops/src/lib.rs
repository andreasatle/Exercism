//! Your task is to convert a number into a string that contains raindrop sounds corresponding to certain potential factors. A factor is a number that evenly divides into another number, leaving no remainder. The simplest way to test if a one number is a factor of another is to use the modulo operation.
//! 
//! The rules of raindrops are that if a given number:
//! ```comment
//! has 3 as a factor, add 'Pling' to the result.
//! has 5 as a factor, add 'Plang' to the result.
//! has 7 as a factor, add 'Plong' to the result.
//! does not have any of 3, 5, or 7 as a factor, the result should be the digits of the number.
//! ```
//! # Examples
//! ```comment
//! 28 has 7 as a factor, but not 3 or 5, so the result would be "Plong".
//! ```
//! ```comment
//! 30 has both 3 and 5 as factors, but not 7, so the result would be "PlingPlang".
//! ```
//! ```comment
//! 34 is not factored by 3, 5, or 7, so the result would be "34".
//! ```

/// Returns a string according to the rules of raindrops.
pub fn raindrops(n: u32) -> String {
    let mut s = String::new();

    if n % 3 == 0 {
        s += "Pling";
    }
    if n % 5 == 0 {
        s += "Plang";
    }
    if n % 7 == 0 {
        s += "Plong";
    }

    if s.len() > 0 {
        return s;
    }

    n.to_string()
}
