use std::cell::RefCell;

pub struct Matrix<T: Default> {
    elements: RefCell<Vec<T>>,
    row_size: usize,
    column_size: usize,
}

impl<T: Clone + Default> Matrix<T> {
    pub fn new(rows: usize, columns: usize, fill_with: T) -> Self {
        if rows >= isize::MAX as usize || columns >= isize::MAX as usize {
            panic!("Maximum matrix size (rows,cols) isize::MAX - 1")
        }
        Self {
            elements: RefCell::new(vec![fill_with; rows * columns]),
            row_size: rows,
            column_size: columns,
        }
    }
    pub fn get<'a>(&'a self, row: usize, column: usize) -> T {
        self.elements.borrow()[self.row_size * row + column].clone()
    }
    pub fn set<'a>(&'a self, row: usize, column: usize, value: T) {
        self.elements.borrow_mut()[self.row_size * row + column] = value;
    }
    pub fn unflatten_to_owned(&self) -> Vec<Vec<T>> {
        let mut out = Vec::new();
        for row in 0..self.column_size {
            let mut out_row = Vec::new();
            for column in 0..self.row_size {
                out_row.push(self.get(row, column).clone());
            }
            out.push(out_row);
        }
        out
    }
}

impl<T: Clone + Default + PartialEq> crate::draw::DrawTarget<T> for Matrix<T> {
    fn draw_at(&mut self, row: isize, col: isize, value: T) -> Result<(), crate::draw::DrawError> {
        if row < 0 || col < 0 || row >= self.row_size as isize || col >= self.column_size as isize {
            return Err(crate::draw::DrawError::OutOfBounds);
        }
        self.set(row as usize, col as usize, value);
        Ok(())
    }

    fn look_at(&mut self, row: isize, col: isize) -> Result<T, crate::draw::DrawError> {
        if row < 0 || col < 0 || row >= self.row_size as isize || col >= self.column_size as isize {
            return Err(crate::draw::DrawError::OutOfBounds);
        }
        Ok(self.get(row as usize, col as usize))
    }
}
