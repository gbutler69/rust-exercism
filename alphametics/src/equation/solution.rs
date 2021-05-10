use std::collections::HashMap;
pub struct EquationSolverBuilder(EquationSolver);
pub struct EquationSolver {
    digits: Vec<EquationSolverDigit>,
    digits_index: HashMap<char, usize>,
}
struct EquationSolverDigit {
    digit: u8,
    allow_zero: bool,
}

pub trait EquationSolution {
    fn solution_for(&self, alpha_digit: char) -> Option<u8>;
    fn solution_digits(&self) -> HashMap<char, u8>;
    fn next_solution(&mut self) -> bool;
}

impl EquationSolverBuilder {
    pub fn new() -> Self {
        Self(EquationSolver::init())
    }
    pub fn push_digit(&mut self, alpha_digit: char, allow_zero: bool) {
        self.0.push_digit(alpha_digit, allow_zero);
    }
    pub fn solve(&mut self) -> &mut impl EquationSolution {
        if self.0.solution_contains_repeats() {
            self.0.next_solution();
        }
        &mut self.0
    }
}

impl EquationSolver {
    fn init() -> Self {
        Self {
            digits: Vec::new(),
            digits_index: HashMap::new(),
        }
    }
    fn push_digit(&mut self, alpha_digit: char, allow_zero: bool) {
        let (digits_index, digits) = (&mut self.digits_index, &mut self.digits);
        digits_index
            .entry(alpha_digit)
            .and_modify(|idx| digits[*idx].disallow_zero(!allow_zero))
            .or_insert_with(|| {
                digits.push(EquationSolverDigit::with(allow_zero));
                digits.len() - 1
            });
    }
    fn next_solution(&mut self) -> bool {
        let mut ok = self.increment_solution();
        while self.solution_contains_repeats() && ok {
            ok = self.increment_solution();
        }
        ok
    }
    fn solution_contains_repeats(&self) -> bool {
        for i in 0..self.digits.len() {
            for j in (i + 1)..self.digits.len() {
                if self.digits[i].digit == self.digits[j].digit {
                    return true;
                }
            }
        }
        false
    }
    fn increment_solution(&mut self) -> bool {
        let mut digit_number: usize = 0;
        let max_digit_idx = self.digits.len() - 1;
        loop {
            match &mut self.digits[digit_number] {
                EquationSolverDigit { digit: 9, .. } if digit_number == max_digit_idx => {
                    return false;
                }
                EquationSolverDigit {
                    digit: digit @ 9,
                    allow_zero,
                    ..
                } => {
                    *digit = if *allow_zero { 0 } else { 1 };
                    digit_number += 1;
                    continue;
                }
                EquationSolverDigit { digit, .. } => {
                    *digit += 1;
                    return true;
                }
            }
        }
    }
}

impl EquationSolution for EquationSolver {
    fn solution_for(&self, alpha_digit: char) -> Option<u8> {
        self.digits_index
            .get(&alpha_digit)
            .map(|idx| self.digits[*idx].digit)
    }
    fn solution_digits(&self) -> HashMap<char, u8> {
        self.digits_index
            .iter()
            .map(move |(c, idx)| (*c, self.digits[*idx].digit))
            .collect()
    }
    fn next_solution(&mut self) -> bool {
        EquationSolver::next_solution(self)
    }
}

impl EquationSolverDigit {
    fn with(allow_zero: bool) -> Self {
        match allow_zero {
            true => Self {
                digit: 0,
                allow_zero,
            },
            false => Self {
                digit: 1,
                allow_zero,
            },
        }
    }
    fn disallow_zero(&mut self, disallow: bool) {
        self.allow_zero = self.allow_zero && !disallow;
    }
}

// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
