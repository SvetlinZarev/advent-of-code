use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

use aoc_shared::hashing::HashSet;

// Explore only paths that are at least as good as the best X paths so far
const BEAM_WIDTH: usize = 1000;

const ROUNDS: u32 = 26;

pub fn part_two(graph: &[(u16, Vec<usize>)]) -> u16 {
    assert!(graph.len() <= 64);

    let mut queue = VecDeque::new();
    queue.push_back((0u64, 0, 0, 0, 0));

    let mut seen = HashSet::default();
    seen.insert((0, 0, 0, 0));

    let mut beam = BinaryHeap::new();

    let mut answer = 0;
    for round in 0..ROUNDS {
        for _ in 0..queue.len() {
            let (opened, me, el, released, rate) = queue.pop_front().unwrap();
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

            // If we stay at the same position, It's always me that will open the valve,
            // and the elephant will go ahead, because the two cases are equivalent
            let me_open = opened & (1 << me) == 0 && graph[me].0 > 0;
            let el_open = opened & (1 << el) == 0 && graph[el].0 > 0 && me != el;

            match (me_open, el_open) {
                (true, true) => {
                    // We'll bot open (different) valves
                    {
                        let next_opened = opened | (1 << me) | (1 << el);
                        let next_rate = rate + graph[me].0 + graph[el].0;

                        if seen.insert((next_opened, me, el, next_released)) {
                            queue.push_back((next_opened, me, el, next_released, next_rate));
                        }
                    }

                    //I'll open and the elephant will not
                    {
                        let next_opened = opened | 1 << me;
                        let next_rate = rate + graph[me].0;

                        for next_el in graph[el].1.iter().copied() {
                            if seen.insert((next_opened, me, next_el, next_released)) {
                                queue.push_back((
                                    next_opened,
                                    me,
                                    next_el,
                                    next_released,
                                    next_rate,
                                ));
                            }
                        }
                    }

                    // The elephant will open and I'll not
                    {
                        let next_opened = opened | 1 << el;
                        let next_rate = rate + graph[el].0;

                        for next_me in graph[me].1.iter().copied() {
                            if seen.insert((next_opened, next_me, el, next_released)) {
                                queue.push_back((
                                    next_opened,
                                    next_me,
                                    el,
                                    next_released,
                                    next_rate,
                                ));
                            }
                        }
                    }
                }

                // I'll open the valve, or if we stand at the same valve, then I'll open it
                (true, false) => {
                    let next_opened = opened | 1 << me;
                    let next_rate = rate + graph[me].0;

                    for next_el in graph[el].1.iter().copied() {
                        if seen.insert((next_opened, me, next_el, next_released)) {
                            queue.push_back((next_opened, me, next_el, next_released, next_rate));
                        }
                    }
                }

                // The elephant will open its valve
                (false, true) => {
                    let next_opened = opened | 1 << el;
                    let next_rate = rate + graph[el].0;

                    for next_me in graph[me].1.iter().copied() {
                        if seen.insert((next_opened, next_me, el, next_released)) {
                            queue.push_back((next_opened, next_me, el, next_released, next_rate));
                        }
                    }
                }

                (false, false) => {}
            }

            // We'll both move without opening any valves
            for next_me in graph[me].1.iter().copied() {
                for next_el in graph[el].1.iter().copied() {
                    if seen.insert((opened, next_me, next_el, next_released)) {
                        queue.push_back((opened, next_me, next_el, next_released, rate));
                    }
                }
            }
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::p2v2::part_two;
    use crate::parse_input;

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let graph = parse_input(input);
        let answer = part_two(&graph);
        assert_eq!(2052, answer);
    }
}
