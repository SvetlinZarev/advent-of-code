use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

use crate::{count_reachable, parse_input};

pub fn part_one(input: &str) -> usize {
    let (mut graph, _) = parse_input(input);

    let seen_size =
        graph.len() / usize::BITS as usize + ((graph.len() % usize::BITS as usize) != 0) as usize;
    let mut seen = vec![0usize; seen_size];
    let mut freq = vec![0u16; graph.len().pow(2)];
    let mut queue = VecDeque::new();

    // Note: adjust how many nodes to try. Trying all nodes will give the
    // correct result, but will be slower. Trying 1/50 of the nodes yields
    // a correct answer for my input
    for node in (0..graph.len()).step_by(50) {
        queue.push_back(node);

        seen.fill(0);
        mark_seen(&mut seen, node);

        while let Some(node) = queue.pop_front() {
            for next in graph[node].iter().copied() {
                if mark_seen(&mut seen, next) {
                    let a = node.min(next);
                    let b = node.max(next);
                    freq[a * graph.len() + b] += 1;

                    queue.push_back(next);
                }
            }
        }
    }

    // Find the top-3 nodes to remove
    let mut pq = BinaryHeap::with_capacity(3);
    for i in 0..graph.len() {
        for j in i + 1..graph.len() {
            let f = freq[i * graph.len() + j];

            if pq.len() < pq.capacity() {
                pq.push((Reverse(f), i, j));
            } else {
                let (Reverse(x), ..) = pq.peek().copied().unwrap();
                if f > x {
                    pq.pop();
                    pq.push((Reverse(f), i, j));
                }
            }
        }
    }

    // disconnect the found edges
    while let Some((_, a, b)) = pq.pop() {
        graph[a].retain(|&x| x != b);
        graph[b].retain(|&x| x != a);
    }

    let set_a = count_reachable(&graph, 0);
    let set_b = graph.len() - set_a;
    return set_a * set_b;
}

fn mark_seen(seen: &mut [usize], n: usize) -> bool {
    let pos = n / usize::BITS as usize;
    let bit = n % usize::BITS as usize;

    if seen[pos] & (1 << bit) == 0 {
        seen[pos] |= 1 << bit;
        return true;
    }

    false
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
