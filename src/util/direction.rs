use crate::util::Orientation;
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub struct DirectionMap<T> {
    up: Option<T>,
    down: Option<T>,
    left: Option<T>,
    right: Option<T>,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub struct DirectionSet {
    map: DirectionMap<()>,
}

impl Direction {
    // pub const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
    pub const fn is_horizontal(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::Up | Self::Down)
    }
    // pub const fn opposite(self) -> Self {
    //     match self {
    //         Self::Up => Self::Down,
    //         Self::Down => Self::Up,
    //         Self::Left => Self::Right,
    //         Self::Right => Self::Left,
    //     }
    // }
    pub const fn perpendicular(self) -> [Self; 2] {
        match self {
            Self::Up | Self::Down => [Self::Left, Self::Right],
            Self::Left | Self::Right => [Self::Up, Self::Down],
        }
    }
    pub const fn orientation(self) -> Orientation {
        match self {
            Self::Up | Self::Down => Orientation::Vertical,
            Self::Left | Self::Right => Orientation::Horizontal,
        }
    }
}

impl<T> DirectionMap<T> {
    // pub const fn get(&self, direction: Direction) -> Option<&T> {
    //     let val = match direction {
    //         Direction::Up => &self.up,
    //         Direction::Down => &self.down,
    //         Direction::Left => &self.left,
    //         Direction::Right => &self.right,
    //     };
    //     val.as_ref()
    // }

    pub fn insert(&mut self, direction: Direction, value: T) -> Option<T> {
        let val = match direction {
            Direction::Up => &mut self.up,
            Direction::Down => &mut self.down,
            Direction::Left => &mut self.left,
            Direction::Right => &mut self.right,
        };
        mem::replace(val, Some(value))
    }

    pub const fn is_empty(&self) -> bool {
        self.up.is_none() && self.down.is_none() && self.left.is_none() && self.right.is_none()
    }
}

impl DirectionSet {
    pub fn insert(&mut self, direction: Direction) -> bool {
        self.map.insert(direction, ()).is_none()
    }

    pub const fn is_empty(self) -> bool {
        self.map.is_empty()
    }
}
