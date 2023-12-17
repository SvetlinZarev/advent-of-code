use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub fn apply(self, r: usize, c: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => r.checked_sub(1).and_then(|r| Some((r, c))),
            Direction::Down => Some((r + 1, c)),
            Direction::Left => c.checked_sub(1).and_then(|c| Some((r, c))),
            Direction::Right => Some((r, c + 1)),
        }
    }

    pub fn rotr(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn rotl(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn vertical(self) -> bool {
        match self {
            Direction::Up => true,
            Direction::Down => true,
            Direction::Left => false,
            Direction::Right => false,
        }
    }
}

pub fn part_one(input: &[u8]) -> u32 {
    const MAX_STEPS: usize = 3;
    const SKIP_STEPS: usize = 0;
    const LEN: usize = MAX_STEPS - SKIP_STEPS;

    dijkstra::<SKIP_STEPS, MAX_STEPS, LEN>(input, &[Direction::Right, Direction::Down], 64)
}

pub fn part_two(input: &[u8]) -> u32 {
    const MAX_STEPS: usize = 10;
    const SKIP_STEPS: usize = 3;
    const LEN: usize = MAX_STEPS - SKIP_STEPS;

    dijkstra::<SKIP_STEPS, MAX_STEPS, LEN>(input, &[Direction::Right], 34_000)
}

pub fn dijkstra<const SKIP: usize, const STEPS: usize, const LEN: usize>(
    grid: &[u8],
    initial_dir: &[Direction],
    queue_size: usize,
) -> u32 {
    let cols = grid.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = grid.len() / cols;

    let mut queue = BinaryHeap::with_capacity(queue_size);
    let mut seen: [[Vec<u32>; 2]; LEN] =
        std::array::from_fn(|_| [vec![u32::MAX; grid.len()], vec![u32::MAX; grid.len()]]);

    // Mark the starting cell as visited
    seen[LEN - 1][Direction::Right.vertical() as usize][0] = 0;

    // Seed the queue with the starting elements
    for d in initial_dir.iter().copied() {
        let (mut r, mut c, mut loss) = step(grid, rows, cols, 0, 0, d, SKIP).unwrap();

        for s in SKIP..STEPS {
            let Some((nr, nc)) = d.apply(r, c) else {
                break;
            };

            if nr >= rows || nc >= cols - 1 {
                break;
            }

            (r, c) = (nr, nc);
            loss += (grid[r * cols + c] - b'0') as u32;

            seen[s - SKIP][d.vertical() as usize][r * cols + c] = loss;
            queue.push((Reverse(loss), (r, c), d));
        }
    }

    while let Some((Reverse(loss), (r, c), d)) = queue.pop() {
        if (r, c) == (rows - 1, cols - 2) {
            return loss;
        }

        for d in [d.rotl(), d.rotr()] {
            let Some((mut r, mut c, mut cost)) = step(grid, rows, cols, r, c, d, SKIP) else {
                continue;
            };
            cost += loss;

            for s in SKIP..STEPS {
                let Some((nr, nc)) = d.apply(r, c) else {
                    break;
                };

                if nr >= rows || nc >= cols - 1 {
                    break;
                }
                (r, c) = (nr, nc);

                cost += (grid[r * cols + c] - b'0') as u32;
                if cost < seen[s - SKIP][d.vertical() as usize][r * cols + c] {
                    seen[s - SKIP][d.vertical() as usize][r * cols + c] = cost;
                    queue.push((Reverse(cost), (r, c), d));
                }
            }
        }
    }

    unreachable!()
}

fn step(
    input: &[u8],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    d: Direction,
    s: usize,
) -> Option<(usize, usize, u32)> {
    let (mut rx, mut cx) = (r, c);
    let mut cost = 0;

    for _ in 0..s {
        let Some((nr, nc)) = d.apply(rx, cx) else {
            return None;
        };

        if nr >= rows || nc >= cols - 1 {
            return None;
        }

        (rx, cx) = (nr, nc);
        cost += (input[rx * cols + cx] - b'0') as u32;
    }

    Some((rx, cx, cost))
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(input.as_bytes());
        assert_eq!(638, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two(input.as_bytes());
        assert_eq!(748, answer);
    }
}
