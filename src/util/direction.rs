#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::struct_excessive_bools)]
pub struct DirectionSet {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Direction {
    pub const fn is_horizontal(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::Up | Self::Down)
    }
}

impl DirectionSet {
    pub fn insert(&mut self, direction: Direction) -> bool {
        let val = self.get_mut(direction);
        if *val {
            false
        } else {
            *val = true;
            true
        }
    }

    pub const fn is_empty(self) -> bool {
        !self.up && !self.down && !self.left && !self.right
    }

    fn get_mut(&mut self, direction: Direction) -> &mut bool {
        match direction {
            Direction::Up => &mut self.up,
            Direction::Down => &mut self.down,
            Direction::Left => &mut self.left,
            Direction::Right => &mut self.right,
        }
    }
}
