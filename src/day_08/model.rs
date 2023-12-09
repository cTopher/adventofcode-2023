use std::collections::HashMap;
use std::iter::Cycle;
use std::slice::Iter;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Map {
    instructions: Vec<Instruction>,
    network: HashMap<String, Node>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Node {
    label: String,
    left: String,
    right: String,
}

pub struct NodeIterator<'a> {
    instructions: Cycle<Iter<'a, Instruction>>,
    network: &'a HashMap<String, Node>,
    current: &'a str,
}

impl Map {
    pub fn get<'a>(&'a self, node: &'a str) -> NodeIterator<'a> {
        NodeIterator::new(self, node)
    }

    pub fn ghosts(&self) -> impl Iterator<Item = NodeIterator> {
        self.network
            .keys()
            .filter(|node| node.ends_with('A'))
            .map(|key| self.get(key))
    }
}

impl Instruction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

impl<'a> NodeIterator<'a> {
    fn new(map: &'a Map, start: &'a str) -> Self {
        Self {
            instructions: map.instructions.iter().cycle(),
            network: &map.network,
            current: start,
        }
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions.next()?;
        let node = &self.network[self.current];
        self.current = match instruction {
            Instruction::Left => &node.left,
            Instruction::Right => &node.right,
        };
        Some(self.current)
    }
}

impl FromStr for Map {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let mut lines = s.lines();
        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(Instruction::from_char)
            .collect();
        let network = lines
            .skip(1)
            .map(|line| Node::from_str(line).unwrap())
            .map(|node| (node.label.clone(), node))
            .collect();
        Ok(Self {
            instructions,
            network,
        })
    }
}

impl FromStr for Node {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (label, node) = s.split_once(" = ").unwrap();
        let (left, right) = node
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        Ok(Self {
            label: label.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}
