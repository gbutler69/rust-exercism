use crate::orientation::{Direction, TurnDirection};

pub enum DrawError {
    OutOfBounds,
}

pub trait DrawTarget<T: Clone + Default + PartialEq> {
    fn draw_at(&mut self, row: isize, col: isize, value: T) -> Result<(), DrawError>;
    fn look_at(&mut self, row: isize, col: isize) -> Result<T, DrawError>;
}

pub struct Turtle {
    position: (isize, isize),
}

impl Turtle {
    pub fn new() -> Self {
        Self { position: (0, 0) }
    }
    pub fn fill_clockwise<T: Clone + Default + PartialEq>(
        &mut self,
        start_at: (isize, isize),
        heading: Direction,
        mut pen: impl Iterator<Item = T>,
        target: &mut impl DrawTarget<T>,
    ) {
        self.position = start_at;
        let mut direction = heading;
        let mut last_position: (isize, isize) = start_at;
        loop {
            match self.draw_until_edge_or_read_next_contains_other_than(
                &[T::default()],
                &direction,
                &mut pen,
                target,
            ) {
                (row, col) if row == last_position.0 && col == last_position.1 => break,
                (row, col) => last_position = (row, col),
            }
            direction = direction.turn(TurnDirection::Clockwise);
        }
    }
    fn draw_until_edge_or_read_next_contains_other_than<T: Clone + Default + PartialEq>(
        &mut self,
        any_of: &[T],
        heading: &Direction,
        pen: &mut impl Iterator<Item = T>,
        target: &mut impl DrawTarget<T>,
    ) -> (isize, isize) {
        loop {
            self.draw_current_position(pen, target);
            let lookahead1 = self.lookahead(1, heading, target);
            let lookahead2 = self.lookahead(2, heading, target);
            match (lookahead1, lookahead2) {
                (Ok(value1), Ok(value2))
                    if any_of.contains(&value1) && any_of.contains(&value2) =>
                {
                    self.forward(1, heading);
                }
                (Ok(value1), _) if any_of.contains(&value1) => {
                    self.forward(1, heading);
                    return self.position;
                }
                _ => return self.position,
            }
        }
    }
    fn draw_current_position<T: Clone + Default + PartialEq>(
        &self,
        pen: &mut impl Iterator<Item = T>,
        target: &mut impl DrawTarget<T>,
    ) {
        let _ = target.draw_at(self.position.0, self.position.1, pen.next().unwrap());
    }
    fn move_to(&mut self, row: isize, col: isize) {
        self.position = (row, col);
    }
    fn forward(&mut self, distance: isize, heading: &Direction) {
        let new_position = self.calculate_relative_position(heading, distance);
        self.move_to(new_position.0, new_position.1);
    }
    fn lookahead<T: Clone + Default + PartialEq>(
        &self,
        distance: isize,
        heading: &Direction,
        target: &mut impl DrawTarget<T>,
    ) -> Result<T, DrawError> {
        let look_position = self.calculate_relative_position(heading, distance);
        target.look_at(look_position.0, look_position.1)
    }
    fn calculate_relative_position(&self, heading: &Direction, distance: isize) -> (isize, isize) {
        let look_position = match heading {
            Direction::North => (self.position.0 - distance, self.position.1),
            Direction::South => (self.position.0 + distance, self.position.1),
            Direction::East => (self.position.0, self.position.1 + distance),
            Direction::West => (self.position.0, self.position.1 - distance),
        };
        look_position
    }
}
