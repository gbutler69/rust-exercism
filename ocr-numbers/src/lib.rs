#![feature(iter_intersperse)]

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
    InvalidDigitLinePattern(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines = input.split('\n').collect::<Vec<_>>();
    if lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }
    if lines[0].len() % 3 != 0 {
        return Err(Error::InvalidColumnCount(lines[0].len()));
    }
    lines
        .into_iter()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(digit_line_group_to_digits)
        .map(|digits_for_group| digits_for_group.collect::<Result<String, Error>>())
        .intersperse_with(|| Ok(",".into()))
        .collect()
}

fn digit_line_group_to_digits<'a>(
    digit_line_groups: &[&'a str],
) -> impl Iterator<Item = Result<&'a str, Error>> {
    digit_line_groups[0]
        .as_bytes()
        .chunks(3)
        .zip(digit_line_groups[1].as_bytes().chunks(3))
        .zip(digit_line_groups[2].as_bytes().chunks(3))
        .zip(digit_line_groups[3].as_bytes().chunks(3))
        .map(|(((line1, line2), line3), line4)| [line1, line2, line3, line4])
        .map(parse_numeral_lines_group)
}

fn parse_numeral_lines_group<'a>(digit_lines: [&[u8]; 4]) -> Result<&'a str, Error> {
    match parse_numeral_lines(
        digit_lines
            .iter()
            .map(|v| v.into_iter().map(|c| *c as char).collect::<String>()),
    ) {
        Ok((code, 4)) => Ok(digit_code_to_digit(code)),
        Ok((_, num_lines)) => Err(Error::InvalidRowCount(num_lines)),
        Err(Error::InvalidDigitLinePattern(_)) => Ok("?".into()),
        Err(err) => Err(err),
    }
}

fn parse_numeral_lines(lines: impl Iterator<Item = String>) -> Result<(u8, usize), Error> {
    let mut code = 0b0_000_000_u8;
    let mut num_lines = 0;
    for (idx, line) in lines.enumerate() {
        num_lines += 1;
        code |= parse_numeral_line(line, idx)?;
    }
    Ok((code, num_lines))
}

fn parse_numeral_line(line: String, idx: usize) -> Result<u8, Error> {
    match line.len() {
        3 => match (idx, line.as_str()) {
            (_, "   ") => Ok(0b0_000_000),
            (0, " _ ") => Ok(0b1_000_000),
            (1, "  |") => Ok(0b0_001_000),
            (1, " _ ") => Ok(0b0_010_000),
            (1, " _|") => Ok(0b0_011_000),
            (1, "|  ") => Ok(0b0_100_000),
            (1, "| |") => Ok(0b0_101_000),
            (1, "|_ ") => Ok(0b0_110_000),
            (1, "|_|") => Ok(0b0_111_000),
            (2, "  |") => Ok(0b0_000_001),
            (2, " _ ") => Ok(0b0_000_010),
            (2, " _|") => Ok(0b0_000_011),
            (2, "|  ") => Ok(0b0_000_100),
            (2, "| |") => Ok(0b0_000_101),
            (2, "|_ ") => Ok(0b0_000_110),
            (2, "|_|") => Ok(0b0_000_111),
            _ => Err(Error::InvalidDigitLinePattern(idx)),
        },
        columns => Err(Error::InvalidColumnCount(columns)),
    }
}

fn digit_code_to_digit(code: u8) -> &'static str {
    match code {
        0b1_101_111 => "0",
        0b0_001_001 => "1",
        0b1_011_110 => "2",
        0b1_011_011 => "3",
        0b0_111_001 => "4",
        0b1_110_011 => "5",
        0b1_110_111 => "6",
        0b1_001_001 => "7",
        0b1_111_111 => "8",
        0b1_111_011 => "9",
        _ => "?",
    }
}
