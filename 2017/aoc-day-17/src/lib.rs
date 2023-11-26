use std::collections::VecDeque;

pub fn part_one_list(step: usize) -> usize {
    let mut buf = [0; 2018];
    for idx in 0..buf.len() {
        buf[idx] = idx;
    }

    for n in 1..2018 {
        let mut pos = n - 1;

        for _ in 0..step {
            pos = buf[pos];
        }

        buf[n] = buf[pos];
        buf[pos] = n;
    }

    buf[2017]
}

pub fn part_one_deque(step: usize) -> usize {
    let mut deque = VecDeque::new();
    deque.push_back(0);

    for n in 1..2018 {
        deque.rotate_left(step % n);
        deque.push_back(n);
    }

    deque[0]
}

pub fn part_two_deque(step: usize) -> usize {
    let mut deque = VecDeque::new();
    deque.push_back(0);

    for n in 1..50_000_001 {
        deque.rotate_left(step % n);
        deque.push_back(n);
    }

    let zero = deque.iter().position(|&x| x == 0).unwrap();
    deque[(zero + 1) % deque.len()]
}

pub fn part_two_idxs(step: usize) -> usize {
    let mut answer = 0;
    let mut idx = 0;

    for n in 1..50_000_001 {
        idx = (idx + step + 1) % n;
        if idx == 0 {
            answer = n;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one_list() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = input.trim().parse().unwrap();

        let answer = part_one_list(input);
        assert_eq!(1487, answer);
    }

    #[test]
    fn test_part_one_deque() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = input.trim().parse().unwrap();

        let answer = part_one_deque(input);
        assert_eq!(1487, answer);
    }

    #[test]
    fn test_part_two_deque() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = input.trim().parse().unwrap();

        let answer = part_two_deque(input);
        assert_eq!(25674054, answer);
    }

    #[test]
    fn test_part_two_idxs() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = input.trim().parse().unwrap();

        let answer = part_two_idxs(input);
        assert_eq!(25674054, answer);
    }
}
