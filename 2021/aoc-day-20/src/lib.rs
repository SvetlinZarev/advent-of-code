use aoc_shared::hashing::{FnvHasher, HashBuilder};
use std::collections::{HashMap, HashSet};

mod parsing;
mod solver;

use crate::solver::solve;
pub use parsing::parse_input;

type HashFnFactory = HashBuilder<FnvHasher>;
pub type Int = usize;
pub type Map<K, V> = HashMap<K, V, HashFnFactory>;
pub type Set<T> = HashSet<T, HashFnFactory>;

pub fn part_one(alg: &[u8], img: &Set<(Int, Int)>, limits: (usize, usize)) -> usize {
    solve(alg, img, limits, 2)
}

pub fn part_two(alg: &[u8], img: &Set<(Int, Int)>, limits: (usize, usize)) -> usize {
    solve(alg, img, limits, 50)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let (alg, img, lim) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(&alg, &img, lim);
        assert_eq!(5316, answer);
    }

    #[test]
    fn test_part_two() {
        let (alg, img, lim) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(&alg, &img, lim);
        assert_eq!(16728, answer);
    }
}
