use std::iter::zip;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Valley {
    pattern: Vec<Vec<Ground>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Ground {
    Ash,
    Rocks,
}

impl Valley {
    #[allow(clippy::option_if_let_else)]
    pub fn reflection_note(&self, smudged: bool) -> usize {
        if let Some(columns) = self.vertical_reflection_line(smudged) {
            columns
        } else if let Some(rows) = self.horizontal_reflection_line(smudged) {
            100 * rows
        } else {
            panic!("No reflection line found");
        }
    }

    fn vertical_reflection_line(&self, smudged: bool) -> Option<usize> {
        (1..self.width()).find(|&columns| self.is_vertical_reflection_line(columns, smudged))
    }

    fn horizontal_reflection_line(&self, smudged: bool) -> Option<usize> {
        (1..self.height()).find(|&rows| self.is_horizontal_reflection_line(rows, smudged))
    }

    fn is_vertical_reflection_line(&self, columns: usize, smudged: bool) -> bool {
        let mut has_inconsistency = false;
        for row in &self.pattern {
            for (a, b) in zip(row.iter().take(columns).rev(), row.iter().skip(columns)) {
                if a != b {
                    if !smudged || has_inconsistency {
                        return false;
                    }
                    has_inconsistency = true;
                }
            }
        }
        has_inconsistency == smudged
    }

    fn is_horizontal_reflection_line(&self, rows: usize, smudged: bool) -> bool {
        let mut has_inconsistency = false;
        let top = self.pattern.iter().take(rows).rev();
        let bottom = self.pattern.iter().skip(rows);
        for (ra, rb) in zip(top, bottom) {
            for (a, b) in zip(ra, rb) {
                if a != b {
                    if !smudged || has_inconsistency {
                        return false;
                    }
                    has_inconsistency = true;
                }
            }
        }
        has_inconsistency == smudged
    }

    fn width(&self) -> usize {
        self.pattern[0].len()
    }

    fn height(&self) -> usize {
        self.pattern.len()
    }
}

impl From<char> for Ground {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rocks,
            _ => panic!("Invalid ground type {c}"),
        }
    }
}

impl FromStr for Valley {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let pattern = s
            .lines()
            .map(|line| line.chars().map(Ground::from).collect())
            .collect();
        Ok(Self { pattern })
    }
}
