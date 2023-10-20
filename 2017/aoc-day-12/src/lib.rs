use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_NUM: Regex = Regex::new(r#"\d+"#).unwrap();
}

pub fn part_one_and_two(input: &str) -> (usize, usize) {
    // Assume that the node IDs are consecutive integers and there are no gaps
    let mut uf = UnionFind::new(0);

    for line in input.lines() {
        let mut src = None;

        for node in REGEX_NUM.find_iter(line) {
            let x = node.as_str().parse::<usize>().unwrap();
            match src {
                None => src = Some(x),
                Some(src) => {
                    uf.union(src, x);
                }
            }
        }
    }

    let group = uf.find(0);
    (uf.group_size(group), uf.number_of_groups())
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

    pub fn find(&mut self, key: usize) -> usize {
        if self.parents[key] == key {
            return key;
        }

        let parent = self.find(self.parents[key]);
        self.parents[key] = parent;
        parent
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        self.expand(a.max(b) + 1);

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

    fn expand(&mut self, size: usize) {
        if size > self.parents.len() {
            let diff = size - self.parents.len();
            self.groups += diff;

            self.parents.extend((size - diff)..size);
            self.sizes.resize(self.parents.len(), 1);
        }
    }

    pub fn group_size(&self, group: usize) -> usize {
        self.sizes[group]
    }

    fn number_of_groups(&self) -> usize {
        self.groups
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one_and_two(&input);

        assert_eq!((113, 202), answer);
    }
}
