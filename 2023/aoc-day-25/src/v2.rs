use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

use aoc_shared::util::BitSet;

use crate::{count_reachable, parse_input};

/*
   Run a BFS starting from each node and increment a counter for each edge once we cross it.
   The edges, that split the graph in 2 should have the highest count, as they bear the load
   every time we cross from the first sub-graph the second.
*/

pub fn part_one(input: &str) -> usize {
    let (mut graph, _) = parse_input(input);

    let mut seen = BitSet::new(graph.len());
    let mut freq = vec![0u16; graph.len().pow(2)];
    let mut queue = VecDeque::new();

    // Note: adjust how many nodes to try. Trying all nodes will give the
    // correct result, but will be slower. Trying 1/50 of the nodes yields
    // a correct answer for my input
    for node in (0..graph.len()).step_by(50) {
        queue.push_back(node);

        seen.clear();
        seen.set(node);

        while let Some(node) = queue.pop_front() {
            for next in graph[node].iter().copied() {
                if seen.mark(next) {
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
