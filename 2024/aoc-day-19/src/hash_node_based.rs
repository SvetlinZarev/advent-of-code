use aoc_shared::hashing::FxHashMap;
use aoc_shared::util::BitSet;

type HashMap<K, V> = FxHashMap<K, V>;

#[derive(Default, Debug, Clone)]
struct Node {
    child: HashMap<u8, Node>,
    word: bool,
}

impl Node {
    pub fn insert(&mut self, word: &[u8]) {
        let mut node = self;

        for &ch in word {
            node = node.child.entry(ch).or_default();
        }

        node.word = true;
    }
}

pub fn part_one(patterns: &[&str], lines: &[&str]) -> usize {
    let mut trie = Node::default();

    patterns
        .into_iter()
        .for_each(|&pattern| trie.insert(pattern.as_bytes()));

    lines
        .into_iter()
        .filter(|l| {
            let mut failed = BitSet::new(l.len());
            contains(&trie, l.as_bytes(), &mut failed)
        })
        .count()
}

fn contains(trie: &Node, word: &[u8], failed: &mut BitSet) -> bool {
    let mut node = trie;

    for (idx, &ch) in word.iter().enumerate() {
        match node.child.get(&ch) {
            None => return false,

            Some(next) => {
                node = next;

                if node.word {
                    let remaining = &word[idx + 1..];
                    if remaining.is_empty() {
                        return true;
                    }

                    if failed.is_set(remaining.len() - 1) {
                        continue;
                    }

                    if contains(trie, remaining, failed) {
                        return true;
                    }

                    failed.set(remaining.len() - 1);
                }
            }
        }
    }

    false
}

pub fn part_two(patterns: &[&str], towels: &[&str]) -> u64 {
    let mut trie = Node::default();

    patterns
        .into_iter()
        .for_each(|&pattern| trie.insert(pattern.as_bytes()));

    towels
        .into_iter()
        .map(|t| {
            let mut counts = vec![-1; t.len() + 1];
            count_ways(&trie, t.as_bytes(), &mut counts)
        })
        .sum::<i64>() as u64
}

fn count_ways(trie: &Node, word: &[u8], cache: &mut [i64]) -> i64 {
    let mut node = trie;
    let mut ways = 0;

    for (idx, &ch) in word.iter().enumerate() {
        match node.child.get(&ch) {
            None => break,

            Some(next) => {
                node = next;

                if node.word {
                    let remaining = &word[idx + 1..];
                    if remaining.is_empty() {
                        ways += 1;
                        break;
                    }

                    ways += if cache[remaining.len()] >= 0 {
                        cache[remaining.len()]
                    } else {
                        count_ways(trie, remaining, cache)
                    };
                }
            }
        }
    }

    cache[word.len()] = ways;
    ways
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;
    use crate::tests::{PART_1_ANSWER, PART_2_ANSWER};
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (patterns, lines) = parse_input(&input).unwrap();

        let answer = part_one(&patterns, &lines);
        assert_eq!(PART_1_ANSWER, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (patterns, lines) = parse_input(&input).unwrap();

        let answer = part_two(&patterns, &lines);
        assert_eq!(PART_2_ANSWER, answer);
    }
}