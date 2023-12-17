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
}

pub fn run(input: &str) -> i64 {
    part_one(input.as_bytes()) as i64
}

pub fn part_one(input: &[u8]) -> u32 {
    const MAX_STEPS: usize = 3;

    let cols = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = input.len() / cols;

    let mut seen = [
        vec![vec![u32::MAX; input.len()]; 4],
        vec![vec![u32::MAX; input.len()]; 4],
        vec![vec![u32::MAX; input.len()]; 4],
    ];
    seen[MAX_STEPS - 1][Direction::Right as usize][0 * cols + 0] = 0u32;

    let mut queue = BinaryHeap::with_capacity(64);
    queue.push((
        Reverse(0u32),
        Reverse(MAX_STEPS - 1),
        (0usize, 0usize),
        Direction::Right,
    ));

    while let Some((Reverse(loss), Reverse(steps), (r, c), d)) = queue.pop() {
        if (r, c) == (rows - 1, cols - 2) {
            return loss;
        }

        let left = d.rotl();
        let right = d.rotr();

        if let Some((nr, nc)) = d.apply(r, c) {
            if steps > 0 && nr < rows && nc < cols - 1 {
                let cost = loss + (input[nr * cols + nc] - b'0') as u32;
                if cost < seen[steps - 1][d as usize][nr * cols + nc] {
                    seen[steps - 1][d as usize][nr * cols + nc] = cost;
                    queue.push((Reverse(cost), Reverse(steps - 1), (nr, nc), d));
                }
            }
        }

        if let Some((nr, nc)) = left.apply(r, c) {
            if nr < rows && nc < cols - 1 {
                let cost = loss + (input[nr * cols + nc] - b'0') as u32;
                if cost < seen[2][left as usize][nr * cols + nc] {
                    seen[2][left as usize][nr * cols + nc] = cost;
                    queue.push((Reverse(cost), Reverse(MAX_STEPS - 1), (nr, nc), left));
                }
            }
        }

        if let Some((nr, nc)) = right.apply(r, c) {
            if nr < rows && nc < cols - 1 {
                let cost = loss + (input[nr * cols + nc] - b'0') as u32;
                if cost < seen[2][right as usize][nr * cols + nc] {
                    seen[2][right as usize][nr * cols + nc] = cost;
                    queue.push((Reverse(cost), Reverse(MAX_STEPS - 1), (nr, nc), right));
                }
            }
        }
    }

    unreachable!()
}

pub fn part_two(input: &[u8]) -> u32 {
    const MAX_STEPS: usize = 10;
    const MIN_STEPS: usize = 4;

    let cols = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = input.len() / cols;

    let mut queue = BinaryHeap::with_capacity(34_000);
    let mut seen = [
        vec![vec![u32::MAX; input.len()]; 4],
        vec![vec![u32::MAX; input.len()]; 4],
        vec![vec![u32::MAX; input.len()]; 4],
        vec![vec![u32::MAX; input.len()]; 4],
        vec![vec![u32::MAX; input.len()]; 4],
        vec![vec![u32::MAX; input.len()]; 4],
        vec![vec![u32::MAX; input.len()]; 4],
    ];

    let (mut ir, mut ic) = (0, 3);
    let mut init_loss = input[1] as u32 + input[2] as u32 + input[3] as u32 - 3 * b'0' as u32;

    for s in MIN_STEPS..=MAX_STEPS {
        let Some((nr, nc)) = Direction::Right.apply(ir, ic) else {
            continue;
        };

        if nr >= rows || nc >= cols - 1 {
            continue;
        }

        (ir, ic) = (nr, nc);
        init_loss += (input[ir * cols + ic] - b'0') as u32;

        seen[s - MIN_STEPS][Direction::Right as usize][ir * cols + ic] = init_loss;
        queue.push((Reverse(init_loss), (ir, ic), Direction::Right));
    }

    while let Some((Reverse(loss), (r, c), d)) = queue.pop() {
        if (r, c) == (rows - 1, cols - 2) {
            return loss;
        }

        for d in [d.rotl(), d.rotr()] {
            let Some((mut r, mut c, mut cost)) = follow(input, rows, cols, r, c, d, MIN_STEPS - 1)
            else {
                continue;
            };
            cost += loss;

            for s in MIN_STEPS..=MAX_STEPS {
                let Some((nr, nc)) = d.apply(r, c) else {
                    break;
                };

                if nr >= rows || nc >= cols - 1 {
                    break;
                }
                (r, c) = (nr, nc);

                cost += (input[r * cols + c] - b'0') as u32;
                if cost < seen[s - MIN_STEPS][d as usize][r * cols + c] {
                    seen[s - MIN_STEPS][d as usize][r * cols + c] = cost;
                    queue.push((Reverse(cost), (r, c), d));
                }
            }
        }
    }

    unreachable!()
}

fn follow(
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
