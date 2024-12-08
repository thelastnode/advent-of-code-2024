use std::io::{stdin, Read};

use anyhow::Result;
use day05::{parse, Graph, Input};

fn process(input: Input) -> i64 {
    let graph = Graph::from(input.rules.iter());
    input
        .updates
        .iter()
        .filter(|&update| graph.is_valid_topo(update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let (_, input) = parse(&input).map_err(|e| e.to_owned())?;

    println!("{}", process(input));

    Ok(())
}
