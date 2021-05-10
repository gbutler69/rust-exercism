pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split_whitespace()
        .flat_map(split_word_to_multiple_words_on_punctuation)
        .flat_map(capitalize_first_letter_of_word_and_lowercase_capitals_preceded_by_capital)
        .filter(char::is_ascii_uppercase)
        .collect()
}

fn split_word_to_multiple_words_on_punctuation(word: &str) -> impl Iterator<Item = &str> + '_ {
    word.split(|c: char| c.is_ascii_punctuation() && c != '\'')
}

fn capitalize_first_letter_of_word_and_lowercase_capitals_preceded_by_capital(
    word: &str,
) -> impl Iterator<Item = char> + '_ {
    word.chars()
        .enumerate()
        .map(capitalize_first_letter)
        .scan(None, lowercase_capitals_preceded_by_capital)
}

fn capitalize_first_letter((idx, c): (usize, char)) -> char {
    match idx {
        0 => c.to_ascii_uppercase(),
        _ => c,
    }
}

fn lowercase_capitals_preceded_by_capital(prev: &mut Option<char>, c: char) -> Option<char> {
    match prev {
        Some(p) if p.is_ascii_uppercase() && c.is_ascii_uppercase() => {
            prev.replace(c);
            Some(c.to_ascii_lowercase())
        }
        _ => {
            prev.replace(c);
            Some(c)
        }
    }
}
