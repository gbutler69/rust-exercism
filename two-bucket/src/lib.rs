#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

struct Constraints<'a> {
    pub capacity_1: u8,
    pub capacity_2: u8,
    pub goal: u8,
    pub start_bucket: &'a Bucket,
}

struct Position<'a> {
    fill_level_1: u8,
    fill_level_2: u8,
    previous: Option<&'a Position<'a>>,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let (fill_level_1, fill_level_2) = match start_bucket {
        Bucket::One => (capacity_1, 0),
        Bucket::Two => (0, capacity_2),
    };
    let constraints = Constraints {
        capacity_1,
        capacity_2,
        goal,
        start_bucket,
    };
    let position = Position {
        fill_level_1,
        fill_level_2,
        previous: None,
    };
    find_best_solution(1, &position, &constraints)
}

fn find_best_solution(
    move_number: u8,
    prior_positions: &Position,
    constraints: &Constraints,
) -> Option<BucketStats> {
    let (start_bucket, goal, capacity_1, capacity_2) = (
        constraints.start_bucket,
        constraints.goal,
        constraints.capacity_1,
        constraints.capacity_2,
    );
    let (fill_level_1, fill_level_2) = (prior_positions.fill_level_1, prior_positions.fill_level_2);
    match (fill_level_1, fill_level_2) {
        (fl1, _) if fl1 == goal => Some(BucketStats {
            moves: move_number,
            goal_bucket: Bucket::One,
            other_bucket: fill_level_2,
        }),
        (_, fl2) if fl2 == goal => Some(BucketStats {
            moves: move_number,
            goal_bucket: Bucket::Two,
            other_bucket: fill_level_1,
        }),
        (fl1, fl2) if fl1 == capacity_1 && *start_bucket == Bucket::Two && fl2 == 0 => None,
        (fl1, fl2) if fl2 == capacity_2 && *start_bucket == Bucket::One && fl1 == 0 => None,
        (fl1, fl2) if already_visited(prior_positions, &(fl1, fl2)) => None,
        _ => {
            let empty_1 = match (fill_level_1, fill_level_2) {
                (0, _) | (_, 0) => None,
                _ => find_best_solution(
                    move_number + 1,
                    &Position {
                        fill_level_1: 0,
                        fill_level_2,
                        previous: Some(prior_positions),
                    },
                    constraints,
                ),
            };
            let empty_2 = match (fill_level_1, fill_level_2) {
                (_, 0) | (0, _) => None,
                _ => find_best_solution(
                    move_number + 1,
                    &Position {
                        fill_level_1,
                        fill_level_2: 0,
                        previous: Some(prior_positions),
                    },
                    constraints,
                ),
            };
            let fill_1 = match fill_level_1 {
                fl1 if fl1 == capacity_1 => None,
                _ => find_best_solution(
                    move_number + 1,
                    &Position {
                        fill_level_1: capacity_1,
                        fill_level_2,
                        previous: Some(prior_positions),
                    },
                    constraints,
                ),
            };
            let fill_2 = match fill_level_2 {
                fl2 if fl2 == capacity_2 => None,
                _ => find_best_solution(
                    move_number + 1,
                    &Position {
                        fill_level_1,
                        fill_level_2: capacity_2,
                        previous: Some(prior_positions),
                    },
                    constraints,
                ),
            };
            let pour_1_to_2 = match (fill_level_1, fill_level_2) {
                (fl1, _) if fl1 == 0 => None,
                (_, fl2) if fl2 == capacity_2 => None,
                (fl1, fl2) if fl1 > capacity_2 - fl2 => find_best_solution(
                    move_number + 1,
                    &Position {
                        fill_level_1: fill_level_1 - (capacity_2 - fl2),
                        fill_level_2: capacity_2,
                        previous: Some(prior_positions),
                    },
                    constraints,
                ),
                (fl1, fl2) if fl1 <= capacity_2 - fl2 => find_best_solution(
                    move_number + 1,
                    &Position {
                        fill_level_1: 0,
                        fill_level_2: fill_level_2 + fill_level_1,
                        previous: Some(prior_positions),
                    },
                    constraints,
                ),
                (_, _) => None,
            };
            let pour_2_to_1 = match (fill_level_1, fill_level_2) {
                (fl1, _) if fl1 == capacity_1 => None,
                (_, fl2) if fl2 == 0 => None,
                (fl1, fl2) if fl2 > capacity_1 - fl1 => find_best_solution(
                    move_number + 1,
                    &Position {
                        fill_level_1: capacity_1,
                        fill_level_2: fill_level_2 - (capacity_1 - fl1),
                        previous: Some(prior_positions),
                    },
                    constraints,
                ),
                (fl1, fl2) if fl1 <= capacity_2 - fl2 => find_best_solution(
                    move_number + 1,
                    &Position {
                        fill_level_1: fill_level_1 + fill_level_2,
                        fill_level_2: 0,
                        previous: Some(prior_positions),
                    },
                    constraints,
                ),
                (_, _) => None,
            };
            [empty_1, empty_2, fill_1, fill_2, pour_1_to_2, pour_2_to_1]
                .iter_mut()
                .filter(|v| (**v).is_some())
                .map(|v| (*v).take().unwrap())
                .min_by(|a, b| a.moves.cmp(&b.moves))
        }
    }
}

fn already_visited(prior_positions: &Position, current_position: &(u8, u8)) -> bool {
    let mut prior_position = prior_positions.previous;
    loop {
        match prior_position {
            Some(prior) => match *current_position == (prior.fill_level_1, prior.fill_level_2) {
                true => return true,
                false => {
                    prior_position = prior.previous;
                    continue;
                }
            },
            None => return false,
        }
    }
}
