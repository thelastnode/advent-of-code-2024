use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Position(pub isize, pub isize);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn offset(&self, pos: &Position) -> Position {
        match self {
            Direction::Up => Position(pos.0 - 1, pos.1),
            Direction::Down => Position(pos.0 + 1, pos.1),
            Direction::Left => Position(pos.0, pos.1 - 1),
            Direction::Right => Position(pos.0, pos.1 + 1),
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Cell {
    Obstacle,
    Guard(Direction),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    pub map: FxHashMap<Position, Cell>,
    pub rows: isize,
    pub cols: isize,
}

impl Input {
    pub fn is_inside(&self, pos: &Position) -> bool {
        pos.0 >= 0 && pos.0 < self.rows && pos.1 >= 0 && pos.1 < self.cols
    }

    pub fn traverse<T: FnMut(&Position, &Direction) -> bool>(
        &self,
        start_pos: &Position,
        start_dir: &Direction,
        mut visit: T,
    ) {
        let (mut guard_position, mut guard_dir) = (start_pos.clone(), start_dir.clone());

        loop {
            if visit(&guard_position, &guard_dir) {
                return;
            }
            let new_pos = guard_dir.offset(&guard_position);

            match self.map.get(&new_pos) {
                Some(Cell::Obstacle) => guard_dir = guard_dir.turn_right(),
                Some(Cell::Guard(_)) | None => {
                    if !self.is_inside(&new_pos) {
                        return;
                    }
                    guard_position = new_pos;
                }
            }
        }
    }

    pub fn with_obstacle(&self, obstacle_pos: &Position) -> Input {
        Input {
            map: self
                .map
                .iter()
                .map(|(pos, cell)| (pos.clone(), cell.clone()))
                .chain(std::iter::once((obstacle_pos.clone(), Cell::Obstacle)))
                .collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub fn does_loop(&self, start_pos: &Position, start_dir: &Direction) -> bool {
        let mut visited = FxHashSet::<(Position, Direction)>::default();
        let mut looped = false;

        self.traverse(start_pos, start_dir, |pos, dir| {
            let new_loc = (pos.clone(), dir.clone());
            if visited.contains(&new_loc) {
                looped = true;
                return true;
            }
            visited.insert(new_loc);
            false
        });

        looped
    }
}

pub fn parse_input(input: &str) -> Input {
    let map = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                match c {
                    '.' => None,
                    '#' => Some(Cell::Obstacle),
                    '^' => Some(Cell::Guard(Direction::Up)),
                    'v' => Some(Cell::Guard(Direction::Down)),
                    '<' => Some(Cell::Guard(Direction::Left)),
                    '>' => Some(Cell::Guard(Direction::Right)),
                    x => panic!("Invalid character '{x}' in input"),
                }
                .map(|cell| (Position(row as isize, col as isize), cell))
            })
        })
        .collect();

    Input {
        map,
        rows: input.lines().count() as isize,
        cols: input.lines().next().unwrap().len() as isize,
    }
}

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashMap;
    use textwrap::dedent;

    use super::*;

    #[test]
    fn test_read_input() {
        let input = dedent(
            "
            ..#
            #..
            ..^",
        );
        let input = input.trim();
        let expected = {
            let mut map = FxHashMap::default();
            map.insert(Position(0, 2), Cell::Obstacle);
            map.insert(Position(1, 0), Cell::Obstacle);
            map.insert(Position(2, 2), Cell::Guard(Direction::Up));
            map
        };
        assert_eq!(parse_input(input).map, expected);
    }
}
