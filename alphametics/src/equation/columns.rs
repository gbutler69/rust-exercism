use std::collections::HashMap;

use super::solution::EquationSolution;

mod column;

pub struct EquationAsColumns {
    columns: Vec<column::EquationColumn>,
    solver: super::solution::EquationSolverBuilder,
}

impl EquationAsColumns {
    pub fn sum(result: impl Iterator<Item = (char, bool)>) -> Self {
        let mut solver = super::solution::EquationSolverBuilder::new();
        let columns = result
            .scan(&mut solver, |solver, (cdigit, allow_zero)| {
                solver.push_digit(cdigit, allow_zero);
                Some((cdigit, allow_zero))
            })
            .map(|(cdigit, _)| column::EquationColumn::sum(cdigit))
            .collect();
        Self { columns, solver }
    }
    pub fn push(&mut self, term_len: usize, term: impl Iterator<Item = (char, bool)>) {
        while self.columns.len() < term_len {
            self.columns.push(column::EquationColumn::sum('_'));
        }
        for ((term, allow_zero), column) in term.zip(self.columns.iter_mut()) {
            column.push(term);
            self.solver.push_digit(term, allow_zero);
        }
    }
    pub fn solve(&mut self) -> Option<HashMap<char, u8>> {
        let solution = self.solver.solve();
        let mut solution_viable = true;
        let mut not_solved = true;
        'OUTER: while solution_viable && not_solved {
            let mut carry = 0;
            for column in self.columns.iter() {
                match column.if_solution_is_valid_then_return_carry(carry, solution) {
                    Some(new_carry) => carry = new_carry,
                    None => {
                        solution_viable = solution.next_solution();
                        continue 'OUTER;
                    }
                }
            }
            not_solved = false;
        }
        match not_solved {
            true => None,
            false => Some(solution.solution_digits()),
        }
    }
}

// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
// TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS  TESTS
