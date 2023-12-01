use aoc_shared::hashing::{FnvHasher, HashBuilder};
use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    let mut has_two = 0;
    let mut has_three = 0;

    for line in input.lines() {
        let mut freq = [0; (b'z' - b'a' + 1) as usize];

        for &ch in line.as_bytes() {
            freq[(ch - b'a') as usize] += 1;
        }

        has_two += freq.iter().any(|&x| x == 2) as u32;
        has_three += freq.iter().any(|&x| x == 3) as u32;
    }

    has_two * has_three
}

pub fn part_two_bruteforce(input: &str) -> String {
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.sort_unstable();

    for (i, a) in input.lines().enumerate() {
        for (j, b) in input.lines().enumerate() {
            if i == j || a.len() != b.len() {
                continue;
            }

            let x = a.as_bytes();
            let y = b.as_bytes();

            let mut difs = 0;
            for k in 0..x.len() {
                difs += (x[k] != y[k]) as u32;
                if difs > 1 {
                    break;
                }
            }

            if difs == 1 {
                return x
                    .iter()
                    .zip(y.iter())
                    .filter(|&(&x, &y)| x == y)
                    .map(|(&x, _)| x as char)
                    .collect();
            }
        }
    }

    unreachable!("the input is crafted to have a solution")
}

pub fn part_two_trie(input: &str) -> String {
    let mut trie = Node::default();
    let mut buf = vec![];

    for line in input.lines() {
        if trie.contains_with_diff(line.as_bytes(), 1, &mut buf) {
            return String::from_utf8(buf).unwrap();
        }

        trie.insert(line.as_bytes());
    }

    unreachable!("the input is crafted to have a solution")
}

#[derive(Default)]
struct Node {
    children: HashMap<u8, Node, HashBuilder<FnvHasher>>,
    word: bool,
}

impl Node {
    fn insert(&mut self, val: &[u8]) {
        let mut node = self;
        for ch in val.iter().copied() {
            node = node.children.entry(ch).or_default();
        }
        node.word = true;
    }

    fn contains_with_diff(&self, val: &[u8], diff: u32, dst: &mut Vec<u8>) -> bool {
        if val.is_empty() {
            return self.word && diff == 0;
        }

        match self.children.get(&val[0]) {
            Some(next) => {
                dst.push(val[0]);
                if next.contains_with_diff(&val[1..], diff, dst) {
                    return true;
                }
                dst.pop();
            }

            None => {
                if diff > 0 {
                    for next in self.children.values() {
                        if next.contains_with_diff(&val[1..], diff - 1, dst) {
                            return true;
                        }
                    }
                }
            }
        }

        false
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
        assert_eq!(6696, answer);
    }

    #[test]
    fn test_part_two_bruteforce() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_bruteforce(&input);
        assert_eq!("bvnfawcnyoeyudzrpgslimtkj", answer);
    }

    #[test]
    fn test_part_two_trie() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_trie(&input);
        assert_eq!("bvnfawcnyoeyudzrpgslimtkj", answer);
    }
}
