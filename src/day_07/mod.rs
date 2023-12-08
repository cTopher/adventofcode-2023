use model::{Bid, Hand};

mod model;

fn parse_input(input: &str) -> Vec<(Hand, Bid)> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (hand.parse().unwrap(), bid.parse().unwrap())
        })
        .collect()
}

#[must_use]
pub fn part_1(input: &str) -> u32 {
    let mut hands = parse_input(input);
    hands.sort_by_key(|(hand, _)| *hand);
    hands
        .into_iter()
        .enumerate()
        .map(|(index, (_, bid))| {
            let rank = u32::try_from(index).unwrap() + 1;
            bid * rank
        })
        .sum()
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
        assert_eq!(6440, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(0, part_1(INPUT));
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
