use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct Rule(pub i64, pub i64);

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    pub rules: Vec<Rule>,
    pub updates: Vec<Vec<i64>>,
}

pub struct Graph {
    dependencies: HashMap<i64, HashSet<i64>>,
    dependents: HashMap<i64, HashSet<i64>>,
}

impl<'a, I> From<I> for Graph
where
    I: Iterator<Item = &'a Rule>,
{
    fn from(rules: I) -> Self {
        let mut dependencies: HashMap<i64, HashSet<i64>> = HashMap::new();
        let mut dependents: HashMap<i64, HashSet<i64>> = HashMap::new();

        for rule in rules {
            dependents.entry(rule.0).or_default().insert(rule.1);
            dependencies.entry(rule.1).or_default().insert(rule.0);
        }

        Graph {
            dependencies,
            dependents,
        }
    }
}

impl Graph {
    pub fn is_valid_topo(&self, ordering: &[i64]) -> bool {
        let mut unvisited = ordering.iter().copied().collect::<HashSet<i64>>();
        for &n in ordering.iter() {
            if let Some(deps) = self.dependencies.get(&n) {
                if deps.iter().any(|in_node| unvisited.contains(in_node)) {
                    return false;
                }
            }
            unvisited.remove(&n);
        }
        true
    }

    pub fn get_topo(&self, input: &[i64]) -> Vec<i64> {
        let mut unvisited: HashSet<i64> = input.iter().copied().collect::<HashSet<i64>>();
        let mut dependencies: HashMap<i64, HashSet<i64>> = self
            .dependencies
            .iter()
            .map(|(&node, deps)| {
                (
                    node,
                    deps.iter()
                        .copied()
                        .filter(|dep| unvisited.contains(dep))
                        .collect(),
                )
            })
            .collect();

        let mut result = Vec::new();

        while !unvisited.is_empty() {
            let &next = unvisited
                .iter()
                .find(|&node| dependencies.get(node).map_or(true, |deps| deps.is_empty()))
                .expect("Could not find a node with no dependencies");
            unvisited.remove(&next);
            result.push(next);
            if let Some(dependents) = self.dependents.get(&next) {
                for &dependent in dependents {
                    dependencies.entry(dependent).and_modify(|deps| {
                        deps.remove(&next);
                    });
                }
            }
        }

        result
    }
}

pub fn parse(input: &str) -> IResult<&str, Input> {
    let rule = separated_pair(complete::i64, tag("|"), complete::i64).map(|(a, b)| Rule(a, b));
    let rules = separated_list1(newline, rule);

    let update = separated_list1(tag(","), complete::i64);
    let updates = separated_list1(newline, update);

    let mut parser = separated_pair(rules, count(newline, 2), updates)
        .map(|(rules, updates)| Input { rules, updates });

    parser.parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "1|2\n3|4\n\n1,2\n3,4";
        let expected = Input {
            rules: vec![Rule(1, 2), Rule(3, 4)],
            updates: vec![vec![1, 2], vec![3, 4]],
        };

        assert_eq!(parse(input), Ok(("", expected)));
    }
}
