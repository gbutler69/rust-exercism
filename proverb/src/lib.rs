use std::{iter::Peekable, slice::Iter};

pub fn build_proverb(word_list: &[&str]) -> String {
    let mut words = word_list.into_iter().peekable();
    let lines = compute_lines_from(&mut words, word_list.len());
    lines.into_iter().collect::<String>()
}

fn compute_lines_from(words: &mut Peekable<Iter<&str>>, words_len: usize) -> Vec<String> {
    let end_line = compute_end_line_by_peeking_at(words);
    let mut lines = compute_beginning_lines_from(words, words_len);
    lines.push(end_line);
    lines
}

fn compute_beginning_lines_from(words: &mut Peekable<Iter<&str>>, words_len: usize) -> Vec<String> {
    let mut lines = Vec::<String>::with_capacity(words_len);
    while let Some(&word) = words.next() {
        lines.push(compute_line_from(word, words.peek()));
    }
    lines
}

fn compute_line_from(word: &str, next_word: Option<&&&str>) -> String {
    match next_word {
        Some(&&next_word) => {
            format!("For want of a {} the {} was lost.\n", word, next_word)
        }
        None => "".into(),
    }
}

fn compute_end_line_by_peeking_at(words: &mut Peekable<Iter<&str>>) -> String {
    match words.peek() {
        Some(&&first_s) => format!("And all for the want of a {}.", first_s),
        None => "".into(),
    }
}
