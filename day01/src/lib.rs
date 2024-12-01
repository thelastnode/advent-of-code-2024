use std::io::{stdin, BufRead};

use anyhow::{anyhow, Result};

pub fn read_input() -> Result<Vec<(i32, i32)>> {
    read_input_from(stdin().lock())
}

pub fn read_input_from(stdin: impl BufRead) -> Result<Vec<(i32, i32)>> {
    let mut numbers: Vec<(i32, i32)> = Vec::new();
    for line in stdin.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<_, _>>()?;
        if nums.len() == 2 {
            numbers.push((nums[0], nums[1]));
        } else {
            return Err(anyhow!("Expected two numbers per line"));
        }
    }
    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = "1 2\n3 4";
        let expected = vec![(1, 2), (3, 4)];
        let numbers = read_input_from(input.as_bytes()).unwrap();
        assert_eq!(numbers, expected);
    }
}
