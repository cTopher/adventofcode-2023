use std::collections::HashMap;
use std::str::FromStr;

use super::{Category, Part, PartRange};

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
    value: u64,
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
pub struct Workflows {
    list: HashMap<String, Workflow>,
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
    const fn evaluate(self, part: Part) -> bool {
        let part_value = part.get(self.category);
        match self.operator {
            Operator::MoreThan => part_value > self.value,
            Operator::LessThan => part_value < self.value,
        }
    }

    fn evaluate_range(self, parts: PartRange) -> (Option<PartRange>, Option<PartRange>) {
        match self.operator {
            Operator::MoreThan => {
                let val = parts.get(self.category);
                if val.min > self.value {
                    return (Some(parts), None);
                }
                if val.max <= self.value {
                    return (None, Some(parts));
                }
                let mut a = parts;
                let mut b = parts;
                a.get_mut(self.category).min = self.value + 1;
                b.get_mut(self.category).max = self.value;
                (Some(a), Some(b))
            }
            Operator::LessThan => {
                let val = parts.get(self.category);
                if val.max < self.value {
                    return (Some(parts), None);
                }
                if val.min >= self.value {
                    return (None, Some(parts));
                }
                let mut a = parts;
                let mut b = parts;
                a.get_mut(self.category).max = self.value - 1;
                b.get_mut(self.category).min = self.value;
                (Some(a), Some(b))
            }
        }
    }
}

impl Workflows {
    pub fn accepts(&self, part: Part) -> bool {
        let mut workflow = "in";
        loop {
            match self.list.get(workflow).unwrap().evaluate(part) {
                RuleResult::Workflow(next_workflow) => workflow = next_workflow.as_str(),
                RuleResult::Accept => return true,
                RuleResult::Reject => return false,
            }
        }
    }

    pub fn possible_accepted_parts(&self) -> u64 {
        let mut accepted = 0;
        let mut todo = vec![(PartRange::new(1, 4000), "in")];
        while let Some((mut range, workflow)) = todo.pop() {
            let workflow = self.list.get(workflow).unwrap();
            for rule in &workflow.rules {
                let (a, b) = rule.condition.map_or((Some(range), None), |condition| {
                    condition.evaluate_range(range)
                });
                if let Some(a) = a {
                    match &rule.result {
                        RuleResult::Accept => accepted += a.size(),
                        RuleResult::Reject => {}
                        RuleResult::Workflow(name) => todo.push((a, &name)),
                    }
                }
                if let Some(b) = b {
                    range = b;
                } else {
                    break;
                }
            }
        }
        accepted
    }
}

// PARSING //

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '>' => Self::MoreThan,
            '<' => Self::LessThan,
            _ => panic!("Unknown operator: {c}"),
        }
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

impl FromStr for Workflows {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let list = s
            .lines()
            .map(|line| line.parse().unwrap())
            .map(|workflow: Workflow| (workflow.name.clone(), workflow))
            .collect();

        Ok(Self { list })
    }
}
