use rand::prelude::*;
use rand::thread_rng;

use crate::parse_input;

const EXPECTED_CUTS: i32 = 3;

pub fn part_one(input: &str) -> usize {
    let (graph, mut connections) = parse_input(input);
    let mut uf = UnionFind::new(graph.len());

    loop {
        // shuffle in order to be able to easily choose a "random" edge
        connections.shuffle(&mut thread_rng());
        let mut edges = connections.as_slice();
        uf.reset();

        while uf.number_of_groups() > 2 {
            let (a, b) = edges[0];
            edges = &edges[1..];

            uf.union(a, b);
        }

        let mut cuts = 0;
        for &(a, b) in edges.iter() {
            if uf.find(a) != uf.find(b) {
                cuts += 1;
            }
        }

        if cuts == EXPECTED_CUTS {
            let root = uf.find(0);
            let size = uf.group_size(root);

            return (graph.len() - size) * size;
        }
    }
}

pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    groups: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            sizes: vec![1; size],
            parents: (0..size).collect(),
            groups: size,
        }
    }

    fn reset(&mut self) {
        self.groups = self.sizes.len();
        self.sizes.fill(1);
        self.parents
            .iter_mut()
            .enumerate()
            .for_each(|(idx, val)| *val = idx);
    }

    pub fn find(&mut self, key: usize) -> usize {
        if self.parents[key] == key {
            return key;
        }

        let parent = self.find(self.parents[key]);
        self.parents[key] = parent;
        parent
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let x = self.find(a);
        let y = self.find(b);

        // A and B are already in the same set -> nothing to do
        if x == y {
            return false;
        }

        let x_size = self.sizes[x];
        let y_size = self.sizes[y];

        if x_size >= y_size {
            self.sizes[x] += y_size;
            self.parents[y] = x;
        } else {
            self.sizes[y] += x_size;
            self.parents[x] = y;
        }

        self.groups -= 1;
        true
    }

    fn number_of_groups(&self) -> usize {
        self.groups
    }

    fn group_size(&self, group: usize) -> usize {
        self.sizes[group]
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(580_800, answer);
    }
}
