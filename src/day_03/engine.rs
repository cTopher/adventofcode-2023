use itertools::Itertools;
use std::rc::Rc;
use std::str::FromStr;

pub struct Schematic {
    values: Vec<Vec<Value>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Value {
    Number(Rc<u32>),
    Gear,
    OtherSymbol,
    Period,
}

// I hate everything about all this code.
impl Schematic {
    pub fn part_numbers(&self) -> impl Iterator<Item = u32> + '_ {
        self.values
            .iter()
            .enumerate()
            .flat_map(move |(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, value)| value.is_symbol())
                    .flat_map(move |(j, _)| self.adjacent_numbers(i, j))
            })
            .dedup_by(Rc::ptr_eq)
            .map(|n| *n)
    }

    pub fn gear_ratios(&self) -> impl Iterator<Item = u32> + '_ {
        self.values.iter().enumerate().flat_map(move |(i, row)| {
            row.iter()
                .enumerate()
                .filter(move |&(_, value)| value == &Value::Gear)
                .filter_map(move |(j, _)| {
                    let numbers: Vec<_> = self.adjacent_numbers(i, j).collect();
                    if numbers.len() == 2 {
                        Some(*numbers[0] * *numbers[1])
                    } else {
                        None
                    }
                })
        })
    }

    fn adjacent_numbers(&self, i: usize, j: usize) -> impl Iterator<Item = Rc<u32>> + '_ {
        (i.saturating_sub(1)..=(i + 1))
            .flat_map(move |i| {
                let row = self.values.get(i);
                (j.saturating_sub(1)..=(j + 1)).filter_map(move |j| row.and_then(|row| row.get(j)))
            })
            .filter_map(Value::number)
            .dedup_by(Rc::ptr_eq)
    }
}

impl Value {
    fn number(&self) -> Option<Rc<u32>> {
        if let Self::Number(n) = self {
            Some(n.clone())
        } else {
            None
        }
    }

    fn is_symbol(&self) -> bool {
        self == &Self::Gear || self == &Self::OtherSymbol
    }
}

impl FromStr for Schematic {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let values = s
            .lines()
            .map(|line| {
                let mut values = Vec::new();
                let mut number = 0;
                let mut digit = 0;
                for c in line.chars() {
                    if let Some(n) = c.to_digit(10) {
                        number = 10 * number + n;
                        digit += 1;
                    } else {
                        let value = Rc::new(number);
                        for _ in 0..digit {
                            values.push(Value::Number(value.clone()));
                        }
                        number = 0;
                        digit = 0;
                        values.push(match c {
                            '.' => Value::Period,
                            '*' => Value::Gear,
                            _ => Value::OtherSymbol,
                        });
                    }
                }
                let value = Rc::new(number);
                for _ in 0..digit {
                    values.push(Value::Number(value.clone()));
                }
                values
            })
            .collect();
        Ok(Self { values })
    }
}
