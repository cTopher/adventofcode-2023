use std::str::FromStr;

pub struct Image {
    pixels: Vec<Vec<Pixel>>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Pixel {
    EmptySpace,
    Galaxy,
}

impl Image {
    pub fn galaxy_distances(&self, expansion: u32) -> u64 {
        distance_sums(self.galaxy_x(expansion)) + distance_sums(self.galaxy_y(expansion))
    }

    fn galaxy_x(&self, expansion: u32) -> Vec<u32> {
        let mut result = Vec::new();
        let mut x = 0;
        for column in 0..self.width() {
            let amount = self
                .pixels
                .iter()
                .filter(|row| row[column] == Pixel::Galaxy)
                .count();
            if amount == 0 {
                x += expansion;
            } else {
                for _ in 0..amount {
                    result.push(x);
                }
                x += 1;
            }
        }
        result
    }

    fn galaxy_y(&self, expansion: u32) -> Vec<u32> {
        let mut result = Vec::new();
        let mut y = 0;
        for row in &self.pixels {
            let amount = row.iter().filter(|&&pixel| pixel == Pixel::Galaxy).count();
            if amount == 0 {
                y += expansion;
            } else {
                for _ in 0..amount {
                    result.push(y);
                }
                y += 1;
            }
        }
        result
    }

    fn width(&self) -> usize {
        self.pixels[0].len()
    }
}

fn distance_sums(positions: Vec<u32>) -> u64 {
    let z = i64::try_from(positions.len() - 1).unwrap();
    let sum: i64 = positions
        .into_iter()
        .enumerate()
        .map(|(i, x)| i64::from(x) * (2 * i64::try_from(i).unwrap() - z))
        .sum();
    u64::try_from(sum).unwrap()
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
