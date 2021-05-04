//! Given a string containing brackets [], braces {}, parentheses (), 
//! or any combination thereof, verify that any and all pairs are matched 
//! and nested correctly.

/// Returns true if all brackets (,),[,],{,} are balanced.
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        if c == '(' {
            stack.push('(');
        }
        if c == '[' {
            stack.push('[');
        }
        if c == '{' {
            stack.push('{');
        }
        if c == ')' {
            if let Some(par) = stack.pop() {
                if par != '(' {
                    return false;
                }
            } else {
                return false;
            }
        }
        if c == ']' {
            if let Some(par) = stack.pop() {
                if par != '[' {
                    return false;
                }
            } else {
                return false;
            }
        }
        if c == '}' {
            if let Some(par) = stack.pop() {
                if par != '{' {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    stack.len() == 0
}
