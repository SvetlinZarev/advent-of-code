use std::error::Error;
use std::num::ParseIntError;

pub struct Node {
    nodes: Vec<Node>,
    meta: Vec<u32>,
}

pub fn parse_input(input: &str) -> Result<Node, Box<dyn Error>> {
    read_node(&mut input.split_ascii_whitespace().map(|x| x.parse()))
}

fn read_node<I: Iterator<Item = Result<u32, ParseIntError>>>(
    input: &mut I,
) -> Result<Node, Box<dyn Error>> {
    let n_nodes = input
        .next()
        .ok_or_else(|| "cannot read header: number of child nodes")??;

    let n_meta = input
        .next()
        .ok_or_else(|| "cannot read header: number of metadata nodes")??;

    let mut nodes = Vec::with_capacity(n_nodes as usize);
    for _ in 0..n_nodes {
        let child = read_node(input)?;
        nodes.push(child);
    }

    let mut meta = Vec::with_capacity(n_meta as usize);
    for _ in 0..n_meta {
        let entry = input.next().ok_or_else(|| "cannot read metadata entry")??;
        meta.push(entry);
    }

    Ok(Node { nodes, meta })
}

pub fn part_one_iter(input: &Node) -> u32 {
    let mut sum = 0;
    let mut stack = vec![input];

    while let Some(node) = stack.pop() {
        sum += node.meta.iter().sum::<u32>();
        stack.extend(node.nodes.iter());
    }

    sum
}

pub fn part_one_rec(input: &Node) -> u32 {
    let mut sum = input.meta.iter().sum::<u32>();

    for child in &input.nodes {
        sum += part_one_rec(child);
    }

    sum
}

pub fn part_two_iter(input: &Node) -> u32 {
    let mut stack = vec![];
    stack.push(input);

    let mut sum = 0;
    while let Some(node) = stack.pop() {
        if node.nodes.is_empty() {
            sum += node.meta.iter().sum::<u32>();
            continue;
        }

        for &idx in node.meta.iter() {
            let idx = idx as usize;

            // All nodes are 1-index based, so a value of 0,
            // does not point to a valid child node
            if idx == 0 {
                continue;
            }

            // Translate to 0-index based arrays fom 1-index based input
            if idx - 1 >= node.nodes.len() {
                continue;
            }

            stack.push(&node.nodes[idx - 1]);
        }
    }

    sum
}

pub fn part_two_rec(input: &Node) -> u32 {
    if input.nodes.is_empty() {
        return input.meta.iter().sum();
    }

    let mut sum = 0;

    for &idx in input.meta.iter() {
        let idx = idx as usize;

        // All nodes are 1-index based, so a value of 0,
        // does not point to a valid child node
        if idx == 0 {
            continue;
        }

        // Translate to 0-index based arrays fom 1-index based input
        if idx - 1 >= input.nodes.len() {
            continue;
        }

        sum += part_two_rec(&input.nodes[idx - 1]);
    }

    sum
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one_iter() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_one_iter(&input);
        assert_eq!(45865, answer);
    }

    #[test]
    fn test_part_one_rec() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_one_rec(&input);
        assert_eq!(45865, answer);
    }

    #[test]
    fn test_part_two_iter() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_two_iter(&input);
        assert_eq!(22608, answer);
    }

    #[test]
    fn test_part_two_rec() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_two_rec(&input);
        assert_eq!(22608, answer);
    }
}
