use std::str::FromStr;

mod spring;

fn solve(input: &str, unfold: bool) -> u64 {
    input
        .lines()
        .map(|l| {
            let Ok(mut row) = spring::Row::from_str(l);
            if unfold {
                row = row.unfold(5);
            }
            let mut solver = spring::Solver::new(&mut row);
            solver.arrangements()
        })
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
        assert_eq!(548_241_300_348_335, part_2(INPUT));
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| black_box(part_1(black_box(INPUT))));
    }

    #[bench]
    // 7 ms/iter (+/- 0)
    // if unfold=15 35 ms/iter (+/- 1)
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| black_box(part_2(black_box(INPUT))));
    }
}
