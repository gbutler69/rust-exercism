use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut word_counts = HashMap::new();
    for word in words.split(|c: char| c.is_whitespace() || c == ',') {
        let word = word
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '\'')
            .flat_map(char::to_lowercase)
            .collect::<String>()
            .trim_matches('\'')
            .to_string();
        if word.is_empty() {
            continue;
        }
        word_counts
            .entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    word_counts
}
