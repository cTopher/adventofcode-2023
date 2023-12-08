use itertools::Itertools;
use std::cmp::Reverse;

pub type Bid = u32;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Hand {
    type_: Type,
    cards: [Card; 5],
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
enum Card {
    Joker,
    Number(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        Self {
            type_: Self::type_for_cards(cards),
            cards,
        }
    }

    fn type_for_cards(cards: [Card; 5]) -> Type {
        let mut counts = cards.iter().counts();
        let jokers = counts.remove(&Card::Joker).unwrap_or(0);
        let mut counts: Vec<_> = counts.into_values().collect();
        counts.sort_unstable_by_key(|count| Reverse(*count));
        match (
            counts.first().copied().unwrap_or(0) + jokers,
            counts.get(1).copied().unwrap_or(0),
        ) {
            (5, _) => Type::FiveOfAKind,
            (4, _) => Type::FourOfAKind,
            (3, 2) => Type::FullHouse,
            (3, _) => Type::ThreeOfAKind,
            (2, 2) => Type::TwoPair,
            (2, _) => Type::OnePair,
            _ => Type::HighCard,
        }
    }

    pub fn from_str(s: &str, has_jokers: bool) -> Self {
        let cards = s
            .chars()
            .map(|c| Card::from_char(c, has_jokers))
            .collect_vec()
            .try_into()
            .unwrap();
        Self::new(cards)
    }
}

impl Card {
    fn from_char(c: char, has_jokers: bool) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' if has_jokers => Self::Joker,
            'J' => Self::J,
            'T' => Self::T,
            _ => Self::Number(u8::try_from(c.to_digit(10).unwrap()).unwrap()),
        }
    }
}
