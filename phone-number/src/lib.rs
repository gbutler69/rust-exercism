#![feature(or_patterns)]

pub fn number(user_number: &str) -> Option<String> {
    user_number
        .chars()
        .filter(char::is_ascii_digit)
        .enumerate()
        .skip_while(|(idx, digit)| *idx == 0 && *digit == '1')
        .map(|(_, digit)| digit)
        .enumerate()
        .map(digit_valid_for_position)
        .collect::<Option<String>>()
        .filter(|number| number.len() == 10)
}

fn digit_valid_for_position((idx, digit): (usize, char)) -> Option<char> {
    match (idx, digit) {
        (0 | 3 | 6, '0'..='1') => None,
        _ => Some(digit),
    }
}
