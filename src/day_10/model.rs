use itertools::Itertools;
use std::str::FromStr;

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Clone, Debug)]
pub struct Sketch {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Ground,
    Pipe { directions: [Direction; 2] },
    Start,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct PipeIterator<'a> {
    sketch: &'a Sketch,
    position: (usize, usize),
    direction: Direction,
}

impl Sketch {
    fn get(&self, position: (usize, usize)) -> Option<&Tile> {
        let (row, column) = position;
        self.tiles.get(row).and_then(|row| row.get(column))
    }

    pub fn max_distance_from_start(&self) -> u32 {
        let start = self.start_position();
        let (mut a, mut b) = DIRECTIONS
            .iter()
            .filter_map(|&direction| {
                if let Tile::Pipe { directions } = self.get(direction.apply(start)?)?
                    && directions.contains(&direction.opposite())
                {
                    Some(PipeIterator {
                        sketch: self,
                        position: start,
                        direction,
                    })
                } else {
                    None
                }
            })
            .collect_tuple()
            .unwrap();
        let mut distance = 0;
        loop {
            if a.next().unwrap() == b.position {
                return distance;
            }
            distance += 1;
            if b.next().unwrap() == a.position {
                return distance;
            }
        }
    }

    fn start_position(&self) -> (usize, usize) {
        self.tiles
            .iter()
            .enumerate()
            .find_map(|(row, tiles)| {
                tiles.iter().enumerate().find_map(|(col, tile)| {
                    if tile == &Tile::Start {
                        Some((row, col))
                    } else {
                        None
                    }
                })
            })
            .unwrap()
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::Pipe {
                directions: [Direction::North, Direction::South],
            },
            '-' => Self::Pipe {
                directions: [Direction::East, Direction::West],
            },
            'L' => Self::Pipe {
                directions: [Direction::North, Direction::East],
            },
            'J' => Self::Pipe {
                directions: [Direction::North, Direction::West],
            },
            '7' => Self::Pipe {
                directions: [Direction::South, Direction::West],
            },
            'F' => Self::Pipe {
                directions: [Direction::South, Direction::East],
            },
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Invalid character"),
        }
    }
}

impl Direction {
    fn apply(self, position: (usize, usize)) -> Option<(usize, usize)> {
        let (row, column) = position;
        Some(match self {
            Self::North => (row.checked_sub(1)?, column),
            Self::East => (row, column + 1),
            Self::South => (row + 1, column),
            Self::West => (row, column.checked_sub(1)?),
        })
    }

    const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

impl Iterator for PipeIterator<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.position = self.direction.apply(self.position)?;
        self.direction = match self.sketch.get(self.position)? {
            Tile::Pipe { directions } => directions
                .iter()
                .copied()
                .find(|&direction| direction != self.direction.opposite())?,
            _ => panic!("Invalid pipe"),
        };
        Some(self.position)
    }
}

impl FromStr for Sketch {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let tiles = s
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        Ok(Self { tiles })
    }
}
