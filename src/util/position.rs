use crate::util::Direction;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Position<const M: usize, const N: usize> {
    pub i: usize,
    pub j: usize,
}

impl<const M: usize, const N: usize> Position<M, N> {
    pub const MAX: Self = Self { i: M - 1, j: N - 1 };

    pub const fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    pub fn add(mut self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Up => self.i = self.i.checked_sub(1)?,
            Direction::Down => {
                self.i += 1;
                if self.i >= M {
                    return None;
                }
            }
            Direction::Left => self.j = self.j.checked_sub(1)?,
            Direction::Right => {
                self.j += 1;
                if self.j >= N {
                    return None;
                }
            }
        }
        Some(self)
    }
}
