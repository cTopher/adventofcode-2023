use std::rc::Rc;
use std::str::FromStr;

pub struct Schematic {
    values: Vec<Vec<Value>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Value {
    Number(Rc<u32>),
    Symbol { is_gear: bool },
    Period,
}

// I hate everything about all this code.
impl Schematic {
    pub fn part_numbers(&self) -> impl Iterator<Item = u32> + '_ {
        let mut result = Vec::new();
        self.values.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, value)| {
                if matches!(value, Value::Symbol { .. }) {
                    for i in i.saturating_sub(1)..=(i + 1) {
                        for j in j.saturating_sub(1)..=(j + 1) {
                            if let Some(Value::Number(n)) =
                                self.values.get(i).and_then(|row| row.get(j))
                            {
                                result.push(n);
                            }
                        }
                    }
                }
            });
        });
        result.dedup_by(|a, b| Rc::ptr_eq(a, b));
        result.into_iter().map(|n| **n)
    }

    pub fn total_gear_ratios(&self) -> u32 {
        let mut result = 0;
        self.values.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, value)| {
                if matches!(value, Value::Symbol { is_gear: true }) {
                    let mut numbers = Vec::new();
                    for i in i.saturating_sub(1)..=(i + 1) {
                        for j in j.saturating_sub(1)..=(j + 1) {
                            if let Some(Value::Number(n)) =
                                self.values.get(i).and_then(|row| row.get(j))
                            {
                                numbers.push(n);
                            }
                        }
                    }
                    numbers.dedup_by(|a, b| Rc::ptr_eq(a, b));
                    if numbers.len() == 2 {
                        result += **numbers[0] * **numbers[1];
                    }
                }
            });
        });
        result
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
                        if c == '.' {
                            values.push(Value::Period);
                        } else {
                            values.push(Value::Symbol { is_gear: c == '*' });
                        }
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
