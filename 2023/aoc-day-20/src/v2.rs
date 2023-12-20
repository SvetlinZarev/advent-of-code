use std::collections::VecDeque;

use aoc_shared::hashing::FxHashMap;
use num::Integer;

type HashMap<K, V> = FxHashMap<K, V>;

const START: usize = 0;
const END: usize = 1;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Kind {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn into_state(self) -> State {
        match self {
            Pulse::Low => State::Low,
            Pulse::High => State::High,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Low,
    High,
}

impl State {
    fn into_pulse(self) -> Pulse {
        match self {
            State::Low => Pulse::Low,
            State::High => Pulse::High,
        }
    }

    fn invert(self) -> Self {
        match self {
            State::Low => State::High,
            State::High => State::Low,
        }
    }
}

const START_NODE: &str = "broadcaster";
const FINAL_NODE: &str = "rx";

pub fn load_graph(input: &str) -> Vec<(Kind, Vec<usize>)> {
    let mut name_ids = HashMap::default();
    name_ids.reserve(64);
    name_ids.insert(START_NODE, START);
    name_ids.insert(FINAL_NODE, END);

    let mut graph = Vec::with_capacity(name_ids.len());

    for line in input.lines() {
        let mut nodes = vec![];
        let mut min_len = 0;

        let (src, rest) = line.split_once(' ').unwrap();
        for next in rest[3..].split(',').map(|n| n.trim()) {
            let mut node_id = name_ids.len();
            node_id = *name_ids.entry(next).or_insert(node_id);

            nodes.push(node_id);
            min_len = min_len.max(node_id);
        }

        let (kind, skip) = match src.as_bytes()[0] {
            b'%' => (Kind::FlipFlop, 1),
            b'&' => (Kind::Conjunction, 1),
            b'b' => (Kind::Broadcaster, 0),
            _ => unreachable!(),
        };

        let mut src_id = name_ids.len();
        src_id = *name_ids.entry(&src[skip..]).or_insert(src_id);
        min_len = min_len.max(src_id);

        if min_len >= graph.len() {
            graph.resize(min_len + 1, (Kind::Broadcaster, vec![]));
        }

        graph[src_id] = (kind, nodes);
    }

    graph
}

pub fn part_one(graph: &[(Kind, Vec<usize>)]) -> u64 {
    let mut ff_state = vec![State::Low; graph.len()];
    let mut cn_state = vec![vec![]; graph.len()];
    let mut queue = VecDeque::new();

    for node in 0..graph.len() {
        let (_, children) = &graph[node];

        for next in children.iter().copied() {
            if graph[next].0 == Kind::Conjunction {
                cn_state[next].push((node, State::Low));
            }
        }
    }

    let mut lows = 0;
    let mut highs = 0;

    for _ in 0..1000 {
        queue.push_back((START, START, Pulse::Low));

        while let Some((src, dst, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::Low => lows += 1,
                Pulse::High => highs += 1,
            }

            let (kind, next) = &graph[dst];
            let output = match kind {
                Kind::FlipFlop => match pulse {
                    Pulse::High => continue,
                    Pulse::Low => {
                        ff_state[dst] = ff_state[dst].invert();
                        ff_state[dst].into_pulse()
                    }
                },
                Kind::Conjunction => {
                    let inputs = &mut cn_state[dst];
                    let (_, state) = inputs.iter_mut().find(|(input, _)| *input == src).unwrap();
                    *state = pulse.into_state();

                    match inputs.iter().all(|&(_, state)| state == State::High) {
                        true => Pulse::Low,
                        false => Pulse::High,
                    }
                }
                Kind::Broadcaster => pulse,
            };

            for node in next.iter().copied() {
                queue.push_back((dst, node, output));
            }
        }
    }

    lows * highs
}

pub fn part_two(graph: &[(Kind, Vec<usize>)]) -> u64 {
    let mut answer = 1;

    let mut ff_state = vec![State::Low; graph.len()];
    let mut cn_state = vec![vec![]; graph.len()];
    let mut queue = VecDeque::new();

    for node in 0..graph.len() {
        let (_, children) = &graph[node];

        for next in children.iter().copied() {
            if graph[next].0 == Kind::Conjunction {
                cn_state[next].push((node, State::Low));
            }
        }
    }

    let start_nodes = &graph[START].1;
    let end_node = graph
        .iter()
        .position(|(_, next)| next.iter().any(|&x| x == END))
        .unwrap();

    for start_node in start_nodes.iter().copied() {
        let mut presses = 0;

        'cycle: loop {
            presses += 1;
            queue.push_back((START, start_node, Pulse::Low));

            while let Some((src, dst, pulse)) = queue.pop_front() {
                if dst == end_node {
                    match pulse {
                        Pulse::Low => continue,
                        Pulse::High => break 'cycle,
                    }
                }

                let (kind, next) = &graph[dst];
                let output = match kind {
                    Kind::FlipFlop => match pulse {
                        Pulse::High => continue,
                        Pulse::Low => {
                            ff_state[dst] = ff_state[dst].invert();
                            ff_state[dst].into_pulse()
                        }
                    },
                    Kind::Conjunction => {
                        let inputs = &mut cn_state[dst];
                        let (_, state) =
                            inputs.iter_mut().find(|(input, _)| *input == src).unwrap();
                        *state = pulse.into_state();

                        match inputs.iter().all(|&(_, state)| state == State::High) {
                            true => Pulse::Low,
                            false => Pulse::High,
                        }
                    }
                    Kind::Broadcaster => pulse,
                };

                for node in next.iter().copied() {
                    queue.push_back((dst, node, output));
                }
            }
        }

        queue.clear();
        answer = answer.lcm(&presses);
    }

    answer
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let graph = load_graph(&input);

        let answer = part_one(&graph);
        assert_eq!(763_500_168, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let graph = load_graph(&input);

        let answer = part_two(&graph);
        assert_eq!(207_652_583_562_007, answer);
    }
}
