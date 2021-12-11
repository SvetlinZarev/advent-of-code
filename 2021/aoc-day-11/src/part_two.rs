pub fn part_two(input: &[Vec<i8>]) -> u64 {
    let mut grid = input.to_vec();
    let mut flashes = 0;
    let mut iterations = 0;

    let synced_flashes = grid.len() * grid[0].len();

    while flashes != synced_flashes {
        flashes = 0;

        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] >= 0 {
                    grid[r][c] += 1;
                    if grid[r][c] > 9 {
                        flash(&mut grid, r, c);
                    }
                }
            }
        }

        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] < 0 {
                    grid[r][c] = 0;
                    flashes += 1;
                }
            }
        }

        iterations += 1;
    }

    iterations
}

fn flash(grid: &mut [Vec<i8>], r: usize, c: usize) {
    grid[r][c] = -1;

    if r > 0 {
        // top-left
        if c > 0 {
            if grid[r - 1][c - 1] >= 0 {
                grid[r - 1][c - 1] += 1;
                if grid[r - 1][c - 1] > 9 {
                    flash(grid, r - 1, c - 1);
                }
            }
        }

        // top-center
        if grid[r - 1][c] >= 0 {
            grid[r - 1][c] += 1;
            if grid[r - 1][c] > 9 {
                flash(grid, r - 1, c);
            }
        }

        // top-right
        if c < grid[r].len() - 1 {
            if grid[r - 1][c + 1] >= 0 {
                grid[r - 1][c + 1] += 1;
                if grid[r - 1][c + 1] > 9 {
                    flash(grid, r - 1, c + 1);
                }
            }
        }
    }

    //left
    if c > 0 {
        if grid[r][c - 1] >= 0 {
            grid[r][c - 1] += 1;
            if grid[r][c - 1] > 9 {
                flash(grid, r, c - 1);
            }
        }
    }

    // right
    if c < grid[r].len() - 1 {
        if grid[r][c + 1] >= 0 {
            grid[r][c + 1] += 1;
            if grid[r][c + 1] > 9 {
                flash(grid, r, c + 1);
            }
        }
    }

    //bottom-left
    if r < grid.len() - 1 {
        if c > 0 {
            if grid[r + 1][c - 1] >= 0 {
                grid[r + 1][c - 1] += 1;
                if grid[r + 1][c - 1] > 9 {
                    flash(grid, r + 1, c - 1);
                }
            }
        }

        // bottom-center
        if grid[r + 1][c] >= 0 {
            grid[r + 1][c] += 1;
            if grid[r + 1][c] > 9 {
                flash(grid, r + 1, c);
            }
        }

        // bottom-right
        if c < grid[r].len() - 1 {
            if grid[r + 1][c + 1] >= 0 {
                grid[r + 1][c + 1] += 1;
                if grid[r + 1][c + 1] > 9 {
                    flash(grid, r + 1, c + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_i8_numeric_grid;

    use super::*;

    #[test]
    fn test_part_two() {
        let input = parse_i8_numeric_grid(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(&input);
        assert_eq!(437, answer);
    }
}
