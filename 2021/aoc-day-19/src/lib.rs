use std::collections::{HashMap, HashSet};

use aoc_shared::hashing::{FnvHasher, HashBuilder};

pub use parsing::parse_input;
pub use part_one::part_one;
pub use part_two::part_two;

mod math;
mod parsing;
mod part_one;
mod part_two;
mod solver;

pub type Int = i32;
pub type Point = (Int, Int, Int);
type HashFnFactory = HashBuilder<FnvHasher>;
type Set<T> = HashSet<T, HashFnFactory>;
type Map<K, V> = HashMap<K, V, HashFnFactory>;
