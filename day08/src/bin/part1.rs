use anyhow::Result;
use day08::{count_antinodes, parse, Location};
use std::{
    io::{stdin, Read},
    iter,
};

fn calculate_mirror(
    base: &(usize, usize),
    other: &(usize, usize),
) -> impl Iterator<Item = (isize, isize)> {
    let &(base_row, base_col) = base;
    let &(other_row, other_col) = other;

    let row_diff = base_row as isize - other_row as isize;
    let col_diff = base_col as isize - other_col as isize;

    iter::once((
        (base_row as isize + row_diff),
        (base_col as isize + col_diff),
    ))
}

fn process(input: Vec<Vec<Location>>) -> usize {
    count_antinodes(&input, calculate_mirror)
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let input = parse(&input);

    println!("{}", process(input));

    Ok(())
}
