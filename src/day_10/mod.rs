mod model;
mod nest;

use model::Sketch;
use std::str::FromStr;

#[must_use]
pub fn part_1(input: &str) -> usize {
    let Ok(sketch) = Sketch::from_str(input);
    sketch.pipe().count().div_ceil(2)
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let Ok(sketch) = Sketch::from_str(input);
    let finder = nest::Finder::new(sketch.width(), sketch.height(), sketch.pipe());
    finder.inside_area()
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
        assert_eq!(8, part_1(EXAMPLE_1));
    }

    #[test]
    fn answer_1() {
        assert_eq!(6909, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(10, part_2(EXAMPLE_2));
    }

    #[test]
    fn answer_2() {
        assert_eq!(461, part_2(INPUT));
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| black_box(part_1(black_box(INPUT))));
    }

    #[bench]
    // 679902 ns/iter (+/- 40530)
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| black_box(part_2(black_box(INPUT))));
    }
}
