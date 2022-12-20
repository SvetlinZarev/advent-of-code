use crate::{wrap, DECRYPTION_KEY};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, Default, Clone)]
struct Node {
    value: i64,
    next: Weak<RefCell<Node>>,
    prev: Weak<RefCell<Node>>,
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
    let (data, zero_idx) = build_array(input);
    mix(&data);
    compute_answer(data, zero_idx)
}

pub fn part_two(input: &[i64]) -> i64 {
    let (data, zero_idx) = build_array(input);
    for idx in 0..data.len() {
        let mut node = data[idx].borrow_mut();
        node.value *= DECRYPTION_KEY;
    }

    for _ in 0..10 {
        mix(&data);
    }

    compute_answer(data, zero_idx)
}

fn build_array(input: &[i64]) -> (Vec<Rc<RefCell<Node>>>, usize) {
    let mut data = Vec::with_capacity(input.len());
    let mut zero_idx = 0;

    // Init the nodes with their values
    for (idx, value) in input.iter().copied().enumerate() {
        data.push(Rc::new(RefCell::new(Node::new(value))));
        if value == 0 {
            zero_idx = idx;
        }
    }

    // Link the nodes
    for idx in 0..input.len() {
        let mut current = data[idx].borrow_mut();
        let next = &data[(idx + 1) % input.len()];
        let prev = &data[idx.checked_sub(1).unwrap_or(input.len() - 1)];

        current.next = Rc::downgrade(next);
        current.prev = Rc::downgrade(prev);
    }

    (data, zero_idx)
}

fn mix(data: &Vec<Rc<RefCell<Node>>>) {
    for idx in 0..data.len() {
        let current = &data[idx];
        let current_weak = Rc::downgrade(current);
        let mut current_ref = current.borrow_mut();

        let shift = wrap(current_ref.value, data.len() - 1);
        if shift == 0 {
            continue;
        }

        let prev = current_ref.prev.clone();
        let next = current_ref.next.clone();

        prev.upgrade().unwrap().borrow_mut().next = next.clone();
        next.upgrade().unwrap().borrow_mut().prev = prev;

        let mut node = current_ref.next.clone();
        for _ in 1..shift {
            node = node.upgrade().unwrap().borrow().next.clone();
        }

        let next = node.upgrade().unwrap().borrow().next.clone();
        let prev = node;

        next.upgrade().unwrap().borrow_mut().prev = current_weak.clone();
        prev.upgrade().unwrap().borrow_mut().next = current_weak.clone();

        current_ref.next = next;
        current_ref.prev = prev;
    }
}

fn compute_answer(data: Vec<Rc<RefCell<Node>>>, zero_idx: usize) -> i64 {
    let mut node = Rc::downgrade(&data[zero_idx]);
    let mut answer = 0;

    for round in 1..3001 {
        node = node.upgrade().unwrap().borrow().next.clone();
        if round % 1000 == 0 {
            answer += node.upgrade().unwrap().borrow().value;
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
