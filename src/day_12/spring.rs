use itertools::Itertools;
use std::assert_matches::debug_assert_matches;
use std::fmt::{Debug, Write};
use std::str::FromStr;
use std::{fmt, iter};

#[derive(Debug, Clone)]
pub struct Row {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
struct RowSlice<'a> {
    row: &'a Row,
    spring_offset: usize,
    group_offset: usize,
}

pub struct Solver<'a> {
    row: &'a Row,
    cache: Vec<Option<u64>>,
}

impl Row {
    fn simplify(&mut self) {
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

    pub fn unfold(self, times: usize) -> Self {
        let spring_len = times * self.springs.len() + times - 1;
        let group_len = times * self.groups.len();
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

    fn as_slice(&self) -> RowSlice<'_> {
        RowSlice {
            row: self,
            spring_offset: 0,
            group_offset: 0,
        }
        .simplify()
        .unwrap()
    }
}

impl<'a> Solver<'a> {
    pub fn new(row: &'a mut Row) -> Self {
        row.simplify();
        let size = (row.springs.len() + 1) * (row.groups.len() + 1);
        let mut cache = vec![None; size];
        cache[size - 1] = Some(1);
        Self { row, cache }
    }

    pub fn arrangements(&mut self) -> u64 {
        self.arrangements_for_slice(self.row.as_slice())
    }

    fn arrangements_for_slice(&mut self, row: RowSlice) -> u64 {
        if let Some(arrangements) = self.cache[row.index()] {
            return arrangements;
        }
        let arrangements = self.calc_arrangements(row);
        self.cache[row.index()] = Some(arrangements);
        arrangements
    }

    fn calc_arrangements(&mut self, row: RowSlice) -> u64 {
        [row.with_operational(), row.with_damaged()]
            .into_iter()
            .flatten()
            .map(|row| self.arrangements_for_slice(row))
            .sum()
    }
}

impl<'a> RowSlice<'a> {
    fn index(&self) -> usize {
        self.spring_offset * (self.row.groups.len() + 1) + self.group_offset
    }

    fn simplify(mut self) -> Option<Self> {
        loop {
            if self.first_spring() == Some(Spring::Operational) {
                self.spring_offset += 1;
            }
            if self.first_spring() == Some(Spring::Damaged) {
                self = self.take_next_group()?;
            } else {
                break;
            }
        }
        if self.springs_is_empty() && !self.groups_is_empty() {
            return None;
        }
        debug_assert_matches!(self.first_spring(), None | Some(Spring::Unknown));
        Some(self)
    }

    fn take_next_group(mut self) -> Option<Self> {
        let group = self.first_group()?;
        self.group_offset += 1;
        for _ in 0..group {
            if matches!(self.first_spring(), None | Some(Spring::Operational)) {
                return None;
            }
            self.spring_offset += 1;
        }
        match self.first_spring() {
            Some(Spring::Operational | Spring::Unknown) => {
                self.spring_offset += 1;
                Some(self)
            }
            Some(Spring::Damaged) => None,
            None if !self.groups_is_empty() => None,
            None => Some(self),
        }
    }

    fn with_damaged(self) -> Option<Self> {
        debug_assert_eq!(self.first_spring(), Some(Spring::Unknown));
        self.take_next_group()?.simplify()
    }

    fn with_operational(mut self) -> Option<Self> {
        debug_assert_eq!(self.first_spring(), Some(Spring::Unknown));
        self.spring_offset += 1;
        self.simplify()
    }

    fn first_spring(&self) -> Option<Spring> {
        self.row.springs.get(self.spring_offset).copied()
    }

    fn first_group(&self) -> Option<usize> {
        self.row.groups.get(self.group_offset).copied()
    }

    fn springs_is_empty(&self) -> bool {
        debug_assert!(self.spring_offset <= self.row.springs.len());
        self.spring_offset >= self.row.springs.len()
    }

    fn groups_is_empty(&self) -> bool {
        debug_assert!(self.group_offset <= self.row.groups.len());
        self.group_offset >= self.row.groups.len()
    }
}

impl fmt::Display for RowSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(spring) = self.first_spring() {
            fmt::Display::fmt(&spring, f)?;
        }
        for spring in self.row.springs.iter().skip(self.spring_offset + 1) {
            fmt::Display::fmt(spring, f)?;
        }
        if let Some(group) = self.first_group() {
            write!(f, " {group}")?;
        }
        for group in self.row.groups.iter().skip(self.group_offset + 1) {
            write!(f, ",{group}")?;
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
        f.write_char((*self).into())
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
