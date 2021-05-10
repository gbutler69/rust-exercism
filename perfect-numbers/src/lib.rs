#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match (num, determine_factors(num).sum::<u64>()) {
        (0, _) => None,
        (num, aliquot_sum) if aliquot_sum == num => Some(Classification::Perfect),
        (num, aliquot_sum) if aliquot_sum > num => Some(Classification::Abundant),
        _ => Some(Classification::Deficient),
    }
}

fn determine_factors(number_to_factor: u64) -> impl Iterator<Item = u64> {
    Factorizer {
        number_to_factor,
        number_to_try_as_factor: 1,
    }
}

pub struct Factorizer {
    number_to_factor: u64,
    number_to_try_as_factor: u64,
}

impl Iterator for Factorizer {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        for fact in self.number_to_try_as_factor..=(self.number_to_factor / 2) {
            self.number_to_try_as_factor += 1;
            let quotient = self.number_to_factor / fact;
            if quotient * fact == self.number_to_factor {
                return Some(fact);
            }
        }
        None
    }
}

// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS
// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS
// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS
// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS
// TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS TESTS

#[test]
fn test_determine_factors_for_1() {
    assert_eq!(Vec::<u64>::new(), determine_factors(1).collect::<Vec<_>>());
}

#[test]
fn test_determine_factors_for_2() {
    assert_eq!(vec![1], determine_factors(2).collect::<Vec<_>>());
}

#[test]
fn test_determine_factors_for_3() {
    assert_eq!(vec![1], determine_factors(3).collect::<Vec<_>>());
}

#[test]
fn test_determine_factors_for_4() {
    assert_eq!(vec![1, 2], determine_factors(4).collect::<Vec<_>>());
}

#[test]
fn test_determine_factors_for_60() {
    assert_eq!(
        vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30],
        determine_factors(60).collect::<Vec<_>>()
    );
}

#[test]
fn test_determine_factors_for_64() {
    assert_eq!(
        vec![1, 2, 4, 8, 16, 32],
        determine_factors(64).collect::<Vec<_>>()
    );
}
