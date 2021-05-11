//! Convert a number, represented as a sequence of digits in one base, to any other base.

//! Implement general base conversion. Given a number in base a, represented as a sequence
//! of digits, convert it to base b.
#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {

    if from_base < 2 {
        return Err(Error::InvalidInputBase)
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase)
    }

    // Limit ourself to only allow numbers that we can represent with u32.
    let mut num: u32 = 0;

    for e in number {
        if *e >= from_base {
            return Err(Error::InvalidDigit(*e));
        }
        num *= from_base;
        num += e;
    }

    // Results will be computed backwards
    let mut converted_number = Vec::new();
    while num > 0 {
        converted_number.push(num%to_base);
        num /= to_base;
    }

    if converted_number.len() == 0 {
        converted_number.push(0);
    }
    
    // Reverse the number before returning.
    converted_number.reverse();
    Ok(converted_number)
}
