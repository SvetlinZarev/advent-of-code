use aoc_shared::hashing::{HashMap, HashSet};
use aoc_shared::parsing::parse_u8_grid;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const MASK_LEFT: u8 = 0b0000_1000;
const MASK_RIGHT: u8 = 0b0000_0100;
const MASK_UP: u8 = 0b0000_0010;
const MASK_DOWN: u8 = 0b0000_0001;
const MASK_WALL: u8 = 0b1000_0000;

const STEP: &[(isize, isize)] = &[(-1, 0), (0, -1), (0, 0), (0, 1), (1, 0)];

pub fn parse_input(input: impl AsRef<str>) -> (Vec<Vec<u8>>, usize, usize) {
    let mut grid = parse_u8_grid(input);
    let start = 1;
    let end = grid[0].len() - 2;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            grid[r][c] = match grid[r][c] {
                b'<' => MASK_LEFT,
                b'>' => MASK_RIGHT,
                b'^' => MASK_UP,
                b'v' => MASK_DOWN,
                b'#' => MASK_WALL,
                b'.' => {
                    // TODO: parse the actual start/end positions
                    0
                }
                _ => panic!("unexpected input at {}:{}", r, c),
            }
        }
    }

    (grid, start, end)
}

fn next_state(grid: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    assert!(grid.len() > 2);
    assert!(grid[0].len() > 2);

    let mut next = vec![vec![0; grid[0].len()]; grid.len()];
    let down = grid.len() - 2;
    let right = grid[0].len() - 2;

    for r in 0..grid.len() {
        assert_eq!(grid[r].len(), next[r].len());

        for c in 0..grid[r].len() {
            if grid[r][c] & MASK_WALL != 0 {
                next[r][c] = MASK_WALL;
                continue;
            }

            if grid[r][c] & MASK_LEFT != 0 {
                let col = if c == 1 { right } else { c - 1 };
                next[r][col] |= MASK_LEFT;
            }

            if grid[r][c] & MASK_RIGHT != 0 {
                let col = if c == right { 1 } else { c + 1 };
                next[r][col] |= MASK_RIGHT;
            }

            if grid[r][c] & MASK_UP != 0 {
                let row = if r == 1 { down } else { r - 1 };
                next[row][c] |= MASK_UP;
            }

            if grid[r][c] & MASK_DOWN != 0 {
                let row = if r == down { 1 } else { r + 1 };
                next[row][c] |= MASK_DOWN;
            }
        }
    }

    next
}

fn generate_all_states(initial: &Vec<Vec<u8>>) -> (Vec<Vec<Vec<u8>>>, usize) {
    let mut states = HashMap::default();
    let mut state = initial.to_vec();

    let cycle_start = loop {
        let next = next_state(&state);
        let order = states.len();

        let state_id = *states.entry(state).or_insert(order);
        if state_id != order {
            break state_id;
        }

        state = next;
    };

    let mut sequence = vec![vec![]; states.len()];
    for (state, idx) in states {
        sequence[idx] = state;
    }

    (sequence, cycle_start)
}

fn manhattan(from: (usize, usize), to: (usize, usize)) -> usize {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}

pub fn solve(
    states: &[Vec<Vec<u8>>],
    cycle_start: usize,
    start_time: usize,
    rs: usize,
    cs: usize,
    re: usize,
    ce: usize,
) -> usize {
    let cycle_len = states.len() - cycle_start;

    let mut seen = HashSet::default();
    let mut pq = BinaryHeap::new();
    pq.push((
        Reverse(manhattan((rs, cs), (re, ce))),
        Reverse(start_time),
        (rs, cs),
    ));

    while let Some((Reverse(_cost), Reverse(time), (r, c))) = pq.pop() {
        if r == re && c == ce {
            return time;
        }

        let state_idx = if time < cycle_start {
            time + 1
        } else {
            cycle_start + (time + 1 - cycle_start) % cycle_len
        };

        let state = &states[state_idx];

        for (dr, dc) in STEP.iter().copied() {
            let Some(rx) = r.checked_add_signed(dr) else {
                continue;
            };
            let Some(cx) = c.checked_add_signed(dc) else {
                continue;
            };
            if rx >= state.len() || cx > state[rx].len() {
                continue;
            }

            if state[rx][cx] == 0 && seen.insert((state_idx, rx, cx)) {
                let cost = manhattan((rx, cx), (re, ce)) + time + 1;
                pq.push((Reverse(cost), Reverse(time + 1), (rx, cx)));
            }
        }
    }

    panic!("no solution")
}

pub fn part_one(grid: &Vec<Vec<u8>>, start: usize, end: usize) -> usize {
    let (states, cycle_start) = generate_all_states(grid);
    solve(&states, cycle_start, 0, 0, start, grid.len() - 1, end)
}

pub fn part_two(grid: &Vec<Vec<u8>>, start: usize, end: usize) -> usize {
    let (states, cycle_start) = generate_all_states(grid);
    let fwd = solve(&states, cycle_start, 0, 0, start, grid.len() - 1, end);
    let bck = solve(&states, cycle_start, fwd, grid.len() - 1, end, 0, start);
    solve(&states, cycle_start, bck, 0, start, grid.len() - 1, end)
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_one, part_two};
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (grid, start, end) = parse_input(input);
        let answer = part_one(&grid, start, end);
        assert_eq!(290, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (grid, start, end) = parse_input(input);
        let answer = part_two(&grid, start, end);
        assert_eq!(842, answer);
    }
}
