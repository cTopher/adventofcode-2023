use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    seed_to_location_map: Map,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MapRange {
    source: Range,
    offset: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Almanac {
    pub fn seed_to_location(&self, seed: u64) -> u64 {
        self.seed_to_location_map.apply(seed)
    }

    pub fn closest_location_for_seeds(&self, seeds: Range) -> u64 {
        self.seed_to_location_map.find_min(seeds)
    }
}

// NOTE: a lot of these methods could be improved using binary search
impl Map {
    fn new(mut ranges: Vec<MapRange>) -> Self {
        ranges.sort();
        let mut index = 0;
        let mut position = 0;
        while index < ranges.len() {
            let range = ranges[index].source;
            if range.start > position {
                ranges.insert(
                    index,
                    MapRange {
                        source: Range::new(position, range.start),
                        offset: 0,
                    },
                );
                index += 1;
            }
            index += 1;
            position = range.end;
        }
        if position < u64::MAX {
            ranges.push(MapRange {
                source: Range::new(position, u64::MAX),
                offset: 0,
            });
        }
        Self { ranges }
    }

    fn apply(&self, source: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|range| range.apply(source))
            .unwrap()
    }

    fn find_min(&self, source: Range) -> u64 {
        self.ranges
            .iter()
            .filter_map(|range| range.find_min(source))
            .min()
            .unwrap()
    }

    fn chain(self, other: &Self) -> Self {
        let ranges = self
            .ranges
            .into_iter()
            .flat_map(|a| {
                let a_destination = a.destination();
                other.ranges.iter().filter_map(move |b| {
                    a_destination.overlap(b.source).map(|overlap| {
                        let source = overlap.transpose(-a.offset);
                        let offset = a.offset + b.offset;
                        MapRange { source, offset }
                    })
                })
            })
            .collect();
        Self { ranges }
    }
}

impl MapRange {
    fn apply(&self, source: u64) -> Option<u64> {
        if self.source.includes(source) {
            Some(source.checked_add_signed(self.offset).unwrap())
        } else {
            None
        }
    }

    fn find_min(&self, source: Range) -> Option<u64> {
        self.source
            .overlap(source)
            .map(|overlap| overlap.start.checked_add_signed(self.offset).unwrap())
    }

    fn destination(&self) -> Range {
        self.source.transpose(self.offset)
    }
}

impl Range {
    pub const fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn overlap(self, other: Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        if start < end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    const fn includes(self, value: u64) -> bool {
        self.start <= value && value < self.end
    }

    fn transpose(self, offset: i64) -> Self {
        Self {
            start: self.start.checked_add_signed(offset).unwrap(),
            end: self.end.checked_add_signed(offset).unwrap(),
        }
    }
}

impl FromStr for Almanac {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let mut parts = s.split("\n\n");
        let seeds = parts
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect();
        let seed_to_location_map = parts
            .map(|s| Map::from_str(s).unwrap())
            .reduce(|a, b| a.chain(&b))
            .unwrap();
        Ok(Self {
            seeds,
            seed_to_location_map,
        })
    }
}

impl FromStr for Map {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let ranges: Vec<MapRange> = s
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect();
        Ok(Self::new(ranges))
    }
}

impl FromStr for MapRange {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (destination_start, source_start, length) = s
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let source = Range::new(source_start, source_start + length);
        let offset =
            i64::try_from(destination_start).unwrap() - i64::try_from(source_start).unwrap();
        Ok(Self { source, offset })
    }
}
