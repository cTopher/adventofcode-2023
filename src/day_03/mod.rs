use crate::day_03::engine::Schematic;
use std::str::FromStr;

mod engine;

#[must_use]
pub fn part_1(input: &str) -> u32 {
    let Ok(schematic) = Schematic::from_str(input);
    schematic.part_numbers().sum()
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    let Ok(schematic) = Schematic::from_str(input);
    schematic.total_gear_ratios()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn example_1() {
        assert_eq!(4361, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(527_369, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(467_835, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(73_074_886, part_2(INPUT));
    }
}
