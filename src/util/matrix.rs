use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::util::Position;

// TODO make this generic over the number of dimensions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<const N: usize, T> {
    data: Box<[[T; N]; N]>,
}

impl<const N: usize, T: Default + Copy> Default for Matrix<N, T> {
    fn default() -> Self {
        Self {
            data: Box::new([[T::default(); N]; N]),
        }
    }
}

impl<const N: usize, T> Index<Position<N>> for Matrix<N, T> {
    type Output = T;

    fn index(&self, Position { i, j }: Position<N>) -> &T {
        &self.data[i][j]
    }
}

impl<const N: usize, T> IndexMut<Position<N>> for Matrix<N, T> {
    fn index_mut(&mut self, Position { i, j }: Position<N>) -> &mut T {
        &mut self.data[i][j]
    }
}

impl<const N: usize, T: From<char> + Debug> FromStr for Matrix<N, T> {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(T::from)
                    .collect::<Vec<T>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[T; N]>>()
            .try_into()
            .unwrap();
        Ok(Self { data })
    }
}
