pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|i| i.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    (1..=n)
        .flat_map(|a| (1..=n).map(move |b| (a, b)))
        .filter(|(a, b)| a != b)
        .map(|(a, b)| a * b)
        .sum()
}
