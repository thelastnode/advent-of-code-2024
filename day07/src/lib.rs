use std::io::BufRead;

use itertools::Itertools;

#[derive(Debug)]
pub enum Op {
    Add,
    Multiply,
    Concatenate,
}

impl Op {
    pub fn apply(&self, left: &i64, right: &i64) -> i64 {
        match self {
            Op::Add => left + right,
            Op::Multiply => left * right,
            Op::Concatenate => format!("{left}{right}").parse().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Equation {
    pub result: i64,
    pub parts: Vec<i64>,
}

impl Equation {
    pub fn is_valid(&self, ops: &[Op]) -> bool {
        (0..(self.parts.len() - 1))
            .map(|_| ops.iter())
            .multi_cartesian_product()
            .any(|ops| {
                let mut ops = ops.iter();
                let result = self
                    .parts
                    .iter()
                    .copied()
                    .reduce(|acc, part| {
                        let op: &&Op = ops.next().unwrap();
                        op.apply(&acc, &part)
                    })
                    .unwrap();
                result == self.result
            })
    }
}

pub fn read_input(input: impl BufRead) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (result, parts) = line.split_once(": ").unwrap();
            Equation {
                result: result.parse().unwrap(),
                parts: parts.split(" ").map(|part| part.parse().unwrap()).collect(),
            }
        })
        .collect()
}
