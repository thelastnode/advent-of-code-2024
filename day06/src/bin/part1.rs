use std::{
    collections::HashSet,
    io::{stdin, Read},
};

use anyhow::Result;
use day06::{parse_input, Cell, Input, Position};

fn process(input: Input) -> i64 {
    let (start_pos, start_dir) = input
        .map
        .iter()
        .find_map(|(pos, cell)| match cell {
            Cell::Guard(dir) => Some((pos, dir)),
            _ => None,
        })
        .expect("Could not find guard");
    let mut visited = HashSet::<Position>::new();

    input.traverse(start_pos, start_dir, |pos, _| {
        visited.insert(pos.clone());
        false
    });

    visited.len() as i64
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    println!("{:?}", process(parse_input(&input)));

    Ok(())
}
