pub fn reply(message: &str) -> &str {
    let num_lowercase = message.chars().filter(|c| c.is_ascii_lowercase()).count();
    let num_uppercase = message.chars().filter(|c| c.is_ascii_uppercase()).count();
    let num_digits = message.chars().filter(|c| c.is_ascii_digit()).count();
    let is_yelling = num_lowercase == 0 && num_uppercase > 0;
    let is_saying_something = num_lowercase + num_uppercase + num_digits > 0;
    let ends_with = message.trim().chars().last();
    match (is_yelling, ends_with, is_saying_something) {
        (false, Some('?'), _) => process_question(message),
        (true, Some('?'), true) => process_yelled_question(message),
        (false, Some(_), true) => process_non_question(message),
        (true, Some(_), true) => process_yelled_non_question(message),
        _ => process_empty_utterance(message),
    }
}

fn process_question(message: &str) -> &str {
    "Sure."
}

fn process_yelled_question(message: &str) -> &str {
    "Calm down, I know what I'm doing!"
}

fn process_non_question(message: &str) -> &str {
    "Whatever."
}

fn process_yelled_non_question(message: &str) -> &str {
    "Whoa, chill out!"
}

fn process_empty_utterance(message: &str) -> &str {
    "Fine. Be that way!"
}
