use aoc_shared::hashing::FxHashMap;

use crate::{EMPTY, FIXED, OVAL};

type HashMap<K, V> = FxHashMap<K, V>;

const TOTAL_CYCLES: u32 = 1_000_000_000;

pub fn part_two(input: &Vec<Vec<u8>>) -> usize {
    let mut grid = input.to_vec();
    let mut seen = HashMap::default();
    seen.insert(identity(&grid), 0);

    let mut cycle_start = 0;
    let mut cycle_len = 0;

    for c in 0..TOTAL_CYCLES {
        cycle(&mut grid);
        if let Some(prev) = seen.insert(identity(&grid), c + 1) {
            cycle_start = prev;
            cycle_len = c - prev + 1;
            break;
        }
    }

    for _ in 0..(TOTAL_CYCLES - cycle_start) % cycle_len {
        cycle(&mut grid);
    }

    calc_beam_load(&grid)
}

fn cycle(grid: &mut Vec<Vec<u8>>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn tilt_north(grid: &mut Vec<Vec<u8>>) {
    let rows = grid.len();
    let cols = grid[0].len();

    for c in 0..cols {
        let mut p = 0;

        for r in 0..rows {
            match grid[r][c] {
                FIXED => {
                    p = r;
                }

                OVAL => {
                    while p < r && grid[p][c] != EMPTY {
                        p += 1;
                    }

                    if p != r {
                        grid[p][c] = OVAL;
                        grid[r][c] = EMPTY;
                    }
                }

                EMPTY => {
                    // do nothing
                }

                _ => unreachable!("invalid input at {}:{}={}", r, c, grid[r][c] as char),
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<u8>>) {
    let rows = grid.len();
    let cols = grid[0].len();

    for c in 0..cols {
        let mut p = rows - 1;

        for r in (0..rows).rev() {
            match grid[r][c] {
                FIXED => {
                    p = r;
                }

                OVAL => {
                    while p > r && grid[p][c] != EMPTY {
                        p -= 1;
                    }

                    if p != r {
                        grid[p][c] = OVAL;
                        grid[r][c] = EMPTY;
                    }
                }

                EMPTY => {
                    // do nothing
                }

                _ => unreachable!("invalid input at {}:{}={}", r, c, grid[r][c] as char),
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<u8>>) {
    for r in 0..grid.len() {
        let mut p = 0;

        for c in 0..grid[r].len() {
            match grid[r][c] {
                FIXED => {
                    p = c;
                }

                OVAL => {
                    while p < c && grid[r][p] != EMPTY {
                        p += 1;
                    }

                    if p != c {
                        grid[r][p] = OVAL;
                        grid[r][c] = EMPTY;
                    }
                }

                EMPTY => {
                    // do nothing
                }

                _ => unreachable!("invalid input at {}:{}={}", r, c, grid[r][c] as char),
            }
        }
    }
}
fn tilt_east(grid: &mut Vec<Vec<u8>>) {
    for r in 0..grid.len() {
        let mut p = grid[r].len() - 1;

        for c in (0..grid[r].len()).rev() {
            match grid[r][c] {
                FIXED => {
                    p = c;
                }

                OVAL => {
                    while p > c && grid[r][p] != EMPTY {
                        p -= 1;
                    }

                    if p != c {
                        grid[r][p] = OVAL;
                        grid[r][c] = EMPTY;
                    }
                }

                EMPTY => {
                    // do nothing
                }

                _ => unreachable!("invalid input at {}:{}={}", r, c, grid[r][c] as char),
            }
        }
    }
}

fn calc_beam_load(grid: &[Vec<u8>]) -> usize {
    let mut answer = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == OVAL {
                answer += grid.len() - r;
            }
        }
    }

    answer
}

fn identity(grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    grid.to_vec()
}
