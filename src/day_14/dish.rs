use std::fmt;
use std::fmt::{Formatter, Write};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Platform {
    rocks: Vec<Vec<Option<Rock>>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rock {
    Rounded,
    CubeShaped,
}

//noinspection DuplicatedCode
impl Platform {
    pub fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn tilt_north(&mut self) {
        let (m, n) = self.size();
        for i in 1..m {
            for j in 0..n {
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
        let (m, n) = self.size();
        for i in 0..m {
            for j in 1..n {
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
        let (m, n) = self.size();
        for i in (0..m - 1).rev() {
            for j in 0..n {
                if self.rocks[i][j] == Some(Rock::Rounded) {
                    if let Some(k) = (i + 1..m)
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
        let (m, n) = self.size();
        for i in 0..m {
            for j in (0..n - 1).rev() {
                if self.rocks[i][j] == Some(Rock::Rounded) {
                    if let Some(k) = (j + 1..n)
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
            .iter()
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

    fn size(&self) -> (usize, usize) {
        (self.rocks.len(), self.rocks[0].len())
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

impl FromStr for Platform {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let rocks = s
            .lines()
            .map(|line| line.chars().map(Rock::from_char).collect())
            .collect();
        Ok(Self { rocks })
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.rocks {
            for rock in row {
                let char = match rock {
                    Some(Rock::Rounded) => 'O',
                    Some(Rock::CubeShaped) => '#',
                    None => '.',
                };
                f.write_char(char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
