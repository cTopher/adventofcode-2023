use std::iter::zip;
use std::str::FromStr;

pub struct Image {
    pixels: Vec<Vec<Pixel>>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Pixel {
    EmptySpace,
    Galaxy,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Galaxy {
    x: u64,
    y: u64,
}

impl Image {
    pub fn galaxy_distances(&self, expansion: u64) -> u64 {
        self.galaxies(expansion)
            .iter()
            .enumerate()
            .map(|(i, galaxy)| {
                self.galaxies(expansion)
                    .iter()
                    .skip(i + 1)
                    .map(|other_galaxy| galaxy.distance(other_galaxy))
                    .sum::<u64>()
            })
            .sum()
    }

    fn galaxies(&self, expansion: u64) -> Vec<Galaxy> {
        let expended_cols: Vec<_> = (0..self.width())
            .map(|column| {
                self.pixels
                    .iter()
                    .all(|row| row[column] == Pixel::EmptySpace)
            })
            .collect();
        let mut galaxies = Vec::new();
        let mut x = 0;
        for row in &self.pixels {
            if row.iter().all(|&pixel| pixel == Pixel::EmptySpace) {
                x += expansion;
                continue;
            }
            let mut y = 0;
            for (&pixel, &expanded_column) in zip(row, &expended_cols) {
                if expanded_column {
                    y += expansion - 1;
                }
                if pixel == Pixel::Galaxy {
                    galaxies.push(Galaxy { x, y });
                }
                y += 1;
            }
            x += 1;
        }
        galaxies
    }

    fn width(&self) -> usize {
        self.pixels[0].len()
    }
}

impl Galaxy {
    const fn distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::EmptySpace,
            '#' => Self::Galaxy,
            _ => panic!("Invalid pixel: {c}"),
        }
    }
}

impl FromStr for Image {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let pixels = s
            .lines()
            .map(|line| line.chars().map(Pixel::from).collect())
            .collect();
        Ok(Self { pixels })
    }
}
