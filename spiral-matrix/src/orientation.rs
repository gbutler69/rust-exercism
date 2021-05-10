pub enum Direction {
    North,
    South,
    East,
    West,
}

pub enum TurnDirection {
    Clockwise,
    #[allow(dead_code)]
    CounterClockwise,
}

impl Direction {
    pub fn turn(&self, direction: TurnDirection) -> Self {
        match direction {
            TurnDirection::Clockwise => match self {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
            TurnDirection::CounterClockwise => match self {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
        }
    }
}
