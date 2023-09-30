use std::collections::{HashSet, VecDeque};

use aoc_shared::hashing::{FnvHasher, HashBuilder};

const TARGET: (u64, u64) = (31, 39);
const DIR: &[(i64, i64)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Space {
    Open,
    Wall,
}

pub fn part_one(input: u64) -> u64 {
    let mut queue = VecDeque::new();
    queue.push_back((1u64, 1u64));

    let mut seen = HashSet::with_hasher(HashBuilder::<FnvHasher>::default());
    let mut steps = 0;

    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (x, y) = queue.pop_front().unwrap();

            for (dx, dy) in DIR.iter().copied() {
                let Some(x) = x.checked_add_signed(dx) else {
                    continue;
                };

                let Some(y) = y.checked_add_signed(dy) else {
                    continue;
                };

                if Space::Open == formula(x, y, input) {
                    if TARGET == (x, y) {
                        return steps + 1;
                    }

                    if seen.insert((x, y)) {
                        queue.push_back((x, y));
                    }
                }
            }
        }

        steps += 1;
    }

    unreachable!("no solution")
}

pub fn part_two(input: u64) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((1u64, 1u64));

    let mut seen = HashSet::with_hasher(HashBuilder::<FnvHasher>::default());
    seen.insert((1, 1));

    let mut steps = 0;
    while !queue.is_empty() && steps < 50 {
        for _ in 0..queue.len() {
            let (x, y) = queue.pop_front().unwrap();

            for (dx, dy) in DIR.iter().copied() {
                let Some(x) = x.checked_add_signed(dx) else {
                    continue;
                };

                let Some(y) = y.checked_add_signed(dy) else {
                    continue;
                };

                if Space::Open == formula(x, y, input) {
                    if seen.insert((x, y)) {
                        queue.push_back((x, y));
                    }
                }
            }
        }

        steps += 1;
    }

    seen.len()
}

fn formula(x: u64, y: u64, input: u64) -> Space {
    let value = x * x + 3 * x + 2 * x * y + y + y * y + input;
    match value.count_ones() % 2 == 0 {
        true => Space::Open,
        false => Space::Wall,
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = input.trim().parse().unwrap();

        let answer = part_one(input);
        assert_eq!(90, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = input.trim().parse().unwrap();

        let answer = part_two(input);
        assert_eq!(135, answer);
    }
}
