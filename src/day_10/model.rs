use std::str::FromStr;

pub const DIRECTIONS: [Direction; 4] = [
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
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct PipeIterator<'a> {
    sketch: &'a Sketch,
    position: (usize, usize),
    direction: Direction,
    started: bool,
}

impl Sketch {
    fn get(&self, position: (usize, usize)) -> Option<&Tile> {
        let (row, column) = position;
        self.tiles.get(row).and_then(|row| row.get(column))
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn pipe(&self) -> PipeIterator<'_> {
        let start = self.start_position();
        let direction = DIRECTIONS
            .iter()
            .find_map(|&direction| {
                if let Tile::Pipe { directions } = self.get(direction.apply(start)?)?
                    && directions.contains(&direction.opposite())
                {
                    Some(direction)
                } else {
                    None
                }
            })
            .unwrap();
        PipeIterator {
            sketch: self,
            position: start,
            direction,
            started: false,
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
    pub fn apply(self, position: (usize, usize)) -> Option<(usize, usize)> {
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

impl PipeIterator<'_> {
    pub const fn direction(&self) -> Direction {
        self.direction
    }
}

impl Iterator for PipeIterator<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.position);
        }
        self.position = self.direction.apply(self.position)?;
        match self.sketch.get(self.position)? {
            Tile::Pipe { directions } => {
                self.direction = directions
                    .iter()
                    .copied()
                    .find(|&direction| direction != self.direction.opposite())?;
                Some(self.position)
            }
            _ => None,
        }
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
