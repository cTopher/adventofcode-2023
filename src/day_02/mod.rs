use game::{CubeSet, Game};

mod game;

fn parse_games(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().map(|line| {
        let Ok(game) = line.parse();
        game
    })
}

#[must_use]
pub fn part_1(input: &str) -> u32 {
    let bag = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    parse_games(input)
        .filter(|game| game.sets.iter().all(|set| bag.contains(set)))
        .map(|game| game.id)
        .sum()
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    parse_games(input)
        .map(|game| game.minimum_set().power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn example_1() {
        assert_eq!(8, part_1(EXAMPLE));
    }

    #[test]
    fn answer_1() {
        assert_eq!(2512, part_1(INPUT));
    }

    #[test]
    fn example_2() {
        assert_eq!(2286, part_2(EXAMPLE));
    }

    #[test]
    fn answer_2() {
        assert_eq!(67335, part_2(INPUT));
    }
}
