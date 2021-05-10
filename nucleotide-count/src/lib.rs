#![feature(or_patterns)]

use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'C' | 'T' | 'G' => dna.chars().try_fold(0, |accum, letter| match letter {
            l if l == nucleotide => Ok(accum + 1),
            'A' | 'C' | 'T' | 'G' => Ok(accum),
            l => Err(l),
        }),
        _ => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    counts.insert('A', 0);
    counts.insert('C', 0);
    counts.insert('T', 0);
    counts.insert('G', 0);
    for letter in dna.chars() {
        match letter {
            l @ ('A' | 'C' | 'T' | 'G') => *counts.get_mut(&l).unwrap() += 1,
            l @ _ => return Err(l),
        }
    }
    Ok(counts)
}
