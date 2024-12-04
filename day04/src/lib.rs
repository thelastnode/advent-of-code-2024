use std::{fmt, io::stdin, ops};

use anyhow::Result;
use itertools::Itertools;

pub struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    pub fn rows(&self) -> usize {
        self.grid.len()
    }

    pub fn cols(&self) -> usize {
        self.grid[0].len()
    }

    pub fn is_valid(&self, row: isize, col: isize) -> bool {
        row >= 0 && col >= 0 && row < self.rows() as isize && col < self.cols() as isize
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for &cell in row {
                write!(f, "{}", cell as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl ops::Index<(isize, isize)> for Grid {
    type Output = u8;

    fn index(&self, (row, col): (isize, isize)) -> &Self::Output {
        &self.grid[row as usize][col as usize]
    }
}

pub fn read_input() -> Result<Grid> {
    Ok(Grid {
        grid: stdin()
            .lines()
            .map(|line| -> Result<Vec<u8>> { Ok(line?.bytes().collect_vec()) })
            .try_collect()?,
    })
}
