use aoc_shared::grid::Direction;

pub struct BucketQueue<T> {
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

#[inline(always)]
pub fn step(
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
