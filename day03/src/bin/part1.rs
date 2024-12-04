use std::io::{stdin, Read};

use anyhow::Result;
use regex::Regex;

fn process(input: &str) -> Result<i64> {
    let result = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)")?
        .captures_iter(input)
        .map(|captures| {
            let left = captures["left"].parse::<i64>().unwrap();
            let right = captures["right"].parse::<i64>().unwrap();
            left * right
        })
        .sum::<i64>();
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    println!("{}", process(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let output =
            process("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
                .unwrap();
        assert_eq!(output, 161);
    }
}
