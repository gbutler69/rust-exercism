use self::term::EquationTerm;

use super::{columns::EquationAsColumns, EquationOp};

mod term;

pub struct EquationAsTerms<'a> {
    terms: Vec<EquationTerm<'a>>,
    result: EquationTerm<'a>,
    op: EquationOp,
}

impl<'a> EquationAsTerms<'a> {
    pub fn sum(terms: impl Iterator<Item = &'a str>, result: &'a str) -> Self {
        Self {
            terms: terms.map(|term| EquationTerm::new(term)).collect(),
            result: EquationTerm::new(result),
            op: EquationOp::ADDITION,
        }
    }

    pub fn as_equation_columns(&mut self) -> EquationAsColumns {
        match self.op {
            EquationOp::ADDITION => {
                let mut equation =
                    EquationAsColumns::sum(self.result.digit_chars_lowest_order_first());
                for term in self.terms.iter() {
                    equation.push(term.len(), term.digit_chars_lowest_order_first());
                }
                equation
            }
        }
    }
}

// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS

#[test]
fn test_equation_as_terms_sum_with_empty_terms_validating_terms_and_result() {
    let actual = EquationAsTerms::sum(vec![].into_iter(), "ABC");
    assert!(actual.terms.is_empty());
    assert_eq!(
        actual
            .result
            .digit_chars_lowest_order_first()
            .collect::<Vec<_>>(),
        EquationTerm::new("ABC")
            .digit_chars_lowest_order_first()
            .collect::<Vec<_>>()
    )
}

#[test]
fn test_equation_as_terms_sum_with_3_terms_validating_terms_and_result() {
    let sample_terms = vec!["TQR", "MNO", "VXP"];
    let sample_equation_terms = sample_terms
        .iter()
        .map(|t| EquationTerm::new(t))
        .collect::<Vec<_>>();
    let actual = EquationAsTerms::sum(sample_terms.into_iter(), "ACQY");
    for (actual_term, expected_term) in actual.terms.into_iter().zip(sample_equation_terms) {
        assert_eq!(
            actual_term
                .digit_chars_lowest_order_first()
                .collect::<Vec<_>>(),
            expected_term
                .digit_chars_lowest_order_first()
                .collect::<Vec<_>>()
        );
    }
    assert_eq!(
        actual
            .result
            .digit_chars_lowest_order_first()
            .collect::<Vec<_>>(),
        EquationTerm::new("ACQY")
            .digit_chars_lowest_order_first()
            .collect::<Vec<_>>()
    )
}
