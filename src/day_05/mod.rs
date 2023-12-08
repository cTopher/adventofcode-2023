use std::str::FromStr;

use model::{Almanac, Range};

mod model;

#[must_use]
pub fn part_1(input: &str) -> u64 {
    let Ok(almanac) = Almanac::from_str(input);
    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.seed_to_location(seed))
        .min()
        .unwrap()
}

#[must_use]
pub fn part_2(input: &str) -> u64 {
    let Ok(almanac) = Almanac::from_str(input);
    almanac
        .seeds
        .chunks(2)
        .map(|chunk| Range::new(chunk[0], chunk[0] + chunk[1]))
        .map(|seeds| almanac.closest_location_for_seeds(seeds))
        .min()
        .unwrap()
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
        assert_eq!(35, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(662_197_086, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(46, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(52_510_809, part_2(INPUT));
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
