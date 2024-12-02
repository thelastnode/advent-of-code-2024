use std::io::{stdin, BufRead};

use anyhow::Result;

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
