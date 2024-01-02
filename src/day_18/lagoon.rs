use crate::util::{Direction, Vector2D};
use std::str::FromStr;

pub struct DigPlan {
    pub steps: Vec<DigStep>,
}

pub struct DigStep {
    pub direction: Direction,
    pub distance: u64,
}

impl DigPlan {
    pub fn volume(&self) -> u64 {
        let vertices = self.vertices();
        let area = vertices
            .windows(2)
            .map(|w| w[0].cross_product(w[1]))
            .sum::<i64>()
            .unsigned_abs()
            / 2;
        let boundary = self.steps.iter().map(|step| step.distance).sum::<u64>();
        area + boundary / 2 + 1
    }

    fn vertices(&self) -> Vec<Vector2D> {
        let mut vertices = Vec::with_capacity(self.steps.len() + 1);
        let mut position = Vector2D::ZERO;
        vertices.push(position);
        for step in &self.steps {
            position += Vector2D::from(step.direction) * i64::try_from(step.distance).unwrap();
            vertices.push(position);
        }
        vertices
    }
}

impl FromStr for DigPlan {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<_>>();
        Ok(Self { steps })
    }
}

impl FromStr for DigStep {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let direction = split.next().unwrap().parse().unwrap();
        let distance = split.next().unwrap().parse().unwrap();
        Ok(Self {
            direction,
            distance,
        })
    }
}
