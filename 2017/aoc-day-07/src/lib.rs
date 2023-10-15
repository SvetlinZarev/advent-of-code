use std::collections::{HashMap, HashSet};
use std::error::Error;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_NODE: Regex =
        Regex::new(r#"^(?<node>[a-z]+) \((?<weight>\d+)\)(?<children> -> [a-z, ]+)?"#).unwrap();
    static ref REGEX_CHILDREN: Regex = Regex::new(r#"([a-z]+)"#).unwrap();
}

pub fn load_input<'l>(
    input: &'l str,
) -> Result<HashMap<&'l str, (i32, Vec<&'l str>)>, Box<dyn Error>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        let Some(cap) = REGEX_NODE.captures(line) else {
            return Err(format!("Line does not match the line regex: {}", line).into());
        };

        let Some(node_name) = cap.name("node").map(|x| x.as_str()) else {
            return Err(format!("Line does not contain a node name: {}", line).into());
        };

        let Some(node_weight) = cap.name("weight").map(|x| x.as_str()) else {
            return Err(format!("Line does not contain a node weight: {}", line).into());
        };
        let node_weight = node_weight.parse()?;

        let mut related = vec![];
        if let Some(children) = cap.name("children").map(|x| x.as_str()) {
            related = REGEX_CHILDREN
                .find_iter(children)
                .map(|x| x.as_str())
                .collect::<Vec<_>>();
        }

        graph.insert(node_name, (node_weight, related));
    }

    Ok(graph)
}

pub fn part_one<'l>(input: &'l HashMap<&'l str, (i32, Vec<&'l str>)>) -> Option<&'l str> {
    let mut child_nodes = HashSet::with_capacity(input.len() + 32);
    for (_, children) in input.values() {
        for child in children.iter().copied() {
            child_nodes.insert(child);
        }
    }

    for parent in input.keys().copied() {
        if !child_nodes.contains(parent) {
            return Some(parent);
        }
    }

    None
}

pub fn part_two(input: &HashMap<&str, (i32, Vec<&str>)>) -> i32 {
    let Some(root) = part_one(input) else {
        panic!("no root node is present");
    };

    let Recursion::Break(answer) = dfs(input, root) else {
        panic!("no solution found");
    };

    answer
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Recursion {
    Continue(i32),
    Break(i32),
}

fn dfs(graph: &HashMap<&str, (i32, Vec<&str>)>, node: &str) -> Recursion {
    let mut node_weight = 0;
    if let Some((w, children)) = graph.get(node) {
        node_weight += w;

        if let Some(&first) = children.first() {
            let reference_weight = match dfs(graph, first) {
                Recursion::Continue(weight) => weight,
                Recursion::Break(answer) => return Recursion::Break(answer),
            };

            node_weight += reference_weight;

            let mut count_reference = 1;
            let mut count_other = 0;

            let mut other_weight = 0;
            let mut affected_node = first;

            for child in children.iter().copied().skip(1) {
                other_weight = match dfs(graph, child) {
                    Recursion::Continue(weight) => weight,
                    Recursion::Break(answer) => return Recursion::Break(answer),
                };

                node_weight += other_weight;

                if other_weight == reference_weight {
                    count_reference += 1;
                } else {
                    count_other += 1;
                    affected_node = child;
                }
            }

            if count_other != 0 {
                if count_other > count_reference {
                    affected_node = first;
                }

                let (weight, _) = graph.get(affected_node).unwrap();
                let answer = *weight + (other_weight - reference_weight);

                return Recursion::Break(answer);
            }
        }
    }

    Recursion::Continue(node_weight)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let graph = load_input(&input).unwrap();

        let answer = part_one(&graph);
        assert_eq!(Some("mwzaxaj"), answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let graph = load_input(&input).unwrap();

        let answer = part_two(&graph);
        assert_eq!(1219, answer);
    }
}
