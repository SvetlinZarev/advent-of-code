use std::cmp::Ordering;
use std::collections::VecDeque;

const CELL_ROCKS: u8 = b'#';
const CELL_SAND: u8 = b'o';
const CELL_FREE: u8 = b' ';
const DEFAULT_COL: usize = 500;

pub fn parse_input(input: impl AsRef<str>) -> (Vec<Vec<u8>>, usize, usize) {
    let input = input.as_ref();
    let mut min_col = DEFAULT_COL;
    let mut max_col = usize::MIN;
    let mut max_row = usize::MIN;

    for line in input.lines() {
        for pair in line.split(" -> ") {
            let (col, row) = pair.split_once(',').unwrap();
            let col = col.parse().unwrap();
            let row = row.parse().unwrap();

            min_col = min_col.min(col);
            max_col = max_col.max(col);
            max_row = max_row.max(row);
        }
    }

    let width = (max_col - min_col).max((max_row + 1) * 2) + 3;
    let shift = match (DEFAULT_COL - min_col).cmp(&(width / 2)) {
        Ordering::Equal => 0,
        Ordering::Less => width / 2 - (DEFAULT_COL - min_col),
        Ordering::Greater => todo!()
    };

    let mut grid = vec![vec![CELL_FREE; width]; max_row + 3];
    for line in input.lines() {
        let mut split = line.split(" -> ");
        let (sc, sr) = split.next().unwrap().split_once(',').unwrap();
        let (mut sc, mut sr): (usize, usize) = (sc.parse().unwrap(), sr.parse().unwrap());

        for pair in split {
            let (col, row) = pair.split_once(',').unwrap();
            let col = col.parse().unwrap();
            let row = row.parse().unwrap();

            // draw vertically
            if sc == col {
                for r in sr.min(row)..=sr.max(row) {
                    grid[r][col - min_col + shift] = CELL_ROCKS
                }
            } else {
                assert_eq!(sr, row);
                for c in sc.min(col)..=sc.max(col) {
                    grid[row][c - min_col + shift] = CELL_ROCKS;
                }
            }

            sc = col;
            sr = row;
        }
    }
    grid[max_row + 2].fill(CELL_ROCKS);

    (grid, max_row, DEFAULT_COL - min_col + shift)
}

pub fn part_one(mut grid: Vec<Vec<u8>>, last_row: usize, initial_column: usize) -> usize {
    let mut sand_particles = 0;
    'all: loop {
        let (mut r, mut c) = (0, initial_column);

        loop {
            if r + 1 > last_row {
                break 'all;
            }

            if grid[r + 1][c] == CELL_FREE {
                r += 1;
                continue;
            }


            if c == 0 {
                break 'all;
            }

            if grid[r + 1][c - 1] == CELL_FREE {
                r += 1;
                c -= 1;
                continue;
            }

            if c == grid[r].len() - 1 {
                break 'all;
            }

            if grid[r + 1][c + 1] == CELL_FREE {
                r += 1;
                c += 1;
                continue;
            }

            grid[r][c] = CELL_SAND;
            sand_particles += 1;
            break;
        }
    }

    sand_particles
}

pub fn part_two_v1(mut grid: Vec<Vec<u8>>, initial_column: usize) -> usize {
    let mut sand_particles = 0;
    while grid[0][initial_column] == CELL_FREE {
        let (mut r, mut c) = (0, initial_column);

        loop {
            if grid[r + 1][c] == CELL_FREE {
                r += 1;
                continue;
            }

            if grid[r + 1][c - 1] == CELL_FREE {
                r += 1;
                c -= 1;
                continue;
            }

            if grid[r + 1][c + 1] == CELL_FREE {
                r += 1;
                c += 1;
                continue;
            }

            grid[r][c] = CELL_SAND;
            sand_particles += 1;
            break;
        }
    }

    sand_particles
}

pub fn part_two_v2(mut grid: Vec<Vec<u8>>, initial_column: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((0, initial_column));

    grid[0][initial_column] = CELL_SAND;
    let mut particles = 1;

    while let Some((r, c)) = queue.pop_front() {
        let row = r + 1;
        for cx in [-1, 0, 1] {
            if let Some(col) = c.checked_add_signed(cx) {
                if grid[row][col] == CELL_FREE {
                    grid[row][col] = CELL_SAND;
                    queue.push_back((row, col));
                    particles += 1;
                }
            }
        }
    }

    particles
}


#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use crate::{parse_input, part_one, part_two_v1, part_two_v2};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (grid, last_row, initial_column) = parse_input(input);
        let answer = part_one(grid, last_row, initial_column);

        assert_eq!(755, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (grid, _, initial_column) = parse_input(input);
        let answer = part_two_v1(grid, initial_column);

        assert_eq!(29805, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (grid, _, initial_column) = parse_input(input);
        let answer = part_two_v2(grid, initial_column);

        assert_eq!(29805, answer);
    }
}
