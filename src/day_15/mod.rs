use std::str::FromStr;

mod lens;

#[must_use]
pub fn part_1(input: &str) -> usize {
    let Ok(sequence) = lens::InitializationSequence::from_str(input);
    sequence.hash_sum()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let Ok(sequence) = lens::InitializationSequence::from_str(input);
    let mut facility = lens::Facility::new();
    facility.perform_initialization(&sequence);
    facility.focussing_power()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::{black_box, Bencher};

    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn example_1() {
        assert_eq!(1320, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(514_281, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(145, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(244_199, part_2(INPUT));
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
