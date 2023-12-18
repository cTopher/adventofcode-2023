use std::str::FromStr;

mod dish;

const CYCLES: usize = 1_000_000_000;

#[must_use]
pub fn part_1(input: &str) -> usize {
    let Ok(mut platform) = dish::Platform::from_str(input);
    platform.tilt_north();
    platform.north_support_beam_load()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let Ok(mut platform) = dish::Platform::from_str(input);
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
        assert_eq!(136, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(110_565, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(64, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(89845, part_2(INPUT));
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| black_box(part_1(black_box(INPUT))));
    }

    #[bench]
    // 22 ms/iter (+/- 1)
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| black_box(part_2(black_box(INPUT))));
    }
}
