use std::collections::HashMap;
use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 20;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

const ROWS: usize = 12;
const COLS: usize = 12;

const IMAGE_WIDTH: usize = COLS * (WIDTH - 2);
const IMAGE_HEIGHT: usize = ROWS * (HEIGHT - 2);

pub type Grid = [[bool; WIDTH]; HEIGHT];

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let (dp, grids) = measure(DAY, "parsing", || parse_input(&input));

    let mut image = None;
    let (d1, _) = measure(DAY, "part 1", || {
        if let Some((solution, img)) = part_one::solve(&grids) {
            image = Some(img);
            return Some(solution);
        }

        None
    });

    let mut d2 = Duration::default();
    if let Some(mut img) = image {
        let (d2x, _) = measure(DAY, "part 2", || part_two::solve(&mut img));
        d2 = d2x;
    }

    dp.add(d1).add(d2)
}

pub fn parse_input(input: &str) -> HashMap<usize, Grid> {
    let mut map = HashMap::new();

    let mut row = 0;
    let mut title = "";
    let mut grid = [[false; WIDTH]; HEIGHT];

    for line in input.lines() {
        if line.is_empty() {
            assert!(!title.is_empty());

            let tile_id = title[5..title.len() - 1].parse().unwrap();
            map.insert(tile_id, grid);

            title = "";
            row = 0;

            continue;
        }

        if title.is_empty() {
            title = line;
            continue;
        }

        for (col, b) in line.as_bytes().iter().copied().enumerate() {
            let flag = match b {
                b'.' => false,
                b'#' => true,
                _ => panic!("Unexpected input on line: {}", line),
            };

            grid[row][col] = flag;
        }
        row += 1;
    }

    map
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let grids = parse_input(&input);

        let (solution, _) = part_one::solve(&grids).unwrap();
        assert_eq!(174206308298779, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let grids = parse_input(&input);

        let (_, mut image) = part_one::solve(&grids).unwrap();

        let solution = part_two::solve(&mut image);
        assert_eq!(2409, solution);
    }
}
