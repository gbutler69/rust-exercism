use super::terms::EquationAsTerms;

/// Parses an equation of the form: ```ABC + CBA + ... + XYZ == TUVW``` and returns an ```EquationAsTerms``` type
pub fn parse<'a>(input: &'a str) -> EquationAsTerms<'a> {
    let mut equation_sides = input.split("==").into_iter().map(|s| s.trim());
    EquationAsTerms::sum(
        parse_addition_terms(equation_sides.next().expect("expected equation with terms on left side and result on right side of equal sign. string was empty it appears.")),
        equation_sides.next().expect("expected equation with terms on left side and result on right side of equal sign. string missing equals sign or missing right side of equation."),
    )
}

fn parse_addition_terms(addition_terms: &str) -> impl Iterator<Item = &str> {
    addition_terms.split('+').map(|t| t.trim())
}

// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS

#[test]
fn test_parse_addition_terms_empty_string() {
    let expected: &[&str] = &[""];
    let actual = parse_addition_terms("").collect::<Vec<&str>>();
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_addition_terms_with_1_term() {
    let expected: &[&str] = &["ABC"];
    let actual = parse_addition_terms("ABC").collect::<Vec<&str>>();
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_addition_terms_with_5_terms() {
    let expected: &[&str] = &["ABC", "DEF", "GHI", "K", "A"];
    let actual = parse_addition_terms("ABC  +   DEF + GHI +K+ A").collect::<Vec<&str>>();
    assert_eq!(actual, expected);
}
