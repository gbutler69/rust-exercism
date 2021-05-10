pub struct PascalsTriangle {
    row_count: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            row_count: row_count as usize,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::<Vec<u32>>::with_capacity(self.row_count);
        for r in 0..self.row_count {
            let mut row = Vec::with_capacity(r + 1);
            for c in 0..=r {
                let sum = match c {
                    c if c > 0 && c < r => rows[r - 1][c - 1] + rows[r - 1][c],
                    _ => 1,
                };
                row.push(sum);
            }
            rows.push(row);
        }
        rows
    }
}
