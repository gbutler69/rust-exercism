pub struct EquationTerm<'a> {
    alpha_digits: &'a str,
}

impl<'a> EquationTerm<'a> {
    pub fn new(alpha_digits: &'a str) -> Self {
        match alpha_digits {
            "" => panic!("Empty Term Not Permitted"),
            _ => match alpha_digits
                .chars()
                .enumerate()
                .filter(|(_, c)| !c.is_ascii_alphabetic())
                .nth(0)
            {
                Some((idx, digit)) => panic!(
                    "Invalid Digit in Term (digit #{}, invalid char '{}')",
                    idx, digit
                ),
                None => Self { alpha_digits },
            },
        }
    }
    pub fn digit_chars_lowest_order_first(&'a self) -> impl Iterator<Item = (char, bool)> + 'a {
        self.alpha_digits[1..]
            .chars()
            .map(|c| c.to_ascii_uppercase())
            .map(|c| (c, true))
            .rev()
            .chain(self.alpha_digits[0..1].chars().map(|c| (c, false)))
    }

    pub fn len(&self) -> usize {
        self.alpha_digits.len()
    }
}

// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS

#[test]
#[should_panic(expected = "Empty Term Not Permitted")]
fn test_term_with_empty_term() {
    EquationTerm::new("");
}

#[test]
#[should_panic(expected = "Invalid Digit in Term (digit #2, invalid char '#')")]
fn test_term_with_invalid_nonalpha_character() {
    EquationTerm::new("AB#Z");
}

#[test]
fn test_term_with_1_valid_character() {
    assert_eq!(EquationTerm::new("A").alpha_digits, "A");
}

#[test]
fn test_term_with_multiple_valid_character() {
    assert_eq!(EquationTerm::new("AABALHHJ").alpha_digits, "AABALHHJ");
}

#[test]
fn test_term_with_all_alpha_character() {
    assert_eq!(
        EquationTerm::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz").alpha_digits,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
    );
}

#[test]
fn test_digit_chars_lowest_order_first_ignoring_allow_zeroes_flag() {
    assert_eq!(
        EquationTerm::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
            .digit_chars_lowest_order_first()
            .map(|(digit, _)| digit)
            .collect::<String>(),
        "ZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBA"
    );
}

#[test]
fn test_digit_chars_lowest_order_first_validating_allow_zeroes_flag() {
    assert_eq!(
        EquationTerm::new("ABCD")
            .digit_chars_lowest_order_first()
            .collect::<Vec<(char, bool)>>(),
        vec![('D', true), ('C', true), ('B', true), ('A', false)]
    );
}
