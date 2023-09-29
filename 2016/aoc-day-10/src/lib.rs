use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Destination {
    Bot(u32),
    Output(u32),
}


pub fn parse_input(input: &str) -> (u32, HashMap<u32, (Destination, Destination, VecDeque<u32>)>) {
    let input_to_bot = regex::Regex::new(r#"value (?<value>\d+) goes to bot (?<bot>\d+)"#).unwrap();
    let bot_to_dest = regex::Regex::new(r#"bot (?<src>\d+) gives low to ((bot (?<l_bot>\d+))|(output (?<l_out>\d+))) and high to ((bot (?<h_bot>\d+))|(output (?<h_out>\d+)))"#).unwrap();

    let mut initial_chips = HashMap::new();
    let mut graph = HashMap::new();

    for line in input.lines() {
        if let Some(cap) = bot_to_dest.captures(line) {
            let src = cap.name("src").unwrap().as_str().parse::<u32>().unwrap();

            let low = if let Some(low) = cap.name("l_bot") {
                Destination::Bot(low.as_str().parse().unwrap())
            } else if let Some(low) = cap.name("l_out") {
                Destination::Output(low.as_str().parse().unwrap())
            } else {
                panic!("invalid input: {}", line);
            };

            let high = if let Some(high) = cap.name("h_bot") {
                Destination::Bot(high.as_str().parse().unwrap())
            } else if let Some(high) = cap.name("h_out") {
                Destination::Output(high.as_str().parse().unwrap())
            } else {
                panic!("invalid input: {}", line);
            };

            if let Some(prev) = graph.insert(src, (low, high, VecDeque::new())) {
                panic!("node {} has 2 output paths: {:?} & {:?}", src, prev, (low, high));
            }
        } else if let Some(cap) = input_to_bot.captures(line) {
            let value = cap.name("value").unwrap().as_str().parse::<u32>().unwrap();
            let bot = cap.name("bot").unwrap().as_str().parse::<u32>().unwrap();

            initial_chips.entry(bot).or_insert(vec![]).push(value);
        } else {
            panic!("invalid input: {}", line);
        }
    }

    let mut start_node = None;
    for (node, (_, _, init)) in graph.iter_mut() {
        if let Some(initial) = initial_chips.remove(node) {
            init.extend(initial);
            if init.len() == 2 {
                if let Some(prev_start) = start_node {
                    panic!("cannot have multiple start nodes: {} & {}", prev_start, *node);
                }
                start_node = Some(*node);
            }
        }
    }

    if !initial_chips.is_empty() {
        panic!("Bots with initial chips do not have a dst rul in the graph: {:?}", initial_chips);
    }
    let start_node = start_node.expect("The input does not contain a valid start node");

    (start_node, graph)
}

pub fn part_one(start_node: u32, mut graph: HashMap<u32, (Destination, Destination, VecDeque<u32>)>) -> u32 {
    let mut node_stack = vec![start_node];

    while let Some(node_id) = node_stack.pop() {
        let (low, high, vals) = graph.get_mut(&node_id).unwrap();

        assert_eq!(vals.len(), 2);
        let a = vals.pop_front().unwrap();
        let b = vals.pop_front().unwrap();
        if (a == 61 && b == 17) || (a == 17 && b == 61) {
            return node_id;
        }

        let low = *low;
        let high = *high;

        if let Destination::Bot(dst) = low {
            let (.., vals) = graph.get_mut(&dst).unwrap();
            vals.push_back(a.min(b));
            if vals.len() == 2 {
                node_stack.push(dst);
            }
        }

        if let Destination::Bot(dst) = high {
            let (.., vals) = graph.get_mut(&dst).unwrap();
            vals.push_back(a.max(b));
            if vals.len() == 2 {
                node_stack.push(dst);
            }
        }
    }

    panic!("there is no node with 2 chips")
}

pub fn part_two(start_node: u32, mut graph: HashMap<u32, (Destination, Destination, VecDeque<u32>)>) -> u32 {
    let mut node_stack = vec![start_node];
    let mut outputs = [0, 0, 0];

    while let Some(node_id) = node_stack.pop() {
        let (low, high, vals) = graph.get_mut(&node_id).unwrap();

        assert_eq!(vals.len(), 2);
        let a = vals.pop_front().unwrap();
        let b = vals.pop_front().unwrap();
        let low = *low;
        let high = *high;

        match low {
            Destination::Bot(dst) => {
                let (.., vals) = graph.get_mut(&dst).unwrap();
                vals.push_back(a.min(b));
                if vals.len() == 2 {
                    node_stack.push(dst);
                }
            }
            Destination::Output(out) => {
                if (0..3).contains(&out) {
                    outputs[out as usize] = a.min(b);
                }
            }
        }

        match high {
            Destination::Bot(dst) => {
                let (.., vals) = graph.get_mut(&dst).unwrap();
                vals.push_back(a.max(b));
                if vals.len() == 2 {
                    node_stack.push(dst);
                }
            }
            Destination::Output(out) => {
                if (0..3).contains(&out) {
                    outputs[out as usize] = a.max(b);
                }
            }
        }

        if outputs.iter().copied().all(|x| x > 0) {
            return outputs.iter().copied().product();
        }
    }

    panic!("there is no node with 2 chips")
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{parse_input, part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (start_node, graph) = parse_input(&input);

        let answer = part_one(start_node, graph);
        assert_eq!(47, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (start_node, graph) = parse_input(&input);

        let answer = part_two(start_node, graph);
        assert_eq!(2666, answer);
    }
}
