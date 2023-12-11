mod oasis;

fn parse_input(input: &str) -> impl Iterator<Item = oasis::History> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

#[must_use]
pub fn part_1(input: &str) -> i32 {
    parse_input(input).map(oasis::History::next).sum()
}

#[must_use]
pub fn part_2(input: &str) -> i32 {
    parse_input(input).map(oasis::History::previous).sum()
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
        assert_eq!(114, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(1_995_001_648, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(988, part_2(INPUT));
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
