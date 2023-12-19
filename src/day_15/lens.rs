use std::str::FromStr;

#[derive(Debug)]
pub struct InitializationSequence {
    steps: Vec<String>,
}

#[derive(Debug)]
pub struct Facility {
    boxes: Vec<Box>,
}

#[derive(Debug, Default, Clone)]
struct Box {
    lenses: Vec<Lens>,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl InitializationSequence {
    pub fn hash_sum(&self) -> usize {
        self.steps.iter().map(|step| hash(step)).sum()
    }
}

impl Facility {
    pub fn new() -> Self {
        Self {
            boxes: vec![Box::default(); 256],
        }
    }

    pub fn perform_initialization(&mut self, sequence: &InitializationSequence) {
        for step in &sequence.steps {
            if let Some(label) = step.strip_suffix('-') {
                self.box_mut(label).remove(label);
            } else {
                let (label, focal_length) = step.split_once('=').unwrap();
                self.box_mut(label).insert(Lens {
                    label: label.to_string(),
                    focal_length: focal_length.parse().unwrap(),
                });
            }
        }
    }

    pub fn focussing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_number, box_)| {
                (box_number + 1)
                    * box_
                        .lenses
                        .iter()
                        .enumerate()
                        .map(|(lens_index, lens)| {
                            let slot = lens_index + 1;
                            slot * lens.focal_length
                        })
                        .sum::<usize>()
            })
            .sum()
    }

    fn box_mut(&mut self, label: &str) -> &mut Box {
        &mut self.boxes[hash(label)]
    }
}

impl Box {
    fn remove(&mut self, label: &str) {
        if let Some(index) = self.lenses.iter().position(|lens| lens.label == label) {
            self.lenses.remove(index);
        }
    }

    fn insert(&mut self, lens: Lens) {
        if let Some(index) = self.lenses.iter().position(|l| l.label == lens.label) {
            self.lenses[index] = lens;
        } else {
            self.lenses.push(lens);
        }
    }
}

fn hash(string: &str) -> usize {
    string
        .chars()
        .fold(0, |result, c| ((result + c as usize) * 17) % 256)
}

impl FromStr for InitializationSequence {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let steps = s.trim_end().split(',').map(str::to_string).collect();
        Ok(Self { steps })
    }
}
