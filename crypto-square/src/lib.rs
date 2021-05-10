pub fn encrypt(input: &str) -> String {
    match input.len() {
        2..=usize::MAX => {
            let (input, cols, rows) = input_to_rectangular_vector(input);
            transpose_input_to_output(input, cols, rows)
        }
        _ => input.into(),
    }
}

fn transpose_input_to_output(input: Vec<u8>, cols: usize, rows: usize) -> String {
    let mut output = String::with_capacity(input.len());
    for col in 0..cols {
        for row in 0..rows {
            output.push(input[row * cols + col] as char);
        }
        if col < cols - 1 {
            output.push(' ')
        };
    }
    output
}

fn input_to_rectangular_vector(input: &str) -> (Vec<u8>, usize, usize) {
    let mut input = input
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    let (cols, rows) = compute_columns_and_rows(input.len());
    while input.len() < cols * rows {
        input.push(' ' as u8);
    }
    (input, cols, rows)
}

fn compute_columns_and_rows(len: usize) -> (usize, usize) {
    let mut cols = (len as f64).sqrt() as usize;
    let mut rows = cols;
    while cols < rows || cols > 1 + rows || cols * rows < len || cols * rows >= len + cols {
        match (cols, rows) {
            (c, r) if c * r < len => cols += 1,
            (c, r) if c > 1 + r => rows += 1,
            (c, r) if c < r => rows -= 1,
            (c, r) if c * r >= len + cols => rows -= 1,
            _ => (),
        }
    }
    (cols, rows)
}
