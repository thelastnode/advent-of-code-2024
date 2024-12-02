use anyhow::Result;
use day02::read_input;
use itertools::Itertools;

enum Direction {
    Increasing(i32),
    Decreasing(i32),
    Neither,
}

fn check_report(report: &[i32]) -> bool {
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
