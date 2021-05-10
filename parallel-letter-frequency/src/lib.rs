use std::collections::HashMap;

use rayon::prelude::*;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap()
        .install(|| {
            input
                .par_iter()
                .with_min_len(10)
                .flat_map(|s| {
                    s.chars()
                        .filter(|c| c.is_alphabetic())
                        .flat_map(char::to_lowercase)
                        .par_bridge()
                })
                .fold(HashMap::new, |mut accum, val| {
                    accum
                        .entry(val)
                        .and_modify(|v| *v += 1)
                        .or_insert_with(|| 1_usize);
                    accum
                })
                .reduce(HashMap::new, |a, b| {
                    b.into_iter().fold(a, |mut accum, (key, val)| {
                        accum.entry(key).and_modify(|v| *v += val).or_insert(val);
                        accum
                    })
                })
        })
}
