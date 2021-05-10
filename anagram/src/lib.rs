use std::collections::HashSet;

pub fn anagrams_for<'a, 'b: 'a>(
    word: &'a str,
    possible_anagrams: &'a [&'b str],
) -> HashSet<&'b str> {
    let mut matched_anagrams = HashSet::new();
    let (word_lowercased, word_sorted_lowercased) = sorted_word_letters(word);
    for &possible_anagram in possible_anagrams {
        add_word_to_matched_set_of_anagrams_if_does_not_match_original_word_but_is_an_anagram(
            possible_anagram,
            &word_lowercased,
            &word_sorted_lowercased,
            &mut matched_anagrams,
        );
    }
    matched_anagrams
}

fn add_word_to_matched_set_of_anagrams_if_does_not_match_original_word_but_is_an_anagram<
    'a,
    'b,
    'c: 'a,
>(
    possible_anagram: &'c str,
    word_lowercased: &'b Vec<char>,
    word_sorted_lowercased: &'b Vec<char>,
    matched_anagrams: &'a mut HashSet<&'c str>,
) {
    let (pa_lowercased, pa_sorted_lowercased) = sorted_word_letters(possible_anagram);
    if pa_lowercased != *word_lowercased && pa_sorted_lowercased == *word_sorted_lowercased {
        matched_anagrams.insert(possible_anagram);
    }
}

fn sorted_word_letters(word: &str) -> (Vec<char>, Vec<char>) {
    let word_letters = word
        .chars()
        .flat_map(char::to_lowercase)
        .collect::<Vec<_>>();
    let mut sorted_word_letters = word_letters.clone();
    sorted_word_letters.sort_unstable();
    (word_letters, sorted_word_letters)
}
