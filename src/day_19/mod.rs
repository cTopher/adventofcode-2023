use model::Input;
use std::str::FromStr;

mod model;

#[must_use]
pub fn part_1(input: &str) -> u32 {
    let Ok(input) = Input::from_str(input);
    input.total_rating()
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
        assert_eq!(19114, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(397_134, part_1(INPUT));
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
    fn bench(b: &mut Bencher) {
        b.iter(|| black_box(part_2(black_box(INPUT))));
    }
}
