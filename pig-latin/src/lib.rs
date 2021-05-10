#![feature(once_cell, custom_inner_attributes)]

pub fn translate(phrase: &str) -> String {
    let phrase_lowercase = phrase.to_lowercase();
    let phrase = phrase_lowercase.as_str();
    let mut translated_phrase = String::new();
    for word in phrase.split(|c: char| c.is_whitespace()) {
        if word.len() > 0 && translated_phrase.len() > 0 {
            translated_phrase += " "
        }
        match word.len() {
            4..=usize::MAX => translated_phrase += word.translate_4_or_more_letter_word().as_str(),
            3 => translated_phrase += word.translate_3_letter_word().as_str(),
            2 => translated_phrase += word.translate_2_letter_word().as_str(),
            1 => translated_phrase += word.translate_1_letter_word().as_str(),
            _ => (),
        }
    }
    translated_phrase
}

trait StrHelper {
    fn translate_4_or_more_letter_word(&self) -> String;
    fn translate_3_letter_word(&self) -> String;
    fn translate_2_letter_word(&self) -> String;
    fn translate_1_letter_word(&self) -> String;
    fn is_single_consonant_sound_at_beginning_usually(&self) -> bool;
    fn and_next_is_2_letter_consonant(&self, next: &str) -> bool;
    fn and_next_2_is_3_letter_consonant(&self, next1: &str, next2: &str) -> bool;
}

impl StrHelper for &str {
    fn translate_4_or_more_letter_word(&self) -> String {
        match (&self[0..=0], &self[1..=1], &self[2..=2], &self[3..]) {
            ("s", "q", "u", rest) => rest.to_string() + "squay",
            ("q", "u", middle, rest) => middle.to_string() + rest + "quay",
            ("y", "t", _, _) | ("x", "r", _, _) => self.to_string() + "ay",
            (first, middle1, middle2, rest)
                if first.and_next_2_is_3_letter_consonant(middle1, middle2) =>
            {
                rest.to_string() + first + middle1 + middle2 + "ay"
            }
            (first, middle1, middle2, rest) if first.and_next_is_2_letter_consonant(middle1) => {
                middle2.to_string() + rest + first + middle1 + "ay"
            }
            (first, middle1, middle2, rest)
                if first.is_single_consonant_sound_at_beginning_usually() =>
            {
                middle1.to_string() + middle2 + rest + first + "ay"
            }
            _ => self.to_string() + "ay",
        }
    }

    fn translate_3_letter_word(&self) -> String {
        match (&self[0..=0], &self[1..=1], &self[2..]) {
            ("y", "t", _) | ("x", "r", _) => self.to_string() + "ay",
            ("q", "u", rest) => rest.to_string() + "quay",
            (first, middle, rest) if first.is_single_consonant_sound_at_beginning_usually() => {
                middle.to_string() + rest + first + "ay"
            }
            _ => self.to_string() + "ay",
        }
    }

    fn translate_2_letter_word(&self) -> String {
        match (&self[0..=0], &self[1..]) {
            (first, rest) if first.is_single_consonant_sound_at_beginning_usually() => {
                rest.to_string() + first + "ay"
            }
            _ => self.to_string() + "ay",
        }
    }

    fn translate_1_letter_word(&self) -> String {
        self.to_string() + "ay"
    }

    fn is_single_consonant_sound_at_beginning_usually(&self) -> bool {
        match self.chars().nth(0) {
            Some(char) => match char {
                'a' | 'e' | 'i' | 'o' | 'u' => false,
                _ => true,
            },
            None => false,
        }
    }

    fn and_next_is_2_letter_consonant(&self, next: &str) -> bool {
        match (&self[0..=0], &next[0..=0]) {
            (first, second)
                if first.is_single_consonant_sound_at_beginning_usually()
                    && second.is_single_consonant_sound_at_beginning_usually() =>
            {
                true
            }
            _ => false,
        }
    }

    fn and_next_2_is_3_letter_consonant(&self, next1: &str, next2: &str) -> bool {
        match (&self[0..=0], &next1[0..=0], &next2[0..=0]) {
            (first, second, third)
                if first.is_single_consonant_sound_at_beginning_usually()
                    && second.is_single_consonant_sound_at_beginning_usually()
                    && third.is_single_consonant_sound_at_beginning_usually()
                    && third != "y" =>
            {
                true
            }
            _ => false,
        }
    }
}
