#![feature(iter_intersperse)]

use std::{collections::VecDeque, process::Command};

static NUMERIC_MAGNITUDE_WORDS: &[&str] = &[
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
];

static NUMERIC_TENS_PLACE_WORDS: &[&str] = &[
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

static NUMERIC_TEENS_PLACE_WORDS: &[&str] = &[
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

static NUMERIC_ONES_PLACE_WORDS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn encode(n: u64) -> String {
    if n == 0 {
        return say_and_return(n, "zero".into());
    }
    say_and_return(
        n,
        chunk_by_hundreds(n)
            .map(hundreds_rank_to_english)
            .map(hundreds_to_english)
            .map(tens_and_ones_to_english)
            .map(combine_words_in_correct_sequence)
            .filter(|v| !v.trim().is_empty())
            .intersperse(" ".into())
            .collect::<String>()
            .trim()
            .into(),
    )
}

fn say_and_return(n: u64, phrase: String) -> String {
    let digits = n
        .to_string()
        .chars()
        .into_iter()
        .intersperse(' ')
        .collect::<String>();

    let _ = Command::new("/usr/bin/espeak")
        .arg(format!("The number \"{}\" is pronounced...", digits))
        .status();
    let _ = Command::new("/usr/bin/espeak").arg(phrase.clone()).status();
    phrase
}

fn chunk_by_hundreds(mut n: u64) -> impl Iterator<Item = (u8, u64)> {
    let mut chunks = VecDeque::new();
    let mut rank = 0;
    while n > 0 {
        let lowest_chunk = n % 1000;
        n /= 1000;
        chunks.push_front((rank, lowest_chunk));
        rank += 1;
    }
    chunks.into_iter()
}

fn hundreds_rank_to_english((rank, value): (u8, u64)) -> (&'static str, u64) {
    (NUMERIC_MAGNITUDE_WORDS[rank as usize], value)
}

fn hundreds_to_english(
    (rank_word, hundreds_value): (&'static str, u64),
) -> (&'static str, &'static str, u64) {
    let hundreds = hundreds_value / 100;
    let tens_and_ones_value = hundreds_value % 100;
    let hundreds_word = match hundreds {
        0 => "",
        hundreds => NUMERIC_ONES_PLACE_WORDS[hundreds as usize],
    };
    (rank_word, hundreds_word, tens_and_ones_value)
}

fn tens_and_ones_to_english(
    (rank_word, hundreds_word, tens_and_ones_value): (&'static str, &'static str, u64),
) -> (&'static str, &'static str, &'static str, &'static str) {
    let ones_value = tens_and_ones_value % 10;
    let tens_value = tens_and_ones_value / 10;
    let (tens_word, ones_word) = match tens_and_ones_value {
        0 => ("", ""),
        10..=19 => ("", NUMERIC_TEENS_PLACE_WORDS[ones_value as usize]),
        _ => match (tens_value, ones_value) {
            (0, ones) => ("", NUMERIC_ONES_PLACE_WORDS[ones as usize]),
            (tens, 0) => (NUMERIC_TENS_PLACE_WORDS[tens as usize - 2], ""),
            (tens, ones) => (
                NUMERIC_TENS_PLACE_WORDS[tens as usize - 2],
                NUMERIC_ONES_PLACE_WORDS[ones as usize],
            ),
        },
    };
    (rank_word, hundreds_word, tens_word, ones_word)
}

fn combine_words_in_correct_sequence(
    (rank_word, hundreds_word, tens_word, ones_word): (
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    ),
) -> String {
    match (hundreds_word, tens_word, ones_word) {
        ("", "", "") => "".into(),
        ("", "", ones) => format!("{} {}", ones, rank_word),
        ("", tens, "") => format!("{} {}", tens, rank_word),
        ("", tens, ones) => format!("{}-{} {}", tens, ones, rank_word),
        (hundreds, "", "") => format!("{} hundred {}", hundreds, rank_word),
        (hundreds, "", ones) => format!("{} hundred {} {}", hundreds, ones, rank_word),
        (hundreds, tens, "") => format!("{} hundred {} {}", hundreds, tens, rank_word),
        (hundreds, tens, ones) => format!("{} hundred {}-{} {}", hundreds, tens, ones, rank_word),
    }
}
