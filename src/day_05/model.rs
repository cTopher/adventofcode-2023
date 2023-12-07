use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil_map: Map,
    soil_to_fertilizer_map: Map,
    fertilizer_to_water_map: Map,
    water_to_light_map: Map,
    light_to_temperature_map: Map,
    temperature_to_humidity_map: Map,
    humidity_to_location_map: Map,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

impl Almanac {
    pub fn seed_locations(&self) -> impl Iterator<Item = u32> + '_ {
        self.seeds.iter().map(|&seed| self.seed_to_location(seed))
    }

    fn seed_to_location(&self, seed: u32) -> u32 {
        let soil = self.seed_to_soil_map.get(seed);
        let fertilizer = self.soil_to_fertilizer_map.get(soil);
        let water = self.fertilizer_to_water_map.get(fertilizer);
        let light = self.water_to_light_map.get(water);
        let temperature = self.light_to_temperature_map.get(light);
        let humidity = self.temperature_to_humidity_map.get(temperature);
        self.humidity_to_location_map.get(humidity)
    }
}

impl Map {
    fn get(&self, source: u32) -> u32 {
        self.ranges
            .iter()
            .rfind(|range| range.source_start <= source)
            .and_then(|range| range.get(source))
            .unwrap_or(source)
    }
}

impl Range {
    fn get(&self, source: u32) -> Option<u32> {
        source
            .checked_sub(self.source_start)
            .filter(|&offset| offset < self.length)
            .map(|offset| self.destination_start + offset)
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
        let mut maps = parts.map(|part| part.parse().unwrap());
        let result = Self {
            seeds,
            seed_to_soil_map: maps.next().unwrap(),
            soil_to_fertilizer_map: maps.next().unwrap(),
            fertilizer_to_water_map: maps.next().unwrap(),
            water_to_light_map: maps.next().unwrap(),
            light_to_temperature_map: maps.next().unwrap(),
            temperature_to_humidity_map: maps.next().unwrap(),
            humidity_to_location_map: maps.next().unwrap(),
        };
        assert_eq!(0, maps.count());
        Ok(result)
    }
}

impl FromStr for Map {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let mut ranges: Vec<Range> = s
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect();
        ranges.sort_by_key(|range| range.source_start);
        Ok(Self { ranges })
    }
}

impl FromStr for Range {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (destination_start, source_start, length) = s
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self {
            destination_start,
            source_start,
            length,
        })
    }
}
