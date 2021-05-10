use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&key, vec)| vec.iter().map(move |&val| (val.to_ascii_lowercase(), key)))
        .collect()
}
