pub struct Luhn {
    digits: String,
}

impl Luhn {
    #[allow(clippy::char_lit_as_u8)]
    pub fn is_valid(&self) -> bool {
        let mut total_whitespace_chars = 0;
        let digit_doubler =
            |idx: usize, c: char| (c as u8 - '0' as u8) * if idx % 2 == 1 { 2 } else { 1 };
        let digit_normalizer = |c| if c > 9 { c - 9 } else { c };
        let result: Option<u8> = self
            .digits
            .chars()
            .rev()
            .filter(|c| {
                if c.is_ascii_whitespace() {
                    total_whitespace_chars += 1;
                    false
                } else {
                    true
                }
            })
            .map(|c| if c.is_ascii_digit() { Some(c) } else { None })
            .enumerate()
            .map(|(idx, c)| c.map(|c| digit_doubler(idx, c)))
            .map(|c| c.map(digit_normalizer))
            .sum();
        self.digits.len() - total_whitespace_chars > 1
            && match result {
                Some(v) => v % 10 == 0,
                None => false,
            }
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        let s: String = input.to_string();
        Self { digits: s }
    }
}
