use std::iter::zip;
use std::str::FromStr;

#[derive(Debug)]
pub struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    pub fn number_of_ways_to_beat(&self) -> u64 {
        let sd = f64::sqrt((self.time * self.time - 4 * self.record_distance) as f64);
        let time = self.time as f64;
        let a = (time + sd) / 2.0;
        let b = (time - sd) / 2.0;
        (a.ceil() - b.floor()) as u64 - 1
    }
}

pub fn parse_races(input: &str) -> impl Iterator<Item = Race> + '_ {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace();
    let distance = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace();
    zip(time, distance).map(|(time, distance)| Race {
        time: time.parse().unwrap(),
        record_distance: distance.parse().unwrap(),
    })
}

impl FromStr for Race {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let s = s.replace(' ', "");
        let mut lines = s.lines();
        let time = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .parse()
            .unwrap();
        let record_distance = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .parse()
            .unwrap();
        Ok(Self {
            time,
            record_distance,
        })
    }
}
