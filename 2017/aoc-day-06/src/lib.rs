use std::collections::{HashMap, HashSet};

pub fn part_one(input: &[u32]) -> usize {
    let mut seen = HashSet::new();
    seen.insert(input.to_vec());

    let mut buf = input.to_vec();
    let mut max_val = 0;
    let mut max_pos = 0;

    for idx in 0..buf.len() {
        if buf[idx] > max_val {
            max_val = buf[idx];
            max_pos = idx;
        }
    }

    loop {
        let val = buf[max_pos];
        buf[max_pos] = 0;

        let increment = val / buf.len() as u32;
        let mut remaining = val - increment * buf.len() as u32;

        let offset = max_pos + 1;
        max_val = buf[0];
        max_pos = 0;

        for idx in 0..buf.len() {
            let pos = (offset + idx) % buf.len();

            buf[pos] += increment + (remaining > 0) as u32;
            remaining = remaining.saturating_sub(1);

            if buf[pos] > max_val || (buf[pos] == max_val && pos < max_pos) {
                max_val = buf[pos];
                max_pos = pos;
            }
        }

        if !seen.insert(buf.to_vec()) {
            break;
        }
    }

    seen.len()
}

pub fn part_two(input: &[u32]) -> usize {
    let mut seen = HashMap::new();
    seen.insert(input.to_vec(), 0);

    let mut buf = input.to_vec();
    let mut max_val = 0;
    let mut max_pos = 0;

    for idx in 0..buf.len() {
        if buf[idx] > max_val {
            max_val = buf[idx];
            max_pos = idx;
        }
    }

    loop {
        let val = buf[max_pos];
        buf[max_pos] = 0;

        let increment = val / buf.len() as u32;
        let mut remaining = val - increment * buf.len() as u32;

        let offset = max_pos + 1;
        max_val = buf[0];
        max_pos = 0;

        for idx in 0..buf.len() {
            let pos = (offset + idx) % buf.len();

            buf[pos] += increment + (remaining > 0) as u32;
            remaining = remaining.saturating_sub(1);

            if buf[pos] > max_val || (buf[pos] == max_val && pos < max_pos) {
                max_val = buf[pos];
                max_pos = pos;
            }
        }

        let sequence = seen.len();
        if let Some(start) = seen.insert(buf.to_vec(), sequence) {
            return sequence - start;
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();

        let answer = part_one(&input);
        assert_eq!(3156, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();

        let answer = part_two(&input);

        assert_eq!(1610, answer);
    }
}
