use std::io::{stdin, Read};

use anyhow::Result;
use day06::{parse_input, Cell, Input, Position};
use rustc_hash::FxHashSet;

fn process(input: Input) -> usize {
    let (start_pos, start_dir) = input
        .map
        .iter()
        .find_map(|(pos, cell)| match cell {
            Cell::Guard(dir) => Some((pos, dir)),
            _ => None,
        })
        .expect("Could not find guard");
    let mut candidate_positions = FxHashSet::<Position>::with_capacity_and_hasher(
        (input.rows * input.cols) as usize,
        Default::default(),
    );
    input.traverse(start_pos, start_dir, |pos, _| {
        candidate_positions.insert(pos.clone());
        false
    });
    candidate_positions.remove(start_pos);

    candidate_positions
        .into_iter()
        .filter(|pos| input.with_obstacle(pos).does_loop(start_pos, start_dir))
        .count()
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    println!("{:?}", process(parse_input(&input)));

    Ok(())
}
