use crate::{Grid, Horizontal, Tile, Vertical, NUM_COLS, NUM_ROWS};

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
                //corner-left
                if is_occupied(src, r, c, Vertical::Up, Horizontal::Left) {
                    occupied += 1;
                }

                //straight-up
                if is_occupied(src, r, c, Vertical::Up, Horizontal::None) {
                    occupied += 1;
                }

                //corner-right
                if is_occupied(src, r, c, Vertical::Up, Horizontal::Right) {
                    occupied += 1;
                }
            }

            //check left
            if is_occupied(src, r, c, Vertical::None, Horizontal::Left) {
                occupied += 1;
            }

            //check right
            if is_occupied(src, r, c, Vertical::None, Horizontal::Right) {
                occupied += 1;
            }

            // Check lower row
            if r < NUM_ROWS - 1 {
                //corner-left
                if is_occupied(src, r, c, Vertical::Down, Horizontal::Left) {
                    occupied += 1;
                }

                //straight-down
                if is_occupied(src, r, c, Vertical::Down, Horizontal::None) {
                    occupied += 1;
                }

                //corner-right
                if is_occupied(src, r, c, Vertical::Down, Horizontal::Right) {
                    occupied += 1;
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

fn is_occupied(grid: &Grid, row: usize, col: usize, dr: Vertical, dc: Horizontal) -> bool {
    if let Some((r, c)) = idx(row, col, dr, dc) {
        return Tile::Occupied == grid[r][c];
    }

    false
}

fn idx(r: usize, c: usize, dr: Vertical, dc: Horizontal) -> Option<(usize, usize)> {
    let (mut row, mut col) = (r, c);

    match dr {
        Vertical::None => {
            // no-op
        }
        Vertical::Up => {
            if row == 0 {
                return None;
            }

            row -= 1;
        }

        Vertical::Down => {
            if row == NUM_ROWS - 1 {
                return None;
            }
            row += 1;
        }
    }

    match dc {
        Horizontal::None => {
            // no-op
        }
        Horizontal::Left => {
            if col == 0 {
                return None;
            }
            col -= 1;
        }
        Horizontal::Right => {
            if col == NUM_COLS - 1 {
                return None;
            }
            col += 1;
        }
    }

    Some((row, col))
}
