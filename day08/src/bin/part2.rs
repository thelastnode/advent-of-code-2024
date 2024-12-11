use anyhow::Result;
use day08::{count_antinodes, parse, Location};
use std::io::{stdin, Read};

fn calculate_mirror(
    &(rows, cols): &(usize, usize),
    base: &(usize, usize),
    other: &(usize, usize),
) -> impl Iterator<Item = (isize, isize)> {
    let &(base_row, base_col) = base;
    let &(other_row, other_col) = other;

    let row_diff = base_row as isize - other_row as isize;
    let col_diff = base_col as isize - other_col as isize;

    (0..)
        .map(move |n| {
            (
                (base_row as isize + n * row_diff),
                (base_col as isize + n * col_diff),
            )
        })
        .take_while(move |&(r, c)| r >= 0 && c >= 0 && r < rows as isize && c < cols as isize)
}

fn process(input: Vec<Vec<Location>>) -> usize {
    let dims = (input.len(), input[0].len());
    count_antinodes(&input, |base, other| calculate_mirror(&dims, base, other))
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let input = parse(&input);

    println!("{}", process(input));

    Ok(())
}
