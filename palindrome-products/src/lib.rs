#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Self {
        let value = a * b;
        Self::assert_value_is_palindrome(value, a, b);
        Self {
            value,
            factors: vec![(a, b)],
        }
    }
    fn assert_value_is_palindrome(value: u64, a: u64, b: u64) {
        debug_assert!(
            Self::is_palindrome(value),
            "Product is not a palindrome: {} * {} = {}",
            a,
            b,
            a * b
        );
    }
    pub fn empty() -> Self {
        Self {
            value: 0,
            factors: Vec::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.factors.is_empty()
    }
    pub fn is_palindrome(value: u64) -> bool {
        let num_digits = (value as f64).log10() as u32 + 1;
        for i in 0..((num_digits as f64 / 2_f64).ceil() as u32) {
            let digit1 = value / 10_u64.pow(i) % 10;
            let digit2 = value / 10_u64.pow(num_digits - i - 1) % 10;
            if digit1 != digit2 {
                return false;
            }
        }
        true
    }
    pub fn value(&self) -> u64 {
        self.value
    }
    pub fn insert(&mut self, a: u64, b: u64) {
        if a * b != self.value {
            panic!(
                "Invalid Factors ({},{}) for Palindrome {}",
                a,
                b,
                self.value()
            )
        }
        self.factors.push((a, b));
    }
    pub fn insert_or_replace_max(&mut self, value: u64, a: u64, b: u64) {
        assert!(
            value != 0,
            "Zero (0) is not considered a palindromic number"
        );
        if value > self.value || self.value == 0 {
            self.value = value;
            self.factors.clear();
            self.insert(a, b);
        } else if value == self.value {
            self.insert(a, b);
        }
    }
    pub fn insert_or_replace_min(&mut self, value: u64, a: u64, b: u64) {
        assert!(
            value != 0,
            "Zero (0) is not considered a palindromic number"
        );
        if value < self.value || self.value == 0 {
            self.value = value;
            self.factors.clear();
            self.insert(a, b);
        } else if value == self.value {
            self.insert(a, b);
        }
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut max_palindrome = Palindrome::empty();
    let mut min_palindrome = Palindrome::empty();
    for i in min..=max {
        for j in i..=max {
            let value = i * j;
            if let true = Palindrome::is_palindrome(value) {
                max_palindrome.insert_or_replace_max(value, i, j);
                min_palindrome.insert_or_replace_min(value, i, j);
            }
        }
    }
    match (min_palindrome, max_palindrome) {
        (p1, p2) if p1.is_empty() && p2.is_empty() => None,
        (p1, p2) => Some((p1, p2)),
    }
}
