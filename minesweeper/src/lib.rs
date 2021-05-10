pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    let cols = if rows > 0 { minefield[0].len() } else { 0 };
    let mut result = vec!["".into(); rows];
    for row in 0..rows {
        for col in 0..cols {
            result[row] += count_around(minefield, rows - 1, cols - 1, row, col)
                .to_string()
                .as_str();
        }
    }
    result
}

fn count_around(
    minefield: &[&str],
    max_row: usize,
    max_col: usize,
    row: usize,
    col: usize,
) -> char {
    if minefield[row].chars().nth(col).unwrap() == '*' {
        return '*';
    }
    let mut accum = 0_u8;
    count_top_left(row, col, minefield, &mut accum);
    count_top_right(row, col, max_col, minefield, &mut accum);
    count_bottom_left(row, max_row, col, minefield, &mut accum);
    count_bottom_right(row, max_row, col, max_col, minefield, &mut accum);
    count_top(row, minefield, col, &mut accum);
    count_left(col, minefield, row, &mut accum);
    count_right(col, max_col, minefield, row, &mut accum);
    count_bottom(row, max_row, minefield, col, &mut accum);
    if accum == 0 {
        ' '
    } else {
        (accum + '0' as u8) as char
    }
}

fn count_top_left(row: usize, col: usize, minefield: &[&str], accum: &mut u8) {
    if row > 0 && col > 0 && minefield[row - 1].chars().nth(col - 1).unwrap() == '*' {
        *accum += 1;
    }
}

fn count_top_right(row: usize, col: usize, max_col: usize, minefield: &[&str], accum: &mut u8) {
    if row > 0 && col < max_col && minefield[row - 1].chars().nth(col + 1).unwrap() == '*' {
        *accum += 1;
    }
}

fn count_bottom_left(row: usize, max_row: usize, col: usize, minefield: &[&str], accum: &mut u8) {
    if row < max_row && col > 0 && minefield[row + 1].chars().nth(col - 1).unwrap() == '*' {
        *accum += 1;
    }
}

fn count_bottom_right(
    row: usize,
    max_row: usize,
    col: usize,
    max_col: usize,
    minefield: &[&str],
    accum: &mut u8,
) {
    if row < max_row && col < max_col && minefield[row + 1].chars().nth(col + 1).unwrap() == '*' {
        *accum += 1;
    }
}

fn count_top(row: usize, minefield: &[&str], col: usize, accum: &mut u8) {
    if row > 0 && minefield[row - 1].chars().nth(col).unwrap() == '*' {
        *accum += 1;
    }
}

fn count_left(col: usize, minefield: &[&str], row: usize, accum: &mut u8) {
    if col > 0 && minefield[row].chars().nth(col - 1).unwrap() == '*' {
        *accum += 1;
    }
}

fn count_right(col: usize, max_col: usize, minefield: &[&str], row: usize, accum: &mut u8) {
    if col < max_col && minefield[row].chars().nth(col + 1).unwrap() == '*' {
        *accum += 1;
    }
}

fn count_bottom(row: usize, max_row: usize, minefield: &[&str], col: usize, accum: &mut u8) {
    if row < max_row && minefield[row + 1].chars().nth(col).unwrap() == '*' {
        *accum += 1;
    }
}
