use aoc_shared::util::BitSet;

#[derive(Default, Clone)]
struct Node {
    children: [Option<Box<Node>>; 5],
    word: bool,
}

impl Node {
    pub fn insert(&mut self, word: &[u8]) {
        if word.is_empty() {
            self.word = true;
            return;
        }

        let idx = self.index(word[0]);
        let node = match &mut self.children[idx] {
            node @ None => node.insert(Box::new(Node::default())),
            Some(node) => node,
        };

        node.insert(&word[1..]);
    }

    fn index(&self, ch: u8) -> usize {
        (((ch as usize * 25) >> 3) + 7) % 8
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
        match &node.children[node.index(ch)] {
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

pub fn part_two(patterns: &[&str], lines: &[&str]) -> u64 {
    let mut trie = Node::default();

    patterns
        .into_iter()
        .for_each(|&pattern| trie.insert(pattern.as_bytes()));

    lines
        .into_iter()
        .map(|l| {
            let mut counts = vec![-1; l.len() + 1];
            contains_ways(&trie, l.as_bytes(), &mut counts)
        })
        .sum::<i64>() as u64
}

fn contains_ways(trie: &Node, word: &[u8], cache: &mut [i64]) -> i64 {
    let mut node = trie;
    let mut ways = 0;

    for (idx, &ch) in word.iter().enumerate() {
        match &node.children[node.index(ch)] {
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
                        contains_ways(trie, remaining, cache)
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
    use crate::array_based::{part_one, part_two};
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
