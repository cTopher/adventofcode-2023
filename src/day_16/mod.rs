use std::str::FromStr;

use model::{Beam, Grid};

use crate::util::{Direction, Position};

mod model;

#[must_use]
pub fn part_1<const N: usize>(input: &str) -> u32 {
    let Ok(grid) = Grid::<N>::from_str(input);
    Beam::new(&grid, Position::default(), Direction::Right).energy()
}

#[must_use]
pub fn part_2<const N: usize>(input: &str) -> u32 {
    let Ok(grid) = Grid::<N>::from_str(input);
    (0..N)
        .flat_map(|i| {
            vec![
                Beam::new(&grid, Position::new(0, i), Direction::Down),
                Beam::new(&grid, Position::new(N - 1, i), Direction::Up),
                Beam::new(&grid, Position::new(i, 0), Direction::Right),
                Beam::new(&grid, Position::new(i, N - 1), Direction::Left),
            ]
        })
        .map(Beam::energy)
        .max()
        .unwrap()
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
        assert_eq!(46, part_1::<10>(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(7472, part_1::<110>(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(51, part_2::<10>(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(0, part_2::<110>(INPUT));
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| black_box(part_1::<110>(black_box(INPUT))));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| black_box(part_2::<110>(black_box(INPUT))));
    }
}
