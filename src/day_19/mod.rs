use std::str::FromStr;

pub(crate) use part::*;
pub(crate) use workflow::*;

mod part;
mod workflow;

#[must_use]
pub fn part_1(input: &str) -> u64 {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let Ok(workflows) = Workflows::from_str(workflows);
    parts
        .lines()
        .map(|line| Part::from_str(line).unwrap())
        .filter(|&part| workflows.accepts(part))
        .map(|part| part.rating())
        .sum()
}

#[must_use]
pub fn part_2(input: &str) -> u64 {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let Ok(workflows) = Workflows::from_str(workflows);
    workflows.possible_accepted_parts()
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
        assert_eq!(19114, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(397_134, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(167_409_079_868_000, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(127_517_902_575_337, part_2(INPUT));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| black_box(part_2(black_box(INPUT))));
    }
}
