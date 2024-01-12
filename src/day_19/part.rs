use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
pub struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

#[derive(Debug, Copy, Clone)]
pub struct Range {
    pub min: u64,
    pub max: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    ExtremeLyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Part {
    pub const fn rating(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }

    pub const fn get(&self, category: Category) -> u64 {
        match category {
            Category::ExtremeLyCoolLooking => self.x,
            Category::Musical => self.m,
            Category::Aerodynamic => self.a,
            Category::Shiny => self.s,
        }
    }
}

impl PartRange {
    pub const fn new(min: u64, max: u64) -> Self {
        let range = Range { min, max };
        Self {
            x: range,
            m: range,
            a: range,
            s: range,
        }
    }

    pub const fn get(&self, category: Category) -> &Range {
        match category {
            Category::ExtremeLyCoolLooking => &self.x,
            Category::Musical => &self.m,
            Category::Aerodynamic => &self.a,
            Category::Shiny => &self.s,
        }
    }

    pub fn get_mut(&mut self, category: Category) -> &mut Range {
        match category {
            Category::ExtremeLyCoolLooking => &mut self.x,
            Category::Musical => &mut self.m,
            Category::Aerodynamic => &mut self.a,
            Category::Shiny => &mut self.s,
        }
    }

    pub const fn size(&self) -> u64 {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }
}

impl Range {
    pub const fn size(&self) -> u64 {
        self.max - self.min + 1
    }
}

impl From<char> for Category {
    fn from(c: char) -> Self {
        match c {
            'x' => Self::ExtremeLyCoolLooking,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => panic!("Unknown category: {c}"),
        }
    }
}

impl FromStr for Part {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, m, a, s) = s
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|c| {
                let (_, val) = c.split_once('=').unwrap();
                u64::from_str(val).unwrap()
            })
            .collect_tuple()
            .unwrap();
        Ok(Self { x, m, a, s })
    }
}
