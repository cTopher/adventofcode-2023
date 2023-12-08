use itertools::Itertools;
use std::str::FromStr;

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
        let counts: Vec<_> = cards.iter().counts().values().copied().collect();
        let pairs = counts.iter().filter(|&&count| count == 2).count();
        if counts.contains(&5) {
            Type::FiveOfAKind
        } else if counts.contains(&4) {
            Type::FourOfAKind
        } else if pairs == 1 && counts.contains(&3) {
            Type::FullHouse
        } else if counts.contains(&3) {
            Type::ThreeOfAKind
        } else if pairs == 2 {
            Type::TwoPair
        } else if pairs == 1 {
            Type::OnePair
        } else {
            Type::HighCard
        }
    }
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            _ => Self::Number(u8::try_from(c.to_digit(10).unwrap()).unwrap()),
        }
    }
}

impl FromStr for Hand {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let cards = s
            .chars()
            .map(Card::from_char)
            .collect_vec()
            .try_into()
            .unwrap();
        Ok(Self::new(cards))
    }
}
