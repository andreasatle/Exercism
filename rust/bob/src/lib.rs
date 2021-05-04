//! Bob is a lackadaisical teenager. In conversation, his responses are very limited.
//! 
//! Bob answers 'Sure.' if you ask him a question, such as "How are you?".
//! 
//! He answers 'Whoa, chill out!' if you YELL AT HIM (in all capitals).
//! 
//! He answers 'Calm down, I know what I'm doing!' if you yell a question at him.
//! 
//! He says 'Fine. Be that way!' if you address him without actually saying anything.
//! 
//! He answers 'Whatever.' to anything else.
//! 
//! Bob's conversational partner is a purist when it comes to written communication and always follows normal rules regarding sentence punctuation in English.

/// returns a reply from Bob
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
