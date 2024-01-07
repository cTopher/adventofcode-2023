use std::str::FromStr;

mod mirror;

fn sum_notes(input: &str, smudged: bool) -> usize {
    input
        .split("\n\n")
        .map(|p| {
            let Ok(valley) = mirror::Valley::from_str(p);
            valley.reflection_note(smudged)
        })
        .sum()
}

#[must_use]
pub fn part_1(input: &str) -> usize {
    sum_notes(input, false)
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    sum_notes(input, true)
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
        assert_eq!(405, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(29213, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(400, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(37453, part_2(INPUT));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| black_box(part_2(black_box(INPUT))));
    }
}
