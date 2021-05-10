use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();
    for c in candidate
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
    {
        if seen.contains(&c) {
            return false;
        }
        seen.insert(c);
    }
    true
}
