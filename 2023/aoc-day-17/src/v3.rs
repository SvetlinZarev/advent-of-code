use crate::common::Direction;

struct BucketQueue<T> {
    buckets: Vec<Vec<T>>,
    min_cost: usize,
}

impl<T> BucketQueue<T> {
    pub fn new(buckets: usize) -> Self {
        return Self {
            buckets: Vec::from_iter((0..buckets).map(|_| vec![])),
            min_cost: buckets,
        };
    }

    pub fn push(&mut self, cost: usize, data: T) {
        self.buckets[cost].push(data);
        self.min_cost = self.min_cost.min(cost);
    }

    pub fn pop(&mut self) -> Option<T> {
        for idx in self.min_cost..self.buckets.len() {
            if !self.buckets[idx].is_empty() {
                self.min_cost = idx;
                return self.buckets[idx].pop();
            }
        }

        None
    }
}

pub fn part_one(input: &[u8]) -> u16 {
    const MAX_STEPS: usize = 3;
    const SKIP_STEPS: usize = 0;
    const LEN: usize = MAX_STEPS - SKIP_STEPS;

    dijkstra::<SKIP_STEPS, MAX_STEPS, LEN>(input, &[Direction::Right, Direction::Down])
}

pub fn part_two(input: &[u8]) -> u16 {
    const MAX_STEPS: usize = 10;
    const SKIP_STEPS: usize = 3;
    const LEN: usize = MAX_STEPS - SKIP_STEPS;
    dijkstra::<SKIP_STEPS, MAX_STEPS, LEN>(input, &[Direction::Right, Direction::Down])
}

pub fn dijkstra<const SKIP: usize, const STEPS: usize, const LEN: usize>(
    grid: &[u8],
    initial_dir: &[Direction],
) -> u16 {
    let cols = grid.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = grid.len() / cols;

    let seen_capacity = rows * cols * 2 / (usize::BITS as usize) + (rows * cols * 2 % (usize::BITS as usize) != 0) as usize;
    let mut seen = vec![0usize; seen_capacity];
    let mut queue = BucketQueue::new((rows + cols) * 9);

    // Seed the queue with the starting elements
    for d in initial_dir.iter().copied() {
        let (mut r, mut c, mut loss) = step(grid, rows, cols, 0, 0, d, SKIP).unwrap();

        for _ in SKIP..STEPS {
            let Some((nr, nc, cost)) = step(grid, rows, cols, r, c, d, 1) else {
                break;
            };

            r = nr;
            c = nc;
            loss += cost;

            queue.push(loss as usize, (loss, r, c, d));
        }
    }

    while let Some((loss, r, c, d)) = queue.pop() {
        if (r, c) == (rows - 1, cols - 2) {
            return loss;
        }

        let (cell, bit) = cache_idx(rows, cols, r, c, d);
        if seen[cell] & (1 << bit) != 0 {
            continue;
        }
        seen[cell] |= 1 << bit;

        for d in [d.rotl(), d.rotr()] {
            let Some((mut r, mut c, mut cost)) = step(grid, rows, cols, r, c, d, SKIP) else {
                continue;
            };
            cost += loss;

            for _ in SKIP..STEPS {
                let Some((nr, nc, cst)) = step(grid, rows, cols, r, c, d, 1) else {
                    break;
                };

                r = nr;
                c = nc;
                cost += cst;

                let (cell, bit) = cache_idx(rows, cols, r, c, d);
                if seen[cell] & (1 << bit) == 0 {
                    queue.push(cost as usize, (cost, r, c, d));
                }
            }
        }
    }

    unreachable!()
}

#[inline(always)]
fn cache_idx(rows: usize, cols: usize, r: usize, c: usize, d: Direction) -> (usize, usize) {
    let key = d.vertical() as usize * rows * cols + r * cols + c;
    let cell = key / usize::BITS as usize;
    let bit = key % usize::BITS as usize;

    (cell, bit)
}

#[inline(always)]
fn step(
    input: &[u8],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    d: Direction,
    s: usize,
) -> Option<(usize, usize, u16)> {
    let (mut rx, mut cx) = (r as isize, c as isize);
    let mut cost = 0;

    for _ in 0..s {
        let (nr, nc) = d.apply_signed(rx, cx);
        if nr < 0 || nc < 0 || nr >= rows as isize || nc >= (cols - 1) as isize {
            return None;
        }

        (rx, cx) = (nr, nc);
        cost += (input[rx as usize * cols + cx as usize] - b'0') as u16;
    }

    Some((rx as usize, cx as usize, cost))
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