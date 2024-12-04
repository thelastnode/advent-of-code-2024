use anyhow::Result;
use day04::{read_input, Grid};
use itertools::Itertools;

const SEARCH_TERM: &[u8] = "MAS".as_bytes();
#[rustfmt::skip]
const DIAGONALS: &[&[(isize, isize)]] = &[
    &[(-1, -1), (0, 0), (1, 1)],
    &[(-1, 1), (0, 0), (1, -1)]
];

fn check_diag<'a, I>(text: I) -> bool
where
    I: Iterator<Item = &'a u8>,
{
    text.enumerate().all(|(i, &x)| x == SEARCH_TERM[i])
}

fn is_valid_x(grid: &Grid, center_row: usize, center_col: usize) -> bool {
    for diag in DIAGONALS {
        let text = diag
            .iter()
            .map(|&(dx, dy)| {
                let (row, col) = (center_row as isize + dx, center_col as isize + dy);
                if grid.is_valid(row, col) {
                    grid[(row, col)]
                } else {
                    b'.'
                }
            })
            .collect_vec();

        let matches_forward = check_diag(text.iter());
        let matches_backward = check_diag(text.iter().rev());

        if !matches_forward && !matches_backward {
            return false;
        }
    }

    true
}

fn process(grid: &Grid) -> i64 {
    let mut count = 0;

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid[(row as isize, col as isize)] == b'A' && is_valid_x(grid, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn main() -> Result<()> {
    let input = read_input()?;

    println!("{}", process(&input));

    Ok(())
}
