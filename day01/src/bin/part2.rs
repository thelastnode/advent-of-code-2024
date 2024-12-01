use day01::read_input;

use anyhow::Result;
use itertools::Itertools;

fn process(inputs: Vec<(i32, i32)>) -> usize {
    let lefts = inputs.iter().map(|(a, _)| a);
    let rights = inputs.iter().map(|&(_, b)| b).counts();

    lefts
        .map(|x| (*x as usize) * rights.get(x).unwrap_or(&0))
        .sum()
}

fn main() -> Result<()> {
    let inputs = read_input()?;
    println!("{}", process(inputs));

    Ok(())
}

#[cfg(test)]
mod tests {
    use day01::read_input_from;
    use textwrap::dedent;

    use super::*;

    #[test]
    fn test_example() {
        let input = dedent(
            "
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        ",
        );
        let input = input.trim();
        let input = read_input_from(input.as_bytes()).unwrap();
        assert_eq!(process(input), 31);
    }
}
