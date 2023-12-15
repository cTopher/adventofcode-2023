use itertools::Itertools;
use std::assert_matches::assert_matches;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use std::{fmt, iter};

#[derive(Debug, Clone)]
pub struct Row {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowSlice<'a> {
    springs: &'a [Spring],
    groups: &'a [usize],
}

pub struct Solver<'a> {
    cache: HashMap<RowSlice<'a>, u64>,
}

impl Row {
    pub fn simplify(&mut self) {
        while self.springs.last().copied() == Some(Spring::Operational) {
            self.springs.pop();
        }
        self.springs = self
            .springs
            .drain(..)
            .skip_while(|s| s.is_operational())
            .coalesce(|a, b| {
                if a.is_operational() && b.is_operational() {
                    Ok(a)
                } else {
                    Err((a, b))
                }
            })
            .collect();
    }

    pub fn unfold(self) -> Self {
        let spring_len = 5 * self.springs.len() + 4;
        let group_len = 5 * self.groups.len();
        let springs = self
            .springs
            .into_iter()
            .chain(iter::once(Spring::Unknown))
            .cycle()
            .take(spring_len)
            .collect();
        let groups = self.groups.into_iter().cycle().take(group_len).collect();
        Self { springs, groups }
    }

    pub fn as_slice(&self) -> RowSlice<'_> {
        RowSlice {
            springs: &self.springs,
            groups: &self.groups,
        }
        .simplify()
        .unwrap()
    }
}

impl<'a> Solver<'a> {
    pub fn new() -> Self {
        let mut cache = HashMap::new();
        cache.insert(RowSlice::EMPTY, 1);
        Self { cache }
    }

    pub fn arrangements(&mut self, row: RowSlice<'a>) -> u64 {
        if let Some(&arrangements) = self.cache.get(&row) {
            return arrangements;
        }
        let arrangements = self.calc_arrangements(row);
        self.cache.insert(row, arrangements);
        arrangements
    }

    fn calc_arrangements(&mut self, row: RowSlice<'a>) -> u64 {
        [row.with_operational(), row.with_damaged()]
            .into_iter()
            .flatten()
            .map(|row| self.arrangements(row))
            .sum()
    }
}

impl RowSlice<'static> {
    const EMPTY: Self = Self {
        springs: &[],
        groups: &[],
    };
}

impl<'a> RowSlice<'a> {
    fn simplify(mut self) -> Option<Self> {
        loop {
            if self.springs.first().copied() == Some(Spring::Operational) {
                self.springs = &self.springs[1..];
            }
            if self.springs.first().copied() == Some(Spring::Damaged) {
                self = self.take_next_group()?;
            } else {
                break;
            }
        }
        if self.springs.is_empty() && !self.groups.is_empty() {
            return None;
        }
        assert_matches!(self.springs.first(), None | Some(Spring::Unknown));
        Some(self)
    }

    fn take_next_group(mut self) -> Option<Self> {
        let group = *self.groups.first()?;
        self.groups = &self.groups[1..];
        for _ in 0..group {
            if matches!(self.springs.first(), None | Some(Spring::Operational)) {
                return None;
            }
            self.springs = &self.springs[1..];
        }
        match self.springs.first() {
            Some(Spring::Operational | Spring::Unknown) => {
                self.springs = &self.springs[1..];
                Some(self)
            }
            Some(Spring::Damaged) => None,
            None if !self.groups.is_empty() => None,
            None => Some(self),
        }
    }

    fn with_damaged(self) -> Option<Self> {
        assert_eq!(self.springs.first(), Some(&Spring::Unknown));
        self.take_next_group()?.simplify()
    }

    fn with_operational(mut self) -> Option<Self> {
        assert_eq!(self.springs.first(), Some(&Spring::Unknown));
        self.springs = &self.springs[1..];
        self.simplify()
    }
}

impl fmt::Display for RowSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(spring) = self.springs.first() {
            fmt::Display::fmt(spring, f)?;
        }
        for spring in self.springs.iter().skip(1) {
            fmt::Display::fmt(spring, f)?;
        }
        if let Some(group) = self.groups.first() {
            fmt::Display::fmt(&' ', f)?;
            fmt::Display::fmt(group, f)?;
        }
        for group in self.groups.iter().skip(1) {
            fmt::Display::fmt(&',', f)?;
            fmt::Display::fmt(group, f)?;
        }
        Ok(())
    }
}

impl Spring {
    fn is_operational(self) -> bool {
        self == Self::Operational
    }
}

impl fmt::Display for Spring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&char::from(*self), f)
    }
}

impl From<Spring> for char {
    fn from(s: Spring) -> Self {
        match s {
            Spring::Operational => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        }
    }
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Invalid spring character: {c}"),
        }
    }
}

impl FromStr for Row {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (springs, groups) = s.split_once(' ').unwrap();
        let springs = springs.chars().map(Spring::from).collect();
        let groups = groups.split(',').map(|s| s.parse().unwrap()).collect();
        Ok(Self { springs, groups })
    }
}
