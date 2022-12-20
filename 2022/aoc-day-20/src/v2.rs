use crate::{wrap, DECRYPTION_KEY};

#[derive(Debug, Default, Clone, Copy)]
struct Node {
    value: i64,
    next: usize,
    prev: usize,
}

impl Node {
    fn new(value: i64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

pub fn part_one(input: &[i64]) -> i64 {
    let (mut data, zero_idx) = build_array(input);
    mix(&mut data);
    compute_answer(data, zero_idx)
}

pub fn part_two(input: &[i64]) -> i64 {
    let (mut data, zero_idx) = build_array(input);
    for idx in 0..data.len() {
        data[idx].value *= DECRYPTION_KEY;
    }

    for _ in 0..10 {
        mix(&mut data);
    }

    compute_answer(data, zero_idx)
}

fn build_array(input: &[i64]) -> (Vec<Node>, usize) {
    let mut data = Vec::with_capacity(input.len());
    let mut zero_idx = 0;

    // Init the nodes with their values
    for (idx, value) in input.iter().copied().enumerate() {
        data.push(Node::new(value));
        if value == 0 {
            zero_idx = idx;
        }
    }

    // Link the nodes
    for idx in 0..input.len() {
        data[idx].next = (idx + 1) % input.len();
        data[idx].prev = idx.checked_sub(1).unwrap_or(input.len() - 1);
    }

    (data, zero_idx)
}

fn mix(data: &mut [Node]) {
    for idx in 0..data.len() {
        let current = data[idx];

        let shift = wrap(current.value, data.len() - 1);
        if shift == 0 {
            continue;
        }

        data[current.prev].next = current.next;
        data[current.next].prev = current.prev;

        let mut node = current.next;
        for _ in 1..shift {
            node = data[node].next;
        }

        let next = data[node].next;
        let prev = node;

        data[next].prev = idx;
        data[prev].next = idx;

        data[idx].next = next;
        data[idx].prev = prev;
    }
}

fn compute_answer(data: Vec<Node>, zero_idx: usize) -> i64 {
    let mut position = zero_idx;
    let mut answer = 0;

    for round in 1..3001 {
        position = data[position].next;
        if round % 1000 == 0 {
            answer += data[position].value;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(7584, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(4907679608191, answer);
    }
}
