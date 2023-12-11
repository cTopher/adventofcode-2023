use std::str::FromStr;

pub struct History {
    values: Vec<i32>,
}

impl History {
    pub fn next(mut self) -> i32 {
        let mut next = 0;
        while self.values.iter().rev().any(|&v| v != 0) {
            for index in 0..(self.values.len() - 1) {
                self.values[index] = self.values[index + 1] - self.values[index];
            }
            next += self.values.pop().unwrap();
        }
        next
    }

    pub fn previous(mut self) -> i32 {
        let mut previous = self.values[0];
        let mut modifier = -1;
        while self.values.iter().rev().any(|&v| v != 0) {
            for index in 0..(self.values.len() - 1) {
                self.values[index] = self.values[index + 1] - self.values[index];
            }
            self.values.pop();
            previous += self.values[0] * modifier;
            modifier *= -1;
        }
        previous
    }
}

impl FromStr for History {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(' ').map(|s| s.parse().unwrap()).collect();
        Ok(Self { values })
    }
}
