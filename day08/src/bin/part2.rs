use std::{
    collections::{HashMap, HashSet},
    io::{stdin, Read},
};

use anyhow::Result;
use itertools::Itertools;

enum Location {
    Empty,
    Node(char),
}

fn parse(input: &str) -> Vec<Vec<Location>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Location::Empty,
                    _ => Location::Node(c),
                })
                .collect()
        })
        .collect()
}

fn group_nodes_by_type(input: &[Vec<Location>]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut nodes = HashMap::new();

    for (row, row_locations) in input.iter().enumerate() {
        for (col, location) in row_locations.iter().enumerate() {
            if let Location::Node(c) = location {
                nodes.entry(*c).or_insert_with(Vec::new).push((row, col));
            }
        }
    }

    nodes
}

fn calculate_mirror(
    (rows, cols): &(usize, usize),
    base: &(usize, usize),
    other: &(usize, usize),
) -> Vec<(isize, isize)> {
    let &(base_row, base_col) = base;
    let &(other_row, other_col) = other;

    let row_diff = base_row as isize - other_row as isize;
    let col_diff = base_col as isize - other_col as isize;

    (0..)
        .map_while(|n| {
            let (r, c) = (
                (base_row as isize + n * row_diff),
                (base_col as isize + n * col_diff),
            );
            if r >= 0 && c >= 0 && r < (*rows as isize) && c < (*cols as isize) {
                Some((r, c))
            } else {
                None
            }
        })
        .collect()
}

fn process(input: Vec<Vec<Location>>) -> usize {
    let nodes_by_type = group_nodes_by_type(&input);

    let mut mirrors = HashSet::new();
    let dims = (input.len(), input[0].len());

    for (_, nodes) in nodes_by_type.iter() {
        for (first, second) in nodes.iter().cartesian_product(nodes.iter()) {
            if first == second {
                continue;
            }
            mirrors.extend(calculate_mirror(&dims, first, second));
            mirrors.extend(calculate_mirror(&dims, second, first));
        }
    }

    mirrors
        .into_iter()
        .filter(|&(row, col)| {
            row >= 0 && col >= 0 && row < (input.len() as isize) && col < (input[0].len() as isize)
        })
        .count()
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let input = parse(&input);

    println!("{}", process(input));

    Ok(())
}
