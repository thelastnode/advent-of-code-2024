use day01::read_input;

use anyhow::Result;

fn process(inputs: Vec<(i32, i32)>) -> i32 {
    let mut lefts = inputs.iter().map(|&(a, _)| a).collect::<Vec<i32>>();
    let mut rights = inputs.iter().map(|&(_, b)| b).collect::<Vec<i32>>();

    lefts.sort();
    rights.sort();

    lefts
        .iter()
        .zip(rights.iter())
        .map(|(a, b)| (a - b).abs())
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
        assert_eq!(process(input), 11);
    }
}
