use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use std::path::Path;

pub mod part_one_v1;
pub mod part_one_v2;
pub mod part_one_v3;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-11.txt";

const NUM_COLS: usize = 96;
const NUM_ROWS: usize = 99;

type Grid = [[Tile; NUM_COLS]; NUM_ROWS];

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);

    let mut grid = parse_input(&input);
    measure_solution(11, 1, "v1", || part_one_v1::solve(&mut grid));

    let mut grid = parse_input(&input);
    measure_solution(11, 1, "v2", || part_one_v2::solve(&mut grid));

    let mut seats = part_one_v3::parse_input(&input);
    measure_solution(11, 1, "v3", || part_one_v3::solve(&mut seats));

    let mut grid = parse_input(&input);
    measure_solution(11, 2, "", || part_two::solve(&mut grid));
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
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);

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
        let input = load_input(DEFAULT_INPUT_PATH);

        let mut grid = parse_input(&input);
        let solution = part_two::solve(&mut grid);
        assert_eq!(2208, solution);
    }
}
