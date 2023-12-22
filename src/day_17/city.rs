use crate::util::{Direction, Matrix, OrientationSet, Position};
use itertools::unfold;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Map<const M: usize, const N: usize> {
    heat_loss: Matrix<M, N, u32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Crucible<const M: usize, const N: usize> {
    heat_loss: u32,
    position: Position<M, N>,
    direction: Direction,
}

impl<const M: usize, const N: usize> Map<M, N> {
    pub fn least_heat_loss(&self, min_move: usize, max_move: usize) -> u32 {
        let mut paths = BinaryHeap::new();
        paths.push(Crucible {
            heat_loss: 0,
            position: Position::default(),
            direction: Direction::Down,
        });
        paths.push(Crucible {
            heat_loss: 0,
            position: Position::default(),
            direction: Direction::Right,
        });
        let mut visited: Matrix<M, N, OrientationSet> = Matrix::default();
        while let Some(crucible) = paths.pop() {
            if crucible.position == Position::MAX {
                return crucible.heat_loss;
            }
            if !visited[crucible.position].insert(crucible.direction.orientation()) {
                continue;
            }
            for direction in crucible.direction.perpendicular() {
                unfold(crucible, |c| {
                    let res = c.goto(direction, &self.heat_loss);
                    if let Some(res) = res {
                        *c = res;
                    }
                    res
                })
                .take(max_move)
                .skip(min_move - 1)
                .for_each(|crucible| paths.push(crucible));
            }
        }
        panic!("No path found");
    }
}

impl<const M: usize, const N: usize> Crucible<M, N> {
    fn goto(mut self, direction: Direction, heat_losses: &Matrix<M, N, u32>) -> Option<Self> {
        self.direction = direction;
        self.position = self.position.add(direction)?;
        self.heat_loss += heat_losses[self.position];
        Some(self)
    }
}

impl<const M: usize, const N: usize> Ord for Crucible<M, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl<const M: usize, const N: usize> PartialOrd for Crucible<M, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const M: usize, const N: usize> FromStr for Map<M, N> {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let heat_loss = Matrix::from_str_map(s, |c| c.to_digit(10).unwrap());
        Ok(Self { heat_loss })
    }
}
