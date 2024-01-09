use std::str::FromStr;

mod dish;

const CYCLES: usize = 1_000_000_000;

#[must_use]
pub fn part_1<const M: usize, const N: usize>(input: &str) -> usize {
    let Ok(mut platform) = dish::Platform::<N, N>::from_str(input);
    platform.tilt_north();
    platform.north_support_beam_load()
}

#[must_use]
pub fn part_2<const M: usize, const N: usize>(input: &str) -> usize {
    let Ok(mut platform) = dish::Platform::<N, N>::from_str(input);
    let mut states = vec![platform.clone()];
    for cycle in 1..=CYCLES {
        platform.spin_cycle();
        if let Some(position) = states.iter().position(|state| state == &platform) {
            let repetition = &states[position..];
            platform = repetition[(CYCLES - cycle) % repetition.len()].clone();
            break;
        }
        states.push(platform.clone());
    }
    platform.north_support_beam_load()
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
        assert_eq!(136, part_1::<10, 10>(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(110_565, part_1::<100, 100>(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(64, part_2::<10, 10>(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(89845, part_2::<100, 100>(INPUT));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| black_box(part_2::<100, 100>(black_box(INPUT))));
    }
}
