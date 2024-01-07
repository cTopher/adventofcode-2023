use model::Scratchcard;

mod model;

fn parse_scratchcards(input: &str) -> impl Iterator<Item = Scratchcard> + '_ {
    input.lines().map(|line| {
        let Ok(scratchcard) = line.parse();
        scratchcard
    })
}

#[must_use]
pub fn part_1(input: &str) -> u32 {
    parse_scratchcards(input)
        .map(|scratchcard| scratchcard.points())
        .sum()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let scratchcards: Vec<_> = parse_scratchcards(input).collect();
    let mut amounts: Vec<usize> = scratchcards.iter().map(|_| 1).collect();
    for (index, scratchcard) in scratchcards.iter().enumerate() {
        let amount = amounts[index];
        amounts
            .iter_mut()
            .skip(index + 1)
            .take(scratchcard.number_of_winning_numbers())
            .for_each(|n| *n += amount);
    }
    amounts.into_iter().sum()
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
        assert_eq!(13, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(28750, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(30, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(10_212_704, part_2(INPUT));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| black_box(part_2(black_box(INPUT))));
    }
}
