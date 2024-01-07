use model::{Bid, Hand};

mod model;

fn winnings(input: &str, has_jokers: bool) -> u32 {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| parse_line(line, has_jokers))
        .collect();
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

fn parse_line(line: &str, has_jokers: bool) -> (Hand, Bid) {
    let (hand, bid) = line.split_once(' ').unwrap();
    (Hand::from_str(hand, has_jokers), bid.parse().unwrap())
}

#[must_use]
pub fn part_1(input: &str) -> u32 {
    winnings(input, false)
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    winnings(input, true)
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
        assert_eq!(248_559_379, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(5905, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(249_631_254, part_2(INPUT));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| black_box(part_2(black_box(INPUT))));
    }
}
