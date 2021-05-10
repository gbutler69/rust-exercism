use std::ops::Add;

pub struct Triangle<T: Add<T, Output = T> + PartialOrd + Copy> {
    sides: [T; 3],
}

impl<T: Add<T, Output = T> + PartialOrd + Copy> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        match (
            sides,
            sides[0] + sides[1] >= sides[2],
            sides[1] + sides[2] >= sides[0],
        ) {
            (_, false, _) | (_, _, false) => None,
            ([a, b, c], _, _) if a == a + a || b == b + b || c == c + c => None,
            _ => Some(Self { sides }),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[0] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1]
            && self.sides[0] != self.sides[2]
            && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[0] == self.sides[2]
            || self.sides[1] == self.sides[2]
    }
}
