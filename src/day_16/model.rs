use crate::util::{Direction, DirectionSet, Matrix, Position};
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct Grid<const N: usize>(Matrix<N, N, Tile>);

#[derive(Debug, Copy, Clone, Default)]
pub enum Tile {
    #[default]
    EmptySpace,
    ForwardMirror,
    BackwardMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

#[derive(Debug)]
pub struct Beam<'a, const N: usize> {
    grid: &'a Grid<N>,
    energized: u32,
    visited: Matrix<N, N, DirectionSet>,
    sub_beams: Vec<(Position<N, N>, Direction)>,
}

impl<'a, const N: usize> Beam<'a, N> {
    pub fn new(grid: &'a Grid<N>, position: Position<N, N>, direction: Direction) -> Self {
        Self {
            grid,
            energized: 0,
            visited: Matrix::default(),
            sub_beams: vec![(position, direction)],
        }
    }

    pub fn energy(mut self) -> u32 {
        while let Some((position, direction)) = self.sub_beams.pop() {
            self.trace_section(position, direction);
        }
        self.energized
    }

    fn trace_section(&mut self, mut position: Position<N, N>, mut direction: Direction) {
        loop {
            let visited = &mut self.visited[position];
            if visited.is_empty() {
                self.energized += 1;
            }
            if !visited.insert(direction) {
                return;
            }
            match self.grid[position] {
                Tile::ForwardMirror => {
                    direction = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    }
                }
                Tile::BackwardMirror => {
                    direction = match direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    }
                }
                Tile::HorizontalSplitter if direction.is_vertical() => {
                    self.sub_beams.push((position, Direction::Left));
                    self.sub_beams.push((position, Direction::Right));
                    return;
                }
                Tile::VerticalSplitter if direction.is_horizontal() => {
                    self.sub_beams.push((position, Direction::Up));
                    self.sub_beams.push((position, Direction::Down));
                    return;
                }
                _ => {}
            }
            if let Some(p) = position.add(direction) {
                position = p;
            } else {
                return;
            }
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::EmptySpace,
            '/' => Self::ForwardMirror,
            '\\' => Self::BackwardMirror,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            _ => panic!("Invalid element: {c}"),
        }
    }
}

impl<const N: usize> Deref for Grid<N> {
    type Target = Matrix<N, N, Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> FromStr for Grid<N> {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        Ok(Self(Matrix::from_str(s)?))
    }
}
