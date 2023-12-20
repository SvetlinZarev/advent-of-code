use std::collections::VecDeque;

use aoc_shared::hashing::FxHashMap;
use num_integer::Integer;

type HashMap<K, V> = FxHashMap<K, V>;

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

pub fn load_graph(input: &str) -> HashMap<&str, (Kind, Vec<&str>)> {
    let mut graph = HashMap::default();

    for line in input.lines() {
        let mut nodes = vec![];
        let (src, rest) = line.split_once(' ').unwrap();

        for next in rest[3..].split(',') {
            nodes.push(next.trim());
        }

        let (kind, skip) = match src.as_bytes()[0] {
            b'%' => (Kind::FlipFlop, 1),
            b'&' => (Kind::Conjunction, 1),
            b'b' => (Kind::Broadcaster, 0),
            _ => unreachable!(),
        };

        graph.insert(&src[skip..], (kind, nodes));
    }

    graph
}

pub fn part_one(graph: &HashMap<&str, (Kind, Vec<&str>)>) -> u64 {
    let (mut conjunctions, _) = init_conjugation_state(graph);
    let mut flip_flops = HashMap::default();
    let mut queue = VecDeque::new();

    let mut lows = 0u64;
    let mut highs = 0u64;

    for _ in 0..1_000 {
        queue.push_back(("", START_NODE, Pulse::Low));

        while let Some((src, dst, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::Low => lows += 1,
                Pulse::High => highs += 1,
            }

            if FINAL_NODE == dst {
                continue;
            }

            handle_node(
                graph,
                &mut conjunctions,
                &mut flip_flops,
                &mut queue,
                src,
                dst,
                pulse,
            );
        }
    }

    lows * highs
}

pub fn part_two(graph: &HashMap<&str, (Kind, Vec<&str>)>) -> u64 {
    let (mut conjunctions, final_node) = init_conjugation_state(graph);
    let mut flip_flops = HashMap::default();
    let mut queue = VecDeque::new();

    // find the starting node of each sub-component
    let Some((_, start_nodes)) = &graph.get(START_NODE) else {
        panic!("Cannot find the Broadcast node in the graph");
    };

    let mut answer = 1;
    'next_node: for &start_node in start_nodes {
        let mut button_presses = 0;

        loop {
            button_presses += 1;
            queue.push_back((START_NODE, start_node, Pulse::Low));

            while let Some((src, dst, pulse)) = queue.pop_front() {
                if final_node == dst {
                    if pulse == Pulse::High {
                        answer = answer.lcm(&button_presses);
                        queue.clear();
                        continue 'next_node;
                    }

                    continue;
                }

                handle_node(
                    graph,
                    &mut conjunctions,
                    &mut flip_flops,
                    &mut queue,
                    src,
                    dst,
                    pulse,
                );
            }
        }
    }

    answer
}

#[inline(always)]
fn handle_node<'l>(
    graph: &HashMap<&'l str, (Kind, Vec<&'l str>)>,
    conjunctions: &mut HashMap<&'l str, HashMap<&'l str, State>>,
    flip_flops: &mut HashMap<&'l str, State>,
    queue: &mut VecDeque<(&'l str, &'l str, Pulse)>,
    src: &'l str,
    dst: &'l str,
    pulse: Pulse,
) {
    let Some((kind, next)) = graph.get(dst) else {
        panic!("Cannot find node {:?} in the graph", dst);
    };

    let output = match kind {
        Kind::Broadcaster => pulse,

        Kind::FlipFlop => {
            if Pulse::High == pulse {
                return;
            }

            let state = flip_flops.entry(dst).or_insert(State::Low);
            *state = state.invert();

            state.into_pulse()
        }

        Kind::Conjunction => {
            let Some(inputs) = conjunctions.get_mut(dst) else {
                panic!("Cannot find conjugation state for {:?}", dst);
            };

            let Some(state) = inputs.get_mut(src) else {
                panic!("Cannot find input {:?} for conjugation {:?}", src, dst);
            };

            *state = pulse.into_state();
            match inputs.values().all(|&x| x == State::High) {
                true => Pulse::Low,
                false => Pulse::High,
            }
        }
    };

    for node in next.iter().copied() {
        queue.push_back((dst, node, output));
    }
}

fn init_conjugation_state<'l>(
    graph: &HashMap<&'l str, (Kind, Vec<&'l str>)>,
) -> (HashMap<&'l str, HashMap<&'l str, State>>, &'l str) {
    let mut conjunctions = HashMap::default();
    let mut final_conjugation = "";

    for (&current, (_, next)) in graph.iter() {
        for node in next.iter().copied() {
            if FINAL_NODE == node {
                final_conjugation = current;
                continue;
            }

            let Some((kind, _)) = graph.get(node) else {
                panic!("Cannot find node {:?} in the graph", node);
            };

            if Kind::Conjunction == *kind {
                conjunctions
                    .entry(node)
                    .or_insert(HashMap::default())
                    .insert(current, State::Low);
            }
        }
    }

    assert_ne!(final_conjugation, "");
    (conjunctions, final_conjugation)
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
