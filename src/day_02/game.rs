use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<CubeSet>,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Game {
    pub fn minimum_set(&self) -> CubeSet {
        let mut result = CubeSet::default();
        for set in &self.sets {
            result.red = result.red.max(set.red);
            result.green = result.green.max(set.green);
            result.blue = result.blue.max(set.blue);
        }
        result
    }
}

impl CubeSet {
    pub const fn contains(&self, subset: &Self) -> bool {
        self.red >= subset.red && self.green >= subset.green && self.blue >= subset.blue
    }

    pub const fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for Game {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (id, sets) = s.split_once(':').unwrap();
        let id = id.strip_prefix("Game ").unwrap().parse().unwrap();
        let sets = sets
            .split(';')
            .map(CubeSet::from_str)
            .collect::<Result<_, !>>()?;
        Ok(Self { id, sets })
    }
}

impl FromStr for CubeSet {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let mut result = Self::default();
        for part in s.trim_start().split(", ") {
            let (amount, color) = part.split_once(' ').unwrap();
            let amount = amount.parse().unwrap();
            match color {
                "red" => result.red = amount,
                "green" => result.green = amount,
                "blue" => result.blue = amount,
                _ => panic!("Unknown color: {color}"),
            }
        }
        Ok(result)
    }
}
