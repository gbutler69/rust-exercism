pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let rows = input.len();
    let cols = input[0].len();
    let mut result_vec = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            let row_max = input[i].iter().map(|v| *v).max().unwrap();
            let col_min = input.iter().map(|r| r[j]).min().unwrap();
            if input[i][j] >= row_max && input[i][j] <= col_min {
                result_vec.push((i, j));
            }
        }
    }
    result_vec
}
