#![feature(destructuring_assignment)]

use std::collections::HashMap;

mod equation;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    equation::addition::parse(input)
        .as_equation_columns()
        .solve()
}
