use anyhow::Result;
use day02::{check_report, read_input};

fn process(input: Vec<Vec<i32>>) -> usize {
    input
        .iter()
        .filter(|&report| {
            if check_report(report) {
                return true;
            }

            for i in 0..report.len() {
                let mut adjusted = report.clone();
                adjusted.remove(i);
                if check_report(&adjusted) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn main() -> Result<()> {
    let input = read_input()?;
    println!("{}", process(input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use day02::read_input_from;
    use textwrap::dedent;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = dedent(
            "\
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        ",
        );
        let input = input.trim();
        let result = process(read_input_from(input.as_bytes())?);
        assert_eq!(result, 4);
        Ok(())
    }
}
