pub fn part_1(input: &str) -> u32 {
    fn parse_calibration_value(input: &str) -> u32 {
        let mut digits = input.chars().filter_map(|c| c.to_digit(10)).peekable();
        10 * digits.peek().unwrap() + digits.last().unwrap()
    }
    input.lines().map(parse_calibration_value).sum()
}

pub fn part_2(input: &str) -> u32 {
    fn parse_calibration_value(input: &str) -> u32 {
        10 * get_first_digit(input) + get_last_digit(input)
    }
    input.lines().map(parse_calibration_value).sum()
}

fn get_first_digit(mut input: &str) -> u32 {
    loop {
        if let Some(digit) = input.chars().next().and_then(|c| c.to_digit(10)) {
            return digit;
        }
        for digit in 1..=9 {
            if input.starts_with(DIGITS[digit - 1]) {
                return u32::try_from(digit).unwrap();
            }
        }
        input = &input[1..];
    }
}

fn get_last_digit(mut input: &str) -> u32 {
    loop {
        if let Some(digit) = input.chars().last().and_then(|c| c.to_digit(10)) {
            return digit;
        }
        for digit in 1..=9 {
            if input.ends_with(DIGITS[digit - 1]) {
                return u32::try_from(digit).unwrap();
            }
        }
        input = &input[..(input.len() - 1)];
    }
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("example_1.txt");
    const EXAMPLE_2: &str = include_str!("example_2.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn example_1() {
        assert_eq!(142, part_1(EXAMPLE_1));
    }

    #[test]
    fn answer_1() {
        assert_eq!(54390, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(281, part_2(EXAMPLE_2));
    }

    #[test]
    fn answer_2() {
        assert_eq!(54277, part_2(INPUT));
    }
}
