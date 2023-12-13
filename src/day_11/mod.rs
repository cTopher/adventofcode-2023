mod model;

use model::Image;
use std::str::FromStr;

#[must_use]
pub fn part_1(input: &str) -> u64 {
    let Ok(image) = Image::from_str(input);
    image.galaxy_distances(2)
}

#[must_use]
pub fn part_2(input: &str) -> u64 {
    let Ok(image) = Image::from_str(input);
    image.galaxy_distances(1_000_000)
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
        assert_eq!(374, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(9_329_143, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        let Ok(image) = Image::from_str(EXAMPLE);
        assert_eq!(1030, image.galaxy_distances(10));
        assert_eq!(8410, image.galaxy_distances(100));
    }

    #[test]
    fn answer_2() {
        assert_eq!(710_674_907_809, part_2(INPUT));
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| black_box(part_1(black_box(INPUT))));
    }

    #[bench]
    // 51192 ns/iter (+/- 729)
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| black_box(part_2(black_box(INPUT))));
    }
}
