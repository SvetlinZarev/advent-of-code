// What type of integer to use for storing the index of the child nodes
type NodeInt = u32;

// My input leads to 795 trie nodes
const MAX_NODES: usize = 1000;

// My input does not have strings longer than 64
const MAX_LEN: usize = 64;

pub fn part_one(patterns: &[&str], towels: &[&str]) -> usize {
    let mut trie = [([0; 5], false); MAX_NODES];
    let mut node_ptr = 0;

    patterns
        .into_iter()
        .for_each(|&pattern| insert(&mut trie, &mut node_ptr, pattern.as_bytes()));

    towels
        .into_iter()
        .filter(|t| contains(&trie, t.as_bytes(), &mut 0)) // the negative cache is fit in a single u64
        .count()
}

fn contains(trie: &[([NodeInt; 5], bool)], word: &[u8], failed: &mut u64) -> bool {
    let mut node = 0;

    for (idx, &ch) in word.iter().enumerate() {
        let key = index(ch);

        let next = trie[node].0[key] as usize;
        if next == 0 {
            return false;
        }

        node = next;
        if trie[node].1 {
            let remaining = &word[idx + 1..];
            if remaining.is_empty() {
                return true;
            }

            let bit_idx = remaining.len() - 1;
            if *failed & (1 << bit_idx) != 0 {
                continue;
            }

            if contains(trie, remaining, failed) {
                return true;
            }

            *failed |= 1 << bit_idx;
        }
    }

    false
}

pub fn part_two(patterns: &[&str], towels: &[&str]) -> u64 {
    let mut trie = [([0; 5], false); MAX_NODES];
    let mut node_ptr = 0;

    patterns
        .into_iter()
        .for_each(|&pattern| insert(&mut trie, &mut node_ptr, pattern.as_bytes()));

    towels
        .into_iter()
        .map(|t| {
            let mut counts = [-1; MAX_LEN];
            count_ways(&trie, t.as_bytes(), &mut counts)
        })
        .sum::<i64>() as u64
}

fn count_ways(trie: &[([NodeInt; 5], bool)], word: &[u8], cache: &mut [i64]) -> i64 {
    let mut node = 0;
    let mut ways = 0;

    for (idx, &ch) in word.iter().enumerate() {
        let key = index(ch);

        let next = trie[node].0[key] as usize;
        if next == 0 {
            break;
        }

        node = next;
        if trie[node].1 {
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

    cache[word.len()] = ways;
    ways
}

fn insert(trie: &mut [([NodeInt; 5], bool)], last_node: &mut NodeInt, word: &[u8]) {
    let mut node = 0;

    for &ch in word {
        let key = index(ch);

        if trie[node].0[key] == 0 {
            *last_node += 1;
            trie[node].0[key] = *last_node;
        }

        node = trie[node].0[key] as usize;
    }

    trie[node].1 = true;
}

const fn index(ch: u8) -> usize {
    ((((ch as usize & 31) * 7) >> 4) + 1) % 8
}

#[cfg(test)]
mod tests {
    use crate::flat_array_based::{part_one, part_two};
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
