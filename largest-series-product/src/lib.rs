#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    if let Some(idx) = string_digits.find(|c: char| !c.is_ascii_digit()) {
        return Err(Error::InvalidDigit(string_digits.chars().nth(idx).unwrap()));
    }
    Ok(string_digits
        .as_bytes()
        .windows(span)
        .map(|w| w.iter().map(|v| *v as u64 - '0' as u64).product())
        .max()
        .unwrap_or(0))
}
