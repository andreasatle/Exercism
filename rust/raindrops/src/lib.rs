/// The rules of `raindrops` are that if a given number:
///
/// - has 3 as a factor, add 'Pling' to the result.
/// - has 5 as a factor, add 'Plang' to the result.
/// - has 7 as a factor, add 'Plong' to the result.
/// - _does not_ have any of 3, 5, or 7 as a factor, the result should be the digits of the number.

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
