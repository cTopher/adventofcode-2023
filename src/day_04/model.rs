use std::str::FromStr;

pub struct Scratchcard {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Scratchcard {
    pub fn points(&self) -> u32 {
        match self.number_of_winning_numbers() {
            0 => 0,
            amount => 2u32.pow(u32::try_from(amount).unwrap() - 1),
        }
    }

    pub fn number_of_winning_numbers(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }
}

impl FromStr for Scratchcard {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (_id, numbers) = s.split_once(':').unwrap();
        let (winning_numbers, numbers) = numbers.split_once('|').unwrap();
        Ok(Self {
            winning_numbers: parse_numbers(winning_numbers),
            numbers: parse_numbers(numbers),
        })
    }
}

fn parse_numbers(s: &str) -> Vec<u32> {
    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}
