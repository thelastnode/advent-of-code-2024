use anyhow::Result;
use day04::{read_input, Grid};

const DIRECTIONS: &[(isize, isize)] = &[
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];
const SEARCH_TERM: &[u8] = "XMAS".as_bytes();

fn is_match(grid: &Grid, start_row: usize, start_col: usize, dx: isize, dy: isize) -> bool {
    let mut row = start_row as isize;
    let mut col = start_col as isize;

    for &c in SEARCH_TERM {
        if !grid.is_valid(row, col) || grid[(row, col)] != c {
            return false;
        }
        row += dx;
        col += dy;
    }
    true
}

fn process(grid: &Grid) -> i64 {
    let mut count = 0;

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            for &(dx, dy) in DIRECTIONS {
                if is_match(grid, row, col, dx, dy) {
                    count += 1;
                }
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
