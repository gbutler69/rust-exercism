// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(mut self) -> Self {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        self
    }

    pub fn advance(mut self) -> Self {
        let (x_delta, y_delta) = match self.direction {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        };
        self.position = (self.position.0 + x_delta, self.position.1 + y_delta);
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for instruction in instructions.chars() {
            self = match instruction {
                'A' => self.advance(),
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                c => panic!(
                    "Invalid Instruction: {} - must be (A)dvance, (L)eft, or (R)ight.",
                    c
                ),
            };
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        self.position.clone()
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
