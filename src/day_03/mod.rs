use std::str::FromStr;

mod engine;

#[must_use]
pub fn part_1(input: &str) -> u32 {
    let Ok(schematic) = engine::Schematic::from_str(input);
    schematic.part_numbers().sum()
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    let Ok(schematic) = engine::Schematic::from_str(input);
    schematic.gear_ratios().sum()
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
        assert_eq!(4361, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(527_369, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(467_835, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(73_074_886, part_2(INPUT));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| black_box(part_2(black_box(INPUT))));
    }
}
