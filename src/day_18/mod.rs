use crate::util::Direction;
use std::str::FromStr;

mod lagoon;

#[must_use]
pub fn part_1(input: &str) -> u64 {
    let Ok(dig_plan) = lagoon::DigPlan::from_str(input);
    dig_plan.volume()
}

#[must_use]
pub fn part_2(input: &str) -> u64 {
    let steps = input
        .lines()
        .map(|line| parse_with_color_swap(line))
        .collect();
    let dig_plan = lagoon::DigPlan { steps };
    dig_plan.volume()
}

fn parse_with_color_swap(line: &str) -> lagoon::DigStep {
    let s = line
        .split(' ')
        .nth(2)
        .unwrap()
        .strip_prefix("(#")
        .unwrap()
        .strip_suffix(')')
        .unwrap();
    let distance = u64::from_str_radix(&s[0..(s.len() - 1)], 16).unwrap();
    let direction = match s.chars().last().unwrap() {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => panic!("Invalid direction"),
    };
    lagoon::DigStep {
        direction,
        distance,
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::{black_box, Bencher};

    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn example_1() {
        assert_eq!(62, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(67891, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(952_408_144_115, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(94_116_351_948_493, part_2(INPUT));
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
