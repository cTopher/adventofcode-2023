use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    ExtremeLyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    result: RuleResult,
}

#[derive(Debug, Copy, Clone)]
struct Condition {
    category: Category,
    operator: Operator,
    value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    MoreThan,
    LessThan,
}

#[derive(Debug)]
enum RuleResult {
    Workflow(String),
    Accept,
    Reject,
}

#[derive(Debug)]
pub struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Part {
    const fn rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    const fn get(&self, category: Category) -> u32 {
        match category {
            Category::ExtremeLyCoolLooking => self.x,
            Category::Musical => self.m,
            Category::Aerodynamic => self.a,
            Category::Shiny => self.s,
        }
    }
}

impl Workflow {
    fn evaluate(&self, part: Part) -> &RuleResult {
        &self
            .rules
            .iter()
            .find(|rule| rule.evaluate(part))
            .unwrap()
            .result
    }
}

impl Rule {
    fn evaluate(&self, part: Part) -> bool {
        self.condition
            .map_or(true, |condition| condition.evaluate(part))
    }
}

impl Condition {
    fn evaluate(self, part: Part) -> bool {
        let part_value = part.get(self.category);
        match self.operator {
            Operator::MoreThan => part_value > self.value,
            Operator::LessThan => part_value < self.value,
        }
    }
}

impl Input {
    fn evaluate(&self, part: Part) -> bool {
        let mut workflow = "in";
        loop {
            match self.workflows.get(workflow).unwrap().evaluate(part) {
                RuleResult::Workflow(next_workflow) => workflow = next_workflow.as_str(),
                RuleResult::Accept => return true,
                RuleResult::Reject => return false,
            }
        }
    }

    pub fn total_rating(&self) -> u32 {
        self.parts
            .iter()
            .filter(|&&part| self.evaluate(part))
            .map(Part::rating)
            .sum()
    }
}

// PARSING //

impl From<char> for Category {
    fn from(c: char) -> Self {
        match c {
            'x' => Self::ExtremeLyCoolLooking,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => panic!("Unknown category: {c}"),
        }
    }
}

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '>' => Self::MoreThan,
            '<' => Self::LessThan,
            _ => panic!("Unknown operator: {c}"),
        }
    }
}

impl FromStr for Part {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, m, a, s) = s
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|c| {
                let (_, val) = c.split_once('=').unwrap();
                u32::from_str(val).unwrap()
            })
            .collect_tuple()
            .unwrap();
        Ok(Self { x, m, a, s })
    }
}

impl FromStr for Workflow {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (name, rules) = s.strip_suffix('}').unwrap().split_once('{').unwrap();
        let name = name.to_string();
        let rules = rules.split(',').map(|rule| rule.parse().unwrap()).collect();
        Ok(Self { name, rules })
    }
}

impl FromStr for Rule {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        if let Some((condition, result)) = s.split_once(':') {
            let condition = Some(condition.parse()?);
            let result = result.parse()?;
            Ok(Self { condition, result })
        } else {
            let condition = None;
            let result = s.parse()?;
            Ok(Self { condition, result })
        }
    }
}

impl FromStr for Condition {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let mut chars = s.chars();
        Ok(Self {
            category: chars.next().unwrap().into(),
            operator: chars.next().unwrap().into(),
            value: chars.as_str().parse().unwrap(),
        })
    }
}

impl FromStr for RuleResult {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            _ => Ok(Self::Workflow(s.to_string())),
        }
    }
}

impl FromStr for Input {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (workflows, parts) = s.split_once("\n\n").unwrap();
        let workflows = workflows
            .lines()
            .map(|line| line.parse().unwrap())
            .map(|workflow: Workflow| (workflow.name.clone(), workflow))
            .collect();
        let parts = parts.lines().map(|line| line.parse().unwrap()).collect();
        Ok(Self { workflows, parts })
    }
}
