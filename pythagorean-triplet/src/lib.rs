use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();
    let mut min_b = u32::MAX;
    for a in 1..sum {
        if a >= min_b {
            break;
        }
        for b in 1..sum - a {
            let c = sum - (a + b);
            if a.pow(2) + b.pow(2) == c.pow(2) {
                result.insert([a, b, c]);
                min_b = min_b.min(b);
            }
        }
    }
    result
}
