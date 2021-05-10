pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<'a> Luhn for &'a str {
    #[allow(clippy::char_lit_as_u8)]

    fn valid_luhn(&self) -> bool {
        let mut total_whitespace_chars = 0;
        let digit_doubler =
            |idx: usize, c: char| (c as u8 - '0' as u8) * if idx % 2 == 1 { 2 } else { 1 };
        let digit_normalizer = |c| if c > 9 { c - 9 } else { c };
        let result: Option<u8> = self
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
        self.len() - total_whitespace_chars > 1
            && match result {
                Some(v) => v % 10 == 0,
                None => false,
            }
    }
}

impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        self.as_str().valid_luhn()
    }
}

pub trait NaturalNumber: Copy {}

impl NaturalNumber for u8 {}
impl NaturalNumber for u16 {}
impl NaturalNumber for u32 {}
impl NaturalNumber for u64 {}

impl<T: Into<u64> + NaturalNumber> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let mut num: u64 = (*self).into();
        let mut sum_mapped_digits = 0;
        for i in 0.. {
            let mut digit_val = num % 10 * (if i % 2 == 0 { 1 } else { 2 });
            if digit_val > 9 {
                digit_val -= 9;
            }
            num /= 10;
            sum_mapped_digits += digit_val;
            if num == 0 {
                break;
            }
        }
        sum_mapped_digits % 10 == 0
    }
}

impl Luhn for usize {
    fn valid_luhn(&self) -> bool {
        (*self as u64).valid_luhn()
    }
}
