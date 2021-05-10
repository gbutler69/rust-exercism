pub struct EquationColumn {
    digits: Vec<char>,
    result_digit: char,
}

impl EquationColumn {
    pub fn sum(result: char) -> Self {
        Self {
            digits: Vec::new(),
            result_digit: result,
        }
    }
    pub fn push(&mut self, term: char) {
        self.digits.push(term);
    }
    pub fn if_solution_is_valid_then_return_carry(
        &self,
        carry: u8,
        solution: &impl super::super::solution::EquationSolution,
    ) -> Option<u8> {
        let result = solution.solution_for(self.result_digit).unwrap();
        let (sum, carry) =
            self.calculate_sum_of_terms_using_proposed_solution_digits_and_carry(carry, solution);
        match sum == result {
            true => Some(carry),
            false => None,
        }
    }
    fn calculate_sum_of_terms_using_proposed_solution_digits_and_carry(
        &self,
        carry: u8,
        solution: &impl super::super::solution::EquationSolution,
    ) -> (u8, u8) {
        let summed_terms = self
            .digits
            .iter()
            .map(|cdigit| solution.solution_for(*cdigit))
            .map(|ndigit| ndigit.unwrap() as u16)
            .sum::<u16>()
            + carry as u16;
        let summed_terms_digit = (summed_terms % 10) as u8;
        let carry = (summed_terms / 10) as u8;
        (summed_terms_digit, carry)
    }
}

// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
