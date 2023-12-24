use std::collections::VecDeque;

use aoc_shared::hashing::FxHashSet;

type HashSet<T> = FxHashSet<T>;


const DIR: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
/*
   General observations:
   * the elf switches between "odd" and "even" positions on each step
   * because the elf can simultaneously go to all 4 neighbouring cells,
     on each step we only **expand** the number of possible positions
     he can be - i.e. once he's been on a grid[r][c], we'll always be
     able to find him there (depending on if it's an even or odd
     cycle - se ethe first pint)
*/

pub fn part_one(input: &str) -> usize {
    const STEPS: usize = 64;

    let cols = input.as_bytes().iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = input.len() / cols;

    let (r, c) = find_start(input.as_bytes(), rows, cols);
    simulate_bounded(input, rows, cols, r, c, STEPS)
}

fn find_start(grid: &[u8], rows: usize, cols: usize) -> (usize, usize) {
    for r in 0..rows {
        for c in 0..cols - 1 {
            if grid[r * cols + c] == b'S' {
                return (r, c);
            }
        }
    }

    unreachable!()
}

pub fn simulate_bounded(
    input: &str,
    rows: usize,
    cols: usize,
    start_r: usize,
    start_c: usize,
    steps: usize,
) -> usize {
    let input = input.as_bytes();

    let mut state = vec![false; input.len() * 2];
    let mut queue = VecDeque::with_capacity(4096);

    queue.push_back((start_r as isize, start_c as isize));
    for step in 1..steps + 1 {
        let is_odd = (step - 1) % 2;

        for _ in 0..queue.len() {
            let (r, c) = queue.pop_front().unwrap();

            for (dr, dc) in DIR {
                let r = r + dr;
                let c = c + dc;
                if r < 0 || c < 0 {
                    continue;
                }

                let r = r as usize;
                let c = c as usize;
                if r >= rows || c >= cols - 1 {
                    continue;
                }

                if input[r * cols + c] == b'#' {
                    continue;
                }

                let cache_key = is_odd * input.len() + r * cols + c;
                if !state[cache_key] {
                    state[cache_key] = true;
                    queue.push_back((r as isize, c as isize));
                }
            }
        }
    }

    let (from, to) = match steps % 2 == 0 {
        false => (0, input.len()),
        true => (input.len(), state.len()),
    };

    state[from..to].iter().copied().filter(|&x| x).count()
}

pub fn part_two(input: &str) -> i64 {
    const STEPS: u64 = 26_501_365;

    let cols = input.as_bytes().iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = input.len() / cols;

    // assert the input grid is a square
    // the `-1` is there because it compensates for the newline character `\n`
    assert_eq!(rows, cols - 1);

    // The problem statement states that we always start at the middle of the input
    let (start_r, start_c) = (rows / 2, rows / 2);

    // Coefficients for our lagrange polynomial: https://en.wikipedia.org/wiki/Lagrange_polynomial
    // * x0=0, y0=values[0]
    // * x1=1, y1=values[1]
    // * x2=2, y2=values[2]
    let mut values = [0; 3];
    simulate_unbounded(
        input,
        rows,
        cols,
        start_r,
        start_c,
        rows / 2 + 2 * rows,
        |step, positions| {
            // Assumptions based on my input:
            // * The start cell `S` is always at the center (i.e. `131 / 2 == 65`)
            // * the number of steps is not arbitrary: it is: 26_501_365 == (131 * 202300) + 65
            //   i.e. 202300 input widths + half width
            //
            // Take three measurements:
            // * half width
            // * half width + width
            // * half width + 2x width
            if step == rows / 2 {
                values[0] = positions as i64;
            } else if step == rows / 2 + rows {
                values[1] = positions as i64;
            } else if step == rows / 2 + 2 * rows {
                values[2] = positions as i64;
            }
        },
    );

    let (a, b, c) = simplified_lagrange(values[0], values[1], values[2]);

    // With the above coefficients, interpolate the value at x=202300
    let target = ((STEPS as i64 - rows as i64 / 2) / rows as i64) as f64;
    let answer = a * target.powi(2) + b * target + c;
    answer as i64
}

pub fn simulate_unbounded(
    input: &str,
    rows: usize,
    cols: usize,
    start_r: usize,
    start_c: usize,
    steps: usize,
    mut after_step: impl FnMut(usize, usize),
) {
    let input = input.as_bytes();

    // take advantage of the oscillating even/odd nature of the movement,
    // in order to always expand to new cells
    let mut visited_odd = HashSet::default();
    visited_odd.reserve(12 * 1024);

    let mut visited_even = HashSet::default();
    visited_even.reserve(12 * 1024);

    let mut queue = VecDeque::with_capacity(2 * 1024);
    queue.push_back((start_r as isize, start_c as isize));

    for step in 0..steps {
        let next = match step % 2 == 0 {
            true => &mut visited_odd,
            false => &mut visited_even,
        };

        for _ in 0..queue.len() {
            let (r, c) = queue.pop_front().unwrap();

            for (dr, dc) in DIR {
                let r = r + dr;
                let c = c + dc;

                let x = c.rem_euclid(cols as isize - 1) as usize;
                let y = r.rem_euclid(rows as isize) as usize;
                if input[y * cols + x] == b'#' {
                    continue;
                }

                if next.insert((r, c)) {
                    queue.push_back((r, c));
                }
            }
        }

        after_step(step + 1, next.len());
    }
}

fn simplified_lagrange(y0: i64, y1: i64, y2: i64) -> (f64, f64, f64) {
    let y0 = y0 as f64;
    let y1 = y1 as f64;
    let y2 = y2 as f64;
    /*
     Lagrange's Interpolation formula for `ax^2 + bx + c`
     with `x=[0,1,2]` and `y=[y0,y1,y2]` we have

     f(x) = (x^2-3x+2) * y0/2 - (x^2-2x)*y1 + (x^2-x) * y2/2

     so the coefficients are:
     a = y0/2 - y1 + y2/2
     b = -3*y0/2 + 2*y1 - y2/2
     c = y0

    https://en.wikipedia.org/wiki/Lagrange_polynomial
    */
    let a = y0 / 2.0 - y1 + y2 / 2.0;
    let b = -3.0 * y0 / 2.0 + 2.0 * y1 - y2 / 2.0;
    let c = y0;

    (a, b, c)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(3_776, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two(&input);
        assert_eq!(625_587_097_150_084, answer);
    }
}
