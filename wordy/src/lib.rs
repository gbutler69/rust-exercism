pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    command
        .parse_keywords("What is")?
        .then_parse_expressions()?
        .then_parse_interogative_punctuation()?
        .then_compute_result()
}

trait Keyword {
    fn parse_keywords<'a, 'b>(&'a self, keyword: &'b str) -> Option<&'a str>;
}

impl Keyword for &str {
    fn parse_keywords<'a, 'b>(&'a self, keywords: &'b str) -> Option<&'a str> {
        let input = helper::ignore_whitespace(self);
        match input.starts_with(keywords) {
            true => Some(&input[keywords.len()..]),
            false => None,
        }
    }
}

enum Operation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    EXPONENTIATE,
}

enum Expression {
    BinaryExpression(Operation, i32),
    SingleNumber(i32),
}

trait ParseExpression {
    fn then_parse_expressions(&self) -> Option<(Vec<Expression>, &str)>;

    fn parse_expressions(input: &str) -> Option<(Vec<Expression>, &str)> {
        let mut expressions = Vec::new();
        match Self::parse_number(input) {
            Some((number, remaining_input)) => {
                expressions.push(Expression::SingleNumber(number));
                Self::parse_binary_expressions((expressions, remaining_input))
            }
            None => None,
        }
    }

    fn parse_binary_expressions(
        result: (Vec<Expression>, &str),
    ) -> Option<(Vec<Expression>, &str)> {
        match Self::parse_binary_expressions_remaining(result) {
            Some((_done @ true, expressions, remaining_input)) => {
                Some((expressions, remaining_input))
            }
            Some((_done @ false, ..)) => None,
            None => None,
        }
    }

    fn parse_binary_expressions_remaining(
        result: (Vec<Expression>, &str),
    ) -> Option<(bool, Vec<Expression>, &str)> {
        match Self::parse_binary_expression(result) {
            Some((_done @ true, expressions, remaining_input)) => {
                Some((true, expressions, remaining_input))
            }
            Some((_done @ false, expressions, remaining_input)) => {
                Self::parse_binary_expressions_remaining((expressions, remaining_input))
            }
            None => None,
        }
    }

    fn parse_binary_expression(
        (mut expressions, input): (Vec<Expression>, &str),
    ) -> Option<(bool, Vec<Expression>, &str)> {
        let input = helper::ignore_whitespace(input);
        match input.starts_with("?") {
            true => Some((true, expressions, input)),
            false => match Self::parse_operation(input) {
                Some((operation, remaining_input)) => match Self::parse_number(remaining_input) {
                    Some((number, remaining_input)) => {
                        let remaining_input = if let Operation::EXPONENTIATE = operation {
                            Self::ignore_exponentiation_suffix(remaining_input)
                        } else {
                            Some(remaining_input)
                        };
                        match remaining_input {
                            Some(remaining_input) => {
                                expressions.push(Expression::BinaryExpression(operation, number));
                                Some((false, expressions, remaining_input))
                            }
                            None => None,
                        }
                    }
                    None => None,
                },
                None => None,
            },
        }
    }

    fn parse_operation<'a>(input: &'a str) -> Option<(Operation, &'a str)> {
        let input = helper::ignore_whitespace(input);
        if input.len() > 4 && &input[0..=3] == "plus" {
            Some((Operation::ADD, &input[4..]))
        } else if input.len() > 5 && &input[0..=4] == "minus" {
            Some((Operation::SUBTRACT, &input[5..]))
        } else if input.len() > 13 && &input[0..=12] == "multiplied by" {
            Some((Operation::MULTIPLY, &input[13..]))
        } else if input.len() > 10 && &input[0..=9] == "divided by" {
            Some((Operation::DIVIDE, &input[10..]))
        } else if input.len() > 13 && &input[0..=12] == "raised to the" {
            Some((Operation::EXPONENTIATE, &input[13..]))
        } else {
            None
        }
    }

    fn parse_number(input: &str) -> Option<(i32, &str)> {
        let remaining_input = helper::ignore_whitespace(input);
        let (is_negative, remaining_input) = Self::parse_sign_if_present(remaining_input);
        let (mut accum, mut remaining_input) = Self::parse_numeral(remaining_input)?;
        while let (Some(numeral), remaining) = Self::parse_numeral_if_present(remaining_input) {
            accum = accum * 10 + numeral;
            remaining_input = remaining;
        }
        Some((if is_negative { -accum } else { accum }, remaining_input))
    }

    fn parse_sign_if_present(input: &str) -> (bool, &str) {
        let input = helper::ignore_whitespace(input);
        match &input[0..=0] {
            "-" => (true, &input[1..]),
            "+" => (false, &input[1..]),
            _ => (false, input),
        }
    }

    fn parse_numeral(input: &str) -> Option<(i32, &str)> {
        let input = helper::ignore_whitespace(input);
        match Self::parse_numeral_if_present(input) {
            (Some(numeral), remaining_input) => Some((numeral, remaining_input)),
            _ => None,
        }
    }

    fn parse_numeral_if_present(input: &str) -> (Option<i32>, &str) {
        match input.chars().nth(0) {
            Some(digit_char @ '0'..='9') => (Some(digit_char as i32 - '0' as i32), &input[1..]),
            _ => (None, input),
        }
    }

    fn ignore_exponentiation_suffix(mut input: &str) -> Option<&str> {
        if input.len() > 2 {
            match &input[0..=1] {
                "st" | "nd" | "rd" | "th" => {
                    input = &input[2..];
                    input = helper::ignore_whitespace(input);
                    if input.len() > 5 && &input[0..=4] == "power" {
                        Some(&input[5..])
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

impl ParseExpression for &str {
    fn then_parse_expressions(&self) -> Option<(Vec<Expression>, &str)> {
        Self::parse_expressions(self)
    }
}

trait ParsePunctuation {
    fn then_parse_interogative_punctuation(self) -> Option<Vec<Expression>>;
    fn parse_interogative_punctuation(
        (expressions, input): (Vec<Expression>, &str),
    ) -> Option<Vec<Expression>> {
        let input = helper::ignore_whitespace(input);
        match input.starts_with("?") {
            true => Some(expressions),
            false => None,
        }
    }
}

impl ParsePunctuation for (Vec<Expression>, &str) {
    fn then_parse_interogative_punctuation(self) -> Option<Vec<Expression>> {
        Self::parse_interogative_punctuation(self)
    }
}

trait Compute {
    fn then_compute_result(self) -> Option<i32>;

    fn compute_result_for_expressions(expressions: Vec<Expression>) -> Option<i32> {
        let mut accum = None;
        for expression in expressions {
            accum = Self::accumulate(expression, accum);
            if let None = accum {
                break;
            }
        }
        accum
    }

    fn accumulate(expression: Expression, accum: Option<i32>) -> Option<i32> {
        match expression {
            Expression::BinaryExpression(op, op_arg) => {
                Self::accumulate_operation(accum, op, op_arg)
            }
            Expression::SingleNumber(number) => match accum {
                Some(_) => None,
                None => Some(number),
            },
        }
    }

    fn accumulate_operation(accum: Option<i32>, op: Operation, op_arg: i32) -> Option<i32> {
        match accum {
            Some(value) => match op {
                Operation::ADD => Some(value + op_arg),
                Operation::SUBTRACT => Some(value - op_arg),
                Operation::MULTIPLY => Some(value * op_arg),
                Operation::DIVIDE => Some(value / op_arg),
                Operation::EXPONENTIATE => Some(if op_arg > 0 {
                    value.pow(op_arg as u32)
                } else {
                    1 / value.pow(op_arg.abs() as u32)
                }),
            },
            None => None,
        }
    }
}

impl Compute for Vec<Expression> {
    fn then_compute_result(self) -> Option<i32> {
        Self::compute_result_for_expressions(self)
    }
}

mod helper {
    pub fn ignore_whitespace(input: &str) -> &str {
        let mut i = 0_usize;
        while input[i..].starts_with(char::is_whitespace) {
            i += 1;
        }
        &input[i..]
    }
}
