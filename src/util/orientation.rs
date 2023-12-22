use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub struct OrientationSet {
    pub horizontal: bool,
    pub vertical: bool,
}

impl OrientationSet {
    pub fn insert(&mut self, orientation: Orientation) -> bool {
        let val = match orientation {
            Orientation::Horizontal => &mut self.horizontal,
            Orientation::Vertical => &mut self.vertical,
        };
        !mem::replace(val, true)
    }
}
