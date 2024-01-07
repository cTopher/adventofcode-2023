use std::str::FromStr;
mod city;

fn solve<const M: usize, const N: usize>(input: &str, min_move: usize, max_move: usize) -> u32 {
    let Ok(map) = city::Map::<M, N>::from_str(input);
    map.least_heat_loss(min_move, max_move)
}

#[must_use]
pub fn part_1<const M: usize, const N: usize>(input: &str) -> u32 {
    solve::<M, N>(input, 1, 3)
}

#[must_use]
pub fn part_2<const M: usize, const N: usize>(input: &str) -> u32 {
    solve::<M, N>(input, 4, 10)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::{black_box, Bencher};

    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const MINI_EXAMPLE: &str = include_str!("mini_example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn example_1() {
        assert_eq!(102, part_1::<13, 13>(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(771, part_1::<141, 141>(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(94, part_2::<13, 13>(EXAMPLE));
    }

    #[test]
    fn mini_example() {
        assert_eq!(71, part_2::<5, 12>(MINI_EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(930, part_2::<141, 141>(INPUT));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| black_box(part_2::<141, 141>(black_box(INPUT))));
    }
}
