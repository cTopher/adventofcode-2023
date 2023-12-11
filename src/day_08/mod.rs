use std::str::FromStr;

use itertools::Itertools;

use model::Map;

mod math;
mod model;

#[must_use]
pub fn part_1(input: &str) -> usize {
    let Ok(map) = Map::from_str(input);
    map.get("AAA")
        .take_while_inclusive(|&node| node != "ZZZ")
        .count()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let Ok(map) = Map::from_str(input);
    map.ghosts()
        .map(|ghost| {
            ghost
                .take_while_inclusive(|node| !node.ends_with('Z'))
                .count()
        })
        .reduce(math::lcm)
        .unwrap()
}

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
        assert_eq!(6, part_1(EXAMPLE_1));
    }

    #[test]
    fn answer_1() {
        assert_eq!(20513, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(6, part_2(EXAMPLE_2));
    }

    #[test]
    fn answer_2() {
        assert_eq!(15_995_167_053_923, part_2(INPUT));
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
