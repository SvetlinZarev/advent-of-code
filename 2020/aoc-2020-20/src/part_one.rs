use std::collections::HashMap;

use crate::{Grid, COLS, HEIGHT, IMAGE_HEIGHT, IMAGE_WIDTH, ROWS, WIDTH};

type Solution = [[(usize, Grid); COLS]; ROWS];

pub fn solve(input: &HashMap<usize, Grid>) -> Option<(usize, Vec<u8>)> {
    assert_eq!(ROWS * COLS, input.len());

    let mut grid = [[(0usize, [[false; WIDTH]; HEIGHT]); COLS]; ROWS];

    if on_tile(&mut grid, &input, 0, 0) {
        let checksum =
            grid[0][0].0 * grid[0][COLS - 1].0 * grid[ROWS - 1][0].0 * grid[ROWS - 1][COLS - 1].0;

        // Copy the corrected image in order to use it in part 02
        let image = to_part_two_input(&grid);

        return Some((checksum, image));
    }

    None
}

fn on_tile(grid: &mut Solution, remaining: &HashMap<usize, Grid>, row: usize, col: usize) -> bool {
    if remaining.is_empty() {
        return true;
    }

    let mut nr = row;
    let mut nc = col + 1;
    if nc >= COLS {
        nc = 0;
        nr += 1;
    }

    for (id, gr) in remaining.iter() {
        grid[row][col] = (*id, gr.clone());

        let mut remaining = remaining.clone();
        remaining.remove(id);

        for _ in 0..4 {
            grid_rotate_left(&mut grid[row][col].1);
            if is_matching(grid, row, col) {
                if on_tile(grid, &remaining, nr, nc) {
                    return true;
                }
            }
        }

        grid_flip_horizontal(&mut grid[row][col].1);

        for _ in 0..4 {
            grid_rotate_left(&mut grid[row][col].1);
            if is_matching(grid, row, col) {
                if on_tile(grid, &remaining, nr, nc) {
                    return true;
                }
            }
        }
    }

    false
}

fn is_matching(grid: &Solution, row: usize, col: usize) -> bool {
    let tile = &grid[row][col].1;

    if row > 0 {
        let top = &grid[row - 1][col].1;
        if top[top.len() - 1] != tile[0] {
            return false;
        }
    }

    if col > 0 {
        let left = &grid[row][col - 1].1;
        for (l, r) in left.iter().zip(tile.iter()) {
            if l[l.len() - 1] != r[0] {
                return false;
            }
        }
    }

    true
}

fn grid_rotate_left(grid: &mut Grid) {
    let original = grid.clone();
    let max_idx = grid.len() - 1;

    for r in 0..grid.len() {
        for c in 0..grid.len() {
            let x = original[r][c];
            grid[max_idx - c][r] = x;
        }
    }
}

fn grid_flip_horizontal(grid: &mut Grid) {
    for r in grid {
        r.reverse();
    }
}

fn to_part_two_input(grid: &Solution) -> Vec<u8> {
    let capacity = IMAGE_HEIGHT * IMAGE_WIDTH;
    let mut image = vec![b'X'; capacity];

    for sr in 0..ROWS {
        for sc in 0..COLS {
            let part = &grid[sr][sc].1;

            for r in 1..HEIGHT - 1 {
                for c in 1..WIDTH - 1 {
                    let ir = sr * (HEIGHT - 2) + (r - 1);
                    let ic = sc * (WIDTH - 2) + (c - 1);
                    let idx = ir * IMAGE_WIDTH + ic;
                    image[idx] = if part[r][c] { b'#' } else { b'.' };
                }
            }
        }
    }

    image
}
