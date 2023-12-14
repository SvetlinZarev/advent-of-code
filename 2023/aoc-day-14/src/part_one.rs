use crate::{EMPTY, FIXED, OVAL};

pub fn part_one(input: &Vec<Vec<u8>>) -> usize {
    let mut answer = 0;

    let mut grid = input.to_vec();
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
                        let x = grid[p][c];
                        grid[p][c] = grid[r][c];
                        grid[r][c] = x;
                    }

                    answer += rows - p;
                }

                EMPTY => {
                    // do nothing
                }

                _ => unreachable!("invalid input at {}:{}={}", r, c, grid[r][c] as char),
            }
        }
    }

    answer
}
