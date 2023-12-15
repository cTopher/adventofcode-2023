use std::str::FromStr;

mod spring;

fn solve(input: &str, unfold: bool) -> u64 {
    let mut solver = spring::Solver::new();
    let rows: Vec<spring::Row> = input
        .lines()
        .map(|l| {
            let Ok(mut row) = spring::Row::from_str(l);
            if unfold {
                row = row.unfold();
            }
            row.simplify();
            row
        })
        .collect();
    rows.iter()
        .map(|row| solver.arrangements(row.as_slice()))
        .sum()
}

#[must_use]
pub fn part_1(input: &str) -> u64 {
    solve(input, false)
}

#[must_use]
pub fn part_2(input: &str) -> u64 {
    solve(input, true)
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
        assert_eq!(21, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(7506, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(525_152, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(548241300348335, part_2(INPUT));
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