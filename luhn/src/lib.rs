/// Check a Luhn checksum.
#[allow(clippy::char_lit_as_u8)]
pub fn is_valid(code: &str) -> bool {
    let mut total_whitespace_chars = 0;
    let digit_doubler =
        |idx: usize, c: char| (c as u8 - '0' as u8) * if idx % 2 == 1 { 2 } else { 1 };
    let digit_normalizer = |c| if c > 9 { c - 9 } else { c };
    let result: Option<u8> = code
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
    code.len() - total_whitespace_chars > 1
        && match result {
            Some(v) => v % 10 == 0,
            None => false,
        }
}
