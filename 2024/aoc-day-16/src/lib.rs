use aoc_shared::grid::Direction;
use aoc_shared::util::BitSet;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

const WALL: u8 = b'#';
const STEP_COST: u32 = 1;
const TURN_COST: u32 = 1000;
const TURN_STEP_COST: u32 = 1001;

const DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Right,
    Direction::Up,
    Direction::Down,
];

pub fn part_one_v1(input: &str) -> u32 {
    let (rows, cols, start_r, start_c, end_r, end_c) = info(input);
    let grid = input.as_bytes();

    let mut visited = vec![u32::MAX; grid.len() * 4];
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), start_r, start_c, Direction::Right));

    while let Some((Reverse(score), r, c, d)) = pq.pop() {
        if (r, c) == (end_r, end_c) {
            return score;
        }

        let directions = [(d, 1), (d.rotl(), 1001), (d.rotr(), 1001)];

        for (h, s) in directions {
            let (y, x) = h.apply_signed(r, c);
            if !(0..rows).contains(&y) {
                continue;
            }
            if !(0..cols - 1).contains(&x) {
                continue;
            }

            let pos = (y * cols + x) as usize;
            if grid[pos] == WALL {
                continue;
            }

            let ss = score + s;
            if visited[pos * 4 + h as usize] <= ss {
                continue;
            }

            visited[pos * 4 + h as usize] = ss;
            pq.push((Reverse(ss), y, x, h));
        }
    }

    unreachable!()
}

// The save as Part-2/V2 adapted for Part-1
pub fn part_one_v2(input: &str) -> u32 {
    let (rows, cols, start_r, start_c, end_r, end_c) = info(input);
    let grid = input.as_bytes();

    let mut costs = vec![u32::MAX; grid.len() * 4];
    costs[(start_r * cols + start_c) as usize * 4 + Direction::Right as usize] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((0, start_r, start_c, Direction::Right));

    let mut min_cost = u32::MAX;
    while let Some((cost, r, c, d)) = queue.pop_front() {
        if (r, c) == (end_r, end_c) {
            min_cost = min_cost.min(cost);
        }

        for h in [d.rotl(), d.rotr()] {
            let cost_idx = (r * cols + c) as usize * 4 + h as usize;
            let next_cost = cost + TURN_COST;

            if next_cost < min_cost && costs[cost_idx] > next_cost {
                costs[cost_idx] = next_cost;
                queue.push_back((next_cost, r, c, h));
            }
        }

        let (y, x) = d.apply_signed(r, c);
        if !(0..rows).contains(&y) {
            continue;
        }
        if !(0..cols - 1).contains(&x) {
            continue;
        }
        if grid[(y * cols + x) as usize] == WALL {
            continue;
        }

        let cost_idx = (y * cols + x) as usize * 4 + d as usize;
        let next_cost = cost + STEP_COST;

        if next_cost <= min_cost && costs[cost_idx] > next_cost {
            costs[cost_idx] = next_cost;
            queue.push_front((next_cost, y, x, d));
        }
    }

    min_cost
}

pub fn part_two_v1(input: &str) -> u32 {
    let (rows, cols, start_r, start_c, end_r, end_c) = info(input);
    let grid = input.as_bytes();

    let mut costs = vec![vec![vec![u32::MAX; cols as usize]; rows as usize]; 4];
    costs[Direction::Right as usize][start_r as usize][start_c as usize] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((0, start_r, start_c, Direction::Right));

    let mut min_cost = u32::MAX;
    while let Some((cost, r, c, d)) = queue.pop_front() {
        if (r, c) == (end_r, end_c) {
            min_cost = min_cost.min(cost);
        }

        for h in [d.rotl(), d.rotr()] {
            if costs[h as usize][r as usize][c as usize] > cost + 1000 {
                costs[h as usize][r as usize][c as usize] = cost + 1000;
                queue.push_back((cost + 1000, r, c, h));
            }
        }

        let (y, x) = d.apply_signed(r, c);
        if !(0..rows).contains(&y) {
            continue;
        }
        if !(0..cols - 1).contains(&x) {
            continue;
        }
        if grid[(y * cols + x) as usize] == b'#' {
            continue;
        }

        if costs[d as usize][y as usize][x as usize] > cost + 1 {
            costs[d as usize][y as usize][x as usize] = cost + 1;
            queue.push_front((cost + 1, y, x, d));
        }
    }

    let mut cells = BitSet::new(grid.len());
    let mut queue = VecDeque::new();
    for d in DIRECTIONS {
        if costs[d as usize][end_r as usize][end_c as usize] == min_cost {
            queue.push_back((min_cost, end_r, end_c, d));
        }
    }

    while let Some((cost, r, c, d)) = queue.pop_front() {
        cells.set((r * cols + c) as usize);

        let rev = d.rotr().rotr();
        let (y, x) = rev.apply_signed(r, c);
        if (0..rows).contains(&y) && (0..cols - 1).contains(&x) {
            // one step back
            if Some(costs[d as usize][y as usize][x as usize]) == cost.checked_sub(1) {
                queue.push_back((cost - 1, y, x, d));
            }

            for rot in [d.rotl(), d.rotr()] {
                if Some(costs[rot as usize][y as usize][x as usize]) == cost.checked_sub(1001) {
                    queue.push_back((cost - 1001, y, x, rot));
                }
            }
        }
    }

    cells.count_ones() as u32
}

// The same algorithm as V1, but optimized:
// * using a 1-dimensional COSTS vector
// * push_front() the cells that are in a straight line
// * push_back() the cells that turn right/left
// * don't push in the queue if it will exceed the best known `min_cost`
pub fn part_two_v2(input: &str) -> u32 {
    let (rows, cols, start_r, start_c, end_r, end_c) = info(input);
    let grid = input.as_bytes();

    let mut costs = vec![u32::MAX; grid.len() * 4];
    costs[(start_r * cols + start_c) as usize * 4 + Direction::Right as usize] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((0, start_r, start_c, Direction::Right));

    let mut min_cost = u32::MAX;
    while let Some((cost, r, c, d)) = queue.pop_front() {
        if (r, c) == (end_r, end_c) {
            min_cost = min_cost.min(cost);
            continue;
        }

        for h in [d.rotl(), d.rotr()] {
            let cost_idx = (r * cols + c) as usize * 4 + h as usize;
            let next_cost = cost + TURN_COST;

            if next_cost < min_cost && costs[cost_idx] > next_cost {
                costs[cost_idx] = next_cost;
                queue.push_back((next_cost, r, c, h));
            }
        }

        let (y, x) = d.apply_signed(r, c);
        if !(0..rows).contains(&y) {
            continue;
        }
        if !(0..cols - 1).contains(&x) {
            continue;
        }
        if grid[(y * cols + x) as usize] == WALL {
            continue;
        }

        let cost_idx = (y * cols + x) as usize * 4 + d as usize;
        let next_cost = cost + STEP_COST;

        if next_cost <= min_cost && costs[cost_idx] > next_cost {
            costs[cost_idx] = next_cost;
            queue.push_front((next_cost, y, x, d));
        }
    }

    let mut cells = BitSet::new(grid.len());
    let mut stack = vec![];

    // find out from which directions we've managed to reach the
    // target, and seed the trace-back stack/queue with them,
    // in order to trace those paths back to the start
    let end_cost_offset = (end_r * cols + end_c) as usize * 4;
    for d in DIRECTIONS {
        if costs[end_cost_offset + d as usize] == min_cost {
            stack.push((min_cost, end_r, end_c, d));
        }
    }

    while let Some((cost, r, c, d)) = stack.pop() {
        cells.set((r * cols + c) as usize);

        let rev = d.rotr().rotr();
        let (y, x) = rev.apply_signed(r, c);
        if (0..rows).contains(&y) && (0..cols - 1).contains(&x) {
            let cost_idx_offset = (y * cols + x) as usize * 4;

            // one step back
            if Some(costs[cost_idx_offset + d as usize]) == cost.checked_sub(1) {
                stack.push((cost - 1, y, x, d));
            }

            // and the 2 left/right neighbours
            for rot in [d.rotl(), d.rotr()] {
                if Some(costs[cost_idx_offset + rot as usize]) == cost.checked_sub(TURN_STEP_COST) {
                    stack.push((cost - TURN_STEP_COST, y, x, rot));
                }
            }
        }
    }

    cells.count_ones() as u32
}

fn info(input: &str) -> (isize, isize, isize, isize, isize, isize) {
    let input = input.as_bytes();

    let (mut rows, mut cols, mut start_r, mut start_c, mut end_r, mut end_c) = (0, 0, 0, 0, 0, 0);

    for (idx, &ch) in input.iter().enumerate() {
        match ch {
            b'\n' => {
                rows += 1;
                if cols == 0 {
                    cols = idx + 1;
                }
            }

            b'S' => {
                start_r = rows;
                start_c = if cols == 0 { idx } else { idx % cols };
            }

            b'E' => {
                end_r = rows;
                end_c = if cols == 0 { idx } else { idx % cols };
            }

            _ => {}
        }
    }

    (
        rows,
        cols as isize,
        start_r,
        start_c as isize,
        end_r,
        end_c as isize,
    )
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one_v1() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one_v1(&input);
        assert_eq!(105496, answer);
    }

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one_v2(&input);
        assert_eq!(105496, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_v1(&input);
        assert_eq!(524, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_v2(&input);
        assert_eq!(524, answer);
    }
}
