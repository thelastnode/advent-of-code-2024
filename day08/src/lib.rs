use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub enum Location {
    Empty,
    Node(char),
}

pub fn parse(input: &str) -> Vec<Vec<Location>> {
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

pub fn group_nodes_by_type(input: &[Vec<Location>]) -> HashMap<char, Vec<(usize, usize)>> {
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

pub fn count_antinodes<F, T>(input: &[Vec<Location>], mut calculate_mirrors: F) -> usize
where
    F: FnMut(&(usize, usize), &(usize, usize)) -> T,
    T: Iterator<Item = (isize, isize)>,
{
    let nodes_by_type = group_nodes_by_type(input);
    let mut mirrors = HashSet::new();

    for (_, nodes) in nodes_by_type.iter() {
        for (first, second) in nodes.iter().cartesian_product(nodes.iter()) {
            if first == second {
                continue;
            }
            mirrors.extend(calculate_mirrors(first, second));
        }
    }

    mirrors
        .into_iter()
        .filter(|&(row, col)| {
            row >= 0 && col >= 0 && row < (input.len() as isize) && col < (input[0].len() as isize)
        })
        .count()
}
