pub fn reply(message: &str) -> &str {
    let mut lowercase: bool = false;
    let mut uppercase: bool = false;
    let mut question: bool = false;
    let mut numeric: bool = false;
    for c in message.chars() {
        if c.is_lowercase() {
            lowercase = true;
        }
        if c.is_uppercase() {
            uppercase = true;
        }
        if c.is_numeric() {
            numeric = true;
        }
    }

    for c in message.chars().rev() {
        if c.is_whitespace() {
            continue
        }
        if c == '?' {
            question = true;
        }
        break;
    }

    println!("{},{},{},{}",message,lowercase,uppercase,question);


    if !lowercase && uppercase && question {
        return "Calm down, I know what I'm doing!";
    }

    if question {
        return "Sure.";
    }

    if !lowercase && !uppercase && !numeric {
        return "Fine. Be that way!";
    }


    if !lowercase && uppercase {
        return "Whoa, chill out!";
    }

    "Whatever."
}
