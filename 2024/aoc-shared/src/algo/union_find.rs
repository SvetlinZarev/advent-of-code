use std::hash::Hash;

use crate::hashing::FxHashMap;
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

    pub fn group_size(&self, group: usize) -> usize {
        self.sizes[group]
    }

    pub fn number_of_groups(&self) -> usize {
        self.groups
    }
}

pub struct UnionFindAny<T> {
    parents: FxHashMap<T, T>,
    sizes: FxHashMap<T, usize>,
}

impl<T> UnionFindAny<T>
where
    T: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            parents: FxHashMap::default(),
            sizes: FxHashMap::default(),
        }
    }

    pub fn reset(&mut self) {
        self.parents.clear();
        self.sizes.clear();
    }

    pub fn find(&mut self, key: T) -> T {
        match self.parents.get(&key) {
            None => key,
            Some(&parent) => {
                let parent = self.find(parent);
                self.parents.insert(key, parent);
                parent
            }
        }
    }

    pub fn union(&mut self, a: T, b: T) -> bool {
        let x = self.find(a);
        let y = self.find(b);

        // A and B are already in the same set -> nothing to do
        if x == y {
            return false;
        }

        let x_size = self.sizes.get(&x).copied().unwrap_or(1);
        let y_size = self.sizes.get(&y).copied().unwrap_or(1);

        if x_size >= y_size {
            *self.sizes.entry(x).or_insert(1) += y_size;
            self.parents.insert(y, x);
        } else {
            *self.sizes.entry(y).or_insert(1) += x_size;
            self.parents.insert(x, y);
        }

        true
    }

    pub fn group_size(&self, group_root: &T) -> usize {
        self.sizes.get(&group_root).copied().unwrap_or(1)
    }

    pub fn find_size(&mut self, key: T) -> usize {
        let group_root = self.find(key);
        self.group_size(&group_root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uf_any() {
        let mut uf = UnionFindAny::new();

        let a = uf.find("x");
        let b = uf.find("y");
        assert_ne!(a, b);

        let ok = uf.union("x", "y");
        assert!(ok);

        let a = uf.find("x");
        let b = uf.find("y");
        assert_eq!(a, b);
    }

    #[test]
    fn test_uf_any_group_size() {
        let mut uf = UnionFindAny::new();

        let size = uf.find_size("x");
        assert_eq!(1, size);

        uf.union("x", "y");
        let size = uf.find_size("x");
        assert_eq!(2, size);
    }
}
