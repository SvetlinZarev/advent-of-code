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
