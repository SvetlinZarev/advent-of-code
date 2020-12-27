use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_one_v1;
pub mod part_one_v2;
pub mod part_one_v3;
pub mod part_two;

pub const DAY: usize = 11;

const NUM_COLS: usize = 96;
const NUM_ROWS: usize = 99;

type Grid = [[Tile; NUM_COLS]; NUM_ROWS];

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (d1p, grid) = measure(DAY, "parsing", || parse_input(&input));
    let (d1a, _) = measure(DAY, "part 1: v1", || part_one_v1::solve(&mut grid.clone()));
    let (d1b, _) = measure(DAY, "part 1: v2", || part_one_v2::solve(&mut grid.clone()));
    let (d1cp, mut seats) = measure(DAY, "parsing", || part_one_v3::parse_input(&input));
    let (d1c, _) = measure(DAY, "part 1: v3", || part_one_v3::solve(&mut seats));
    let (d2, _) = measure(DAY, "part 2", || part_two::solve(&mut grid.clone()));

    let s1 = d1p.add(d1a.min(d1b)).add(d2);
    let s2 = d1cp.add(d1c).add(d1p).add(d2);
    s1.min(s2)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Floor,
    Free,
    Occupied,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Horizontal {
    None,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Vertical {
    None,
    Up,
    Down,
}

pub fn parse_input(input: &str) -> Grid {
    let mut grid = [[Tile::Free; NUM_COLS]; NUM_ROWS];
    init_grid(&mut grid, &input);
    grid
}

fn init_grid(grid: &mut Grid, input: &str) {
    for (line_idx, line) in input.lines().enumerate() {
        let line = line.as_bytes();
        let row = &mut grid[line_idx];

        for (col_idx, col) in line.iter().copied().enumerate() {
            row[col_idx] = match col {
                b'.' => Tile::Floor,
                b'L' => Tile::Free,
                b'#' => Tile::Occupied,
                _ => unreachable!("Invalid input: {} ({}/{})", col, line_idx, col_idx),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));

        let mut grid = parse_input(&input);
        let solution = part_one_v1::solve(&mut grid);
        assert_eq!(2424, solution);

        let mut grid = parse_input(&input);
        let solution = part_one_v2::solve(&mut grid);
        assert_eq!(2424, solution);

        let mut seats = part_one_v3::parse_input(&input);
        let solution = part_one_v3::solve(&mut seats);
        assert_eq!(2424, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));

        let mut grid = parse_input(&input);
        let solution = part_two::solve(&mut grid);
        assert_eq!(2208, solution);
    }
}
