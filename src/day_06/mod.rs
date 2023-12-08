use std::str::FromStr;

mod boat;

#[must_use]
pub fn part_1(input: &str) -> u64 {
    boat::parse_races(input)
        .map(|race| race.number_of_ways_to_beat())
        .product()
}

#[must_use]
pub fn part_2(input: &str) -> u64 {
    let Ok(race) = boat::Race::from_str(input);
    race.number_of_ways_to_beat()
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
        assert_eq!(288, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(293_046, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(71503, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(35_150_181, part_2(INPUT));
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
