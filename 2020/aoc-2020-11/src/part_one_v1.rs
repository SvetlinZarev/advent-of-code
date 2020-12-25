use crate::{Grid, Tile, NUM_COLS, NUM_ROWS};

pub fn solve(src: &mut Grid) -> usize {
    let mut dst_grid = src.clone();

    let mut s = src;
    let mut d = &mut dst_grid;

    loop {
        match next_state(s, d) {
            None => std::mem::swap(&mut s, &mut d),
            Some(value) => break value,
        }
    }
}

fn next_state(src: &Grid, dst: &mut Grid) -> Option<usize> {
    for r in 0..NUM_ROWS {
        for c in 0..NUM_COLS {
            if Tile::Floor == src[r][c] {
                continue;
            }

            let mut occupied = 0;

            // Upper row
            if r > 0 {
                //check left
                if c > 0 {
                    if Tile::Occupied == src[r - 1][c - 1] {
                        occupied += 1;
                    }
                }

                // straight up
                if Tile::Occupied == src[r - 1][c] {
                    occupied += 1;
                }

                //check right
                if c < NUM_COLS - 1 {
                    if Tile::Occupied == src[r - 1][c + 1] {
                        occupied += 1;
                    }
                }
            }

            //check left
            if c > 0 {
                if Tile::Occupied == src[r][c - 1] {
                    occupied += 1;
                }
            }

            //check right
            if c < NUM_COLS - 1 {
                if Tile::Occupied == src[r][c + 1] {
                    occupied += 1;
                }
            }

            // Check lower row
            if r < NUM_ROWS - 1 {
                //check left
                if c > 0 {
                    if Tile::Occupied == src[r + 1][c - 1] {
                        occupied += 1;
                    }
                }

                // straight down
                if Tile::Occupied == src[r + 1][c] {
                    occupied += 1;
                }

                //check right
                if c < NUM_COLS - 1 {
                    if Tile::Occupied == src[r + 1][c + 1] {
                        occupied += 1;
                    }
                }
            }

            if occupied == 0 {
                dst[r][c] = Tile::Occupied;
            } else if occupied >= 4 {
                dst[r][c] = Tile::Free;
            } else {
                dst[r][c] = src[r][c];
            }
        }
    }

    let mut occupied = 0;
    for r in 0..NUM_ROWS {
        for c in 0..NUM_COLS {
            if src[r][c] != dst[r][c] {
                return None;
            }

            if Tile::Occupied == src[r][c] {
                occupied += 1;
            }
        }
    }
    Some(occupied)
}
