use crate::util::Matrix;
use std::fmt;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Platform<const M: usize, const N: usize> {
    rocks: Matrix<M, N, Option<Rock>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rock {
    Rounded,
    CubeShaped,
}

//noinspection DuplicatedCode
impl<const M: usize, const N: usize> Platform<M, N> {
    pub fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn tilt_north(&mut self) {
        for i in 1..M {
            for j in 0..N {
                if self.rocks[i][j] == Some(Rock::Rounded) {
                    if let Some(k) = (0..i)
                        .rev()
                        .take_while(|&k| self.rocks[k][j].is_none())
                        .last()
                    {
                        self.rocks[i][j] = None;
                        self.rocks[k][j] = Some(Rock::Rounded);
                    }
                }
            }
        }
    }

    pub fn tilt_west(&mut self) {
        for i in 0..M {
            for j in 1..N {
                if self.rocks[i][j] == Some(Rock::Rounded) {
                    if let Some(k) = (0..j)
                        .rev()
                        .take_while(|&k| self.rocks[i][k].is_none())
                        .last()
                    {
                        self.rocks[i][j] = None;
                        self.rocks[i][k] = Some(Rock::Rounded);
                    }
                }
            }
        }
    }

    pub fn tilt_south(&mut self) {
        for i in (0..M - 1).rev() {
            for j in 0..N {
                if self.rocks[i][j] == Some(Rock::Rounded) {
                    if let Some(k) = (i + 1..M)
                        .take_while(|&k| self.rocks[k][j].is_none())
                        .last()
                    {
                        self.rocks[i][j] = None;
                        self.rocks[k][j] = Some(Rock::Rounded);
                    }
                }
            }
        }
    }

    pub fn tilt_east(&mut self) {
        for i in 0..M {
            for j in (0..N - 1).rev() {
                if self.rocks[i][j] == Some(Rock::Rounded) {
                    if let Some(k) = (j + 1..N)
                        .take_while(|&k| self.rocks[i][k].is_none())
                        .last()
                    {
                        self.rocks[i][j] = None;
                        self.rocks[i][k] = Some(Rock::Rounded);
                    }
                }
            }
        }
    }

    pub fn north_support_beam_load(&self) -> usize {
        self.rocks
            .rows()
            .rev()
            .enumerate()
            .map(|(index, row)| {
                let rounded_rocks = row
                    .iter()
                    .filter(|&&rock| rock == Some(Rock::Rounded))
                    .count();
                (index + 1) * rounded_rocks
            })
            .sum()
    }
}

impl Rock {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'O' => Some(Self::Rounded),
            '#' => Some(Self::CubeShaped),
            '.' => None,
            _ => panic!("invalid rock char {c}"),
        }
    }
}

impl fmt::Display for Rock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char = match self {
            Self::Rounded => 'O',
            Self::CubeShaped => '#',
        };
        f.write_char(char)
    }
}

impl<const M: usize, const N: usize> fmt::Display for Platform<M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rocks.fmt(f)
    }
}

impl<const M: usize, const N: usize> FromStr for Platform<M, N> {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let rocks = Matrix::from_str_map(s, Rock::from_char);
        Ok(Self { rocks })
    }
}
