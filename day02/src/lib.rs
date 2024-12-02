use anyhow::Result;
use itertools::Itertools;
use std::io::{stdin, BufRead};

pub fn read_input() -> Result<Vec<Vec<i32>>> {
    read_input_from(stdin().lock())
}

pub fn read_input_from(source: impl BufRead) -> Result<Vec<Vec<i32>>> {
    source
        .lines()
        .map(|line| {
            line?
                .split_whitespace()
                .map(|level| level.parse::<i32>().map_err(anyhow::Error::from))
                .collect::<Result<Vec<i32>>>()
        })
        .collect()
}

#[derive(Debug)]
pub enum Direction {
    Increasing(i32),
    Decreasing(i32),
    Neither,
}

pub fn check_report(report: &[i32]) -> bool {
    let directions = report
        .iter()
        .tuple_windows()
        .map(|(a, b)| match b - a {
            0 => Direction::Neither,
            diff if diff > 0 => Direction::Increasing(diff),
            diff if diff < 0 => Direction::Decreasing(diff),
            _ => unreachable!(),
        })
        .collect_vec();
    let all_changing_in_same_direction =
        directions
            .iter()
            .tuple_windows()
            .all(|(a, b)| match (a, b) {
                (Direction::Increasing(_), Direction::Increasing(_)) => true,
                (Direction::Decreasing(_), Direction::Decreasing(_)) => true,
                (Direction::Neither, _) | (_, Direction::Neither) => false,
                _ => false,
            });
    let all_safe_magnitude = directions.iter().all(|direction| match direction {
        Direction::Decreasing(diff) | Direction::Increasing(diff) => matches!(diff.abs(), 1..=3),
        _ => false,
    });

    all_changing_in_same_direction && all_safe_magnitude
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = "1 2 3\n4 5 6\n7 8 9\n";
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(read_input_from(input.as_bytes()).unwrap(), expected);
    }
}
