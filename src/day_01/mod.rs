pub fn part_1(input: &str) -> u32 {
    fn parse_calibration_value(input: &str) -> u32 {
        let mut digits = input.chars().filter_map(|c| c.to_digit(10)).peekable();
        10 * digits.peek().unwrap() + digits.last().unwrap()
    }
    input.lines().map(parse_calibration_value).sum()
}

pub fn part_2(input: &str) -> u32 {
    fn parse_calibration_value(input: &str) -> u32 {
        10 * first_digit(input) + last_digit(input)
    }
    input.lines().map(parse_calibration_value).sum()
}

fn first_digit(mut input: &str) -> u32 {
    loop {
        if let Some(digit) = input.chars().next().and_then(|c| c.to_digit(10)) {
            return digit;
        }
        if let Some(digit) = find_spelled_digit(|spelled| input.starts_with(spelled)) {
            return digit;
        }
        input = &input[1..];
    }
}

fn last_digit(mut input: &str) -> u32 {
    loop {
        if let Some(digit) = input.chars().last().and_then(|c| c.to_digit(10)) {
            return digit;
        }
        if let Some(digit) = find_spelled_digit(|spelled| input.ends_with(spelled)) {
            return digit;
        }
        input = &input[..(input.len() - 1)];
    }
}

fn find_spelled_digit(predicate: impl Fn(&str) -> bool) -> Option<u32> {
    SPELLED_DIGITS
        .iter()
        .position(|spelled| predicate(spelled))
        .map(|pos| u32::try_from(pos + 1).unwrap())
}

const SPELLED_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[cfg(test)]
mod tests {
    extern crate test;

    use test::{black_box, Bencher};

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

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| black_box(part_1(black_box(INPUT))));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| black_box(part_2(black_box(INPUT))));
    }
}
