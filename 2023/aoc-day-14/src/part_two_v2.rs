use aoc_shared::hashing::FxHashMap;

use crate::{EMPTY, FIXED, OVAL};

type HashMap<K, V> = FxHashMap<K, V>;

const TOTAL_CYCLES: u32 = 1_000_000_000;

pub fn part_two(input: &[u8]) -> usize {
    let mut grid = input.to_vec();

    // Assume the input file uses only '\n' for new lines.
    // Because of that additional new line, we'll
    // have to use `cols-1` when working with the columns
    let cols = grid.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = grid.len() / cols;

    let mut seen = HashMap::default();
    seen.insert(identity(&grid), 0);

    let mut cycle_start = 0;
    let mut cycle_len = 0;

    for c in 0..TOTAL_CYCLES {
        cycle(&mut grid, rows, cols);
        if let Some(prev) = seen.insert(identity(&grid), c + 1) {
            cycle_start = prev;
            cycle_len = c - prev + 1;
            break;
        }
    }

    for _ in 0..(TOTAL_CYCLES - cycle_start) % cycle_len {
        cycle(&mut grid, rows, cols);
    }

    calc_beam_load(&grid, rows, cols)
}

fn cycle(grid: &mut Vec<u8>, rows: usize, cols: usize) {
    tilt_north(grid, rows, cols);
    tilt_west(grid, rows, cols);
    tilt_south(grid, rows, cols);
    tilt_east(grid, rows, cols);
}

fn tilt_north(grid: &mut Vec<u8>, rows: usize, cols: usize) {
    for c in 0..cols - 1 {
        let mut p = 0;

        for r in 0..rows {
            match grid[r * cols + c] {
                FIXED => {
                    p = r + 1;
                }

                OVAL => {
                    while p < r && grid[p * cols + c] != EMPTY {
                        p += 1;
                    }

                    if p != r {
                        grid[p * cols + c] = OVAL;
                        grid[r * cols + c] = EMPTY;
                    }

                    p += 1;
                }

                EMPTY => {
                    // do nothing
                }

                _ => unreachable!(
                    "invalid input at {}:{}={}",
                    r,
                    c,
                    grid[r * cols + c] as char
                ),
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<u8>, rows: usize, cols: usize) {
    for c in 0..cols - 1 {
        let mut p = rows - 1;

        for r in (0..rows).rev() {
            match grid[r * cols + c] {
                FIXED => {
                    p = r.saturating_sub(1);
                }

                OVAL => {
                    while p > r && grid[p * cols + c] != EMPTY {
                        p -= 1;
                    }

                    if p != r {
                        grid[p * cols + c] = OVAL;
                        grid[r * cols + c] = EMPTY;
                    }

                    p = p.saturating_sub(1);
                }

                EMPTY => {
                    // do nothing
                }

                _ => unreachable!(
                    "invalid input at {}:{}={}",
                    r,
                    c,
                    grid[r * cols + c] as char
                ),
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<u8>, rows: usize, cols: usize) {
    for r in 0..rows {
        let mut p = 0;

        for c in 0..cols - 1 {
            match grid[r * cols + c] {
                FIXED => {
                    p = c + 1;
                }

                OVAL => {
                    while p < c && grid[r * cols + p] != EMPTY {
                        p += 1;
                    }

                    if p != c {
                        grid[r * cols + p] = OVAL;
                        grid[r * cols + c] = EMPTY;
                    }

                    p += 1;
                }

                EMPTY => {
                    // do nothing
                }

                _ => unreachable!(
                    "invalid input at {}:{}={}",
                    r,
                    c,
                    grid[r * cols + c] as char
                ),
            }
        }
    }
}
fn tilt_east(grid: &mut Vec<u8>, rows: usize, cols: usize) {
    for r in 0..rows {
        let mut p = cols - 2;

        for c in (0..cols - 1).rev() {
            match grid[r * cols + c] {
                FIXED => {
                    p = c.saturating_sub(1);
                }

                OVAL => {
                    while p > c && grid[r * cols + p] != EMPTY {
                        p -= 1;
                    }

                    if p != c {
                        grid[r * cols + p] = OVAL;
                        grid[r * cols + c] = EMPTY;
                    }

                    p = p.saturating_sub(1);
                }

                EMPTY => {
                    // do nothing
                }

                _ => unreachable!(
                    "invalid input at {}:{}={}",
                    r,
                    c,
                    grid[r * cols + c] as char
                ),
            }
        }
    }
}

fn calc_beam_load(grid: &[u8], rows: usize, cols: usize) -> usize {
    let mut answer = 0;

    for r in 0..rows {
        for c in 0..cols - 1 {
            if grid[r * cols + c] == OVAL {
                answer += rows - r;
            }
        }
    }

    answer
}

fn identity(grid: &[u8]) -> Vec<u8> {
    grid.to_vec()
}
