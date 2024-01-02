use crate::util::Direction;
use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vector2D {
    pub x: i64,
    pub y: i64,
}

impl Vector2D {
    pub const ZERO: Self = Self::new(0, 0);

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn cross_product(self, rhs: Self) -> i64 {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl From<Direction> for Vector2D {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new(0, 1),
            Direction::Down => Self::new(0, -1),
            Direction::Left => Self::new(-1, 0),
            Direction::Right => Self::new(1, 0),
        }
    }
}

impl Mul<i64> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
