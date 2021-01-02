use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

const DAY: usize = 18;
const WIDTH: usize = 100;
const STEPS: usize = 100;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let (dp, parsed) = measure(DAY, "parsing", || parse_input(&input, WIDTH));

    let (d1, _) = measure(DAY, "part 1", || solve_part_one(&parsed, WIDTH));
    let (d2, _) = measure(DAY, "part 2", || solve_part_two(&parsed, WIDTH));

    dp + d1 + d2
}

fn parse_input(input: &str, width: usize) -> Vec<u8> {
    let mut grid = Vec::with_capacity(width * width);

    for line in input.lines() {
        for &ch in line.as_bytes() {
            match ch {
                b'.' => grid.push(0),
                b'#' => grid.push(1),
                _ => panic!("Unexpected character: {}", ch as char),
            }
        }

        assert_eq!(0, grid.len() % width);
    }

    assert_eq!(width * width, grid.len());
    grid
}

fn solve_part_one(grid: &[u8], width: usize) -> usize {
    solve(&grid, width, false)
}

fn solve_part_two(grid: &[u8], width: usize) -> usize {
    solve(&grid, width, true)
}

fn solve(grid: &[u8], width: usize, with_broken_lights: bool) -> usize {
    assert_eq!(width * width, grid.len());
    let mut grid = grid.to_vec();
    let mut counts = vec![0; grid.len()];

    if with_broken_lights {
        set_broken_lights(&mut grid, width);
    }

    for _ in 0..STEPS {
        step(&mut grid, &mut counts, width);

        if with_broken_lights {
            set_broken_lights(&mut grid, width);
        }
    }

    grid.iter().copied().map(|b| b as usize).sum()
}

fn set_broken_lights(grid: &mut [u8], width: usize) {
    grid[0 * width + 0] = 1; //top left
    grid[0 * width + width - 1] = 1; //top right
    grid[(width - 1) * width + 0] = 1; //bottom left
    grid[(width - 1) * width + width - 1] = 1; //bottom right
}

fn step(grid: &mut [u8], counts: &mut [u8], width: usize) {
    for r in 0..width {
        let offset = r * width;

        for c in 0..width {
            let idx = offset + c;
            if grid[idx] > 0 {
                // upper row
                if r > 0 {
                    let offset = (r - 1) * width;

                    if c > 0 {
                        counts[offset + c - 1] += 1;
                    }

                    counts[offset + c] += 1;

                    if c < width - 1 {
                        counts[offset + c + 1] += 1;
                    }
                }

                // middle row
                {
                    let offset = r * width;

                    if c > 0 {
                        counts[offset + c - 1] += 1;
                    }

                    if c < width - 1 {
                        counts[offset + c + 1] += 1;
                    }
                }

                // bottom row
                if r < width - 1 {
                    let offset = (r + 1) * width;

                    if c > 0 {
                        counts[offset + c - 1] += 1;
                    }

                    counts[offset + c] += 1;

                    if c < width - 1 {
                        counts[offset + c + 1] += 1;
                    }
                }
            }
        }
    }

    for r in 0..width {
        let offset = r * width;

        for c in 0..width {
            let idx = offset + c;

            if counts[idx] == 3 {
                grid[idx] = 1;
            } else if counts[idx] == 2 && grid[idx] > 0 {
                grid[idx] = 1;
            } else {
                grid[idx] = 0;
            }

            counts[idx] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let parsed = parse_input(&input, WIDTH);
        let answer = solve_part_one(&parsed, WIDTH);

        assert_eq!(814, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let parsed = parse_input(&input, WIDTH);
        let answer = solve_part_two(&parsed, WIDTH);

        assert_eq!(924, answer);
    }
}
