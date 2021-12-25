const EAST: u8 = b'>';
const SOUTH: u8 = b'v';
const EMPTY: u8 = b'.';

pub fn part_one(mut grid: Vec<Vec<u8>>) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut iterations = 0;
    let mut previous_south = vec![false; cols];
    let mut first_row_empty_spots = vec![false; cols];
    let mut has_mutation = true;

    while has_mutation {
        has_mutation = false;
        iterations += 1;

        for c in 0..previous_south.len() {
            previous_south[c] = grid[0][c] == SOUTH;
        }

        for r in 0..rows {
            let first = grid[r][0];
            let last = grid[r][cols - 1];

            let mut c = cols - 1;
            while c > 0 {
                if grid[r][c] == EMPTY && grid[r][c - 1] == EAST {
                    grid[r][c - 1] = EMPTY;
                    grid[r][c] = EAST;
                    has_mutation = true;
                    c -= 1;
                }

                c = c.saturating_sub(1);

                while c > 0 && grid[r][c] != EMPTY {
                    c -= 1;
                }
            }

            if last == EAST && first == EMPTY {
                grid[r][0] = EAST;
                grid[r][cols - 1] = EMPTY;
                has_mutation = true;
            }

            if r == 0 {
                for c in 0..cols {
                    first_row_empty_spots[c] = grid[r][c] == EMPTY;
                }
            }

            if r > 0 {
                for c in 0..grid[r].len() {
                    if grid[r][c] == EMPTY && previous_south[c] {
                        previous_south[c] = false;
                        grid[r][c] = SOUTH;
                        grid[r - 1][c] = EMPTY;
                        has_mutation = true;
                    } else {
                        previous_south[c] = grid[r][c] == SOUTH;
                    }
                }
            }
        }

        for c in 0..previous_south.len() {
            if first_row_empty_spots[c] && previous_south[c] {
                grid[0][c] = SOUTH;
                grid[rows - 1][c] = EMPTY;
                has_mutation = true;
            }
        }
    }

    iterations
}
