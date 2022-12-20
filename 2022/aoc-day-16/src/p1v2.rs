use aoc_shared::hashing::HashSet;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

// Explore only paths that are at least as good as the best X paths so far
const BEAM_WIDTH: usize = 10;

const ROUNDS: u32 = 30;

pub fn part_one(graph: &[(u16, Vec<usize>)]) -> u16 {
    assert!(graph.len() <= 64);

    let mut queue = VecDeque::new();
    queue.push_back((0u64, 0, 0, 0));

    let mut seen = HashSet::default();
    seen.insert((0, 0, 0));

    let mut beam = BinaryHeap::new();

    let mut answer = 0;
    for round in 0..ROUNDS {
        for _ in 0..queue.len() {
            let (opened, position, released, rate) = queue.pop_front().unwrap();
            let next_released = released + rate;

            // Just drain the elements in the last round,
            // because will not use the newly generate ones
            if round == ROUNDS - 1 {
                answer = answer.max(next_released);
                continue;
            }

            if beam.len() < BEAM_WIDTH {
                beam.push(Reverse(next_released));
            } else {
                let smallest = beam.peek().unwrap().0;
                if next_released > smallest {
                    beam.pop();
                    beam.push(Reverse(next_released));
                } else if next_released < smallest {
                    // SKIP current path
                    continue;
                }
            }

            if opened & (1 << position) == 0 && graph[position].0 > 0 {
                let next_opened = opened | 1 << position;
                let next_rate = rate + graph[position].0;

                if seen.insert((next_opened, position, next_released)) {
                    queue.push_back((next_opened, position, next_released, next_rate));
                }
            }

            for next in graph[position].1.iter().copied() {
                if seen.insert((opened, next, next_released)) {
                    queue.push_back((opened, next, next_released, rate));
                }
            }
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use crate::p1v2::part_one;
    use crate::parse_input;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let graph = parse_input(input);
        let answer = part_one(&graph);
        assert_eq!(1584, answer);
    }
}
