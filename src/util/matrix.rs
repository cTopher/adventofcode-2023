use std::fmt::{Debug, Write};
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::{fmt, slice};

use crate::util::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<const M: usize, const N: usize, T> {
    data: Box<[[T; N]; M]>,
}

impl<const M: usize, const N: usize, T: Debug> Matrix<M, N, T> {
    pub fn from_str_map<F: Fn(char) -> T + Copy>(s: &str, f: F) -> Self {
        let data = s
            .lines()
            .map(|line| line.chars().map(f).collect::<Vec<T>>().try_into().unwrap())
            .collect::<Vec<[T; N]>>()
            .try_into()
            .unwrap();
        Self { data }
    }

    pub fn rows(&self) -> slice::Iter<'_, [T; N]> {
        self.data.iter()
    }
}

impl<const M: usize, const N: usize, T: Default + Copy> Default for Matrix<M, N, T> {
    fn default() -> Self {
        Self {
            data: Box::new([[T::default(); N]; M]),
        }
    }
}

impl<const M: usize, const N: usize, T> Index<usize> for Matrix<M, N, T> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const M: usize, const N: usize, T> IndexMut<usize> for Matrix<M, N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const M: usize, const N: usize, T> Index<Position<M, N>> for Matrix<M, N, T> {
    type Output = T;

    fn index(&self, Position { i, j }: Position<M, N>) -> &T {
        &self.data[i][j]
    }
}

impl<const M: usize, const N: usize, T> IndexMut<Position<M, N>> for Matrix<M, N, T> {
    fn index_mut(&mut self, Position { i, j }: Position<M, N>) -> &mut T {
        &mut self.data[i][j]
    }
}

impl<const M: usize, const N: usize, T: From<char> + Debug> FromStr for Matrix<M, N, T> {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        Ok(Self::from_str_map(s, T::from))
    }
}

impl<const M: usize, const N: usize, T: fmt::Display> fmt::Display for Matrix<M, N, Option<T>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.iter() {
            for e in row {
                if let Some(e) = e {
                    e.fmt(f)?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
