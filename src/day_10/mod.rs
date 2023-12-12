mod model;

use model::Sketch;
use std::str::FromStr;

#[must_use]
pub fn part_1(input: &str) -> usize {
    let Ok(sketch) = Sketch::from_str(input);
    sketch.pipe().count().div_ceil(2)
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    0
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
        assert_eq!(8, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(6909, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(0, part_2(INPUT));
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
