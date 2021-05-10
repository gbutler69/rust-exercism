/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut place = 10;
    let mut sum = 0;
    for digit in isbn.chars() {
        sum += match (place, digit) {
            (1, 'X') | (1, 'x') => 10,
            (1..=10, digit) if digit.is_digit(10) => (digit as u32 - '0' as u32) * place,
            (_, '-') => continue,
            (_, _) => return false,
        };
        place -= 1;
    }
    sum % 11 == 0 && place == 0
}
