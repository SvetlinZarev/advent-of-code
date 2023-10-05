use std::collections::VecDeque;

pub fn part_one(input: u32) -> u32 {
    let mut elfs = (1..=input).collect::<VecDeque<_>>();
    while let Some(elf) = elfs.pop_front() {
        match elfs.pop_front() {
            Some(_) => elfs.push_back(elf),
            None => return elf,
        }
    }

    unreachable!()
}

pub fn part_two(input: u32) -> u32 {
    let mut a = (1..=input / 2).collect::<VecDeque<_>>();
    let mut b = (input / 2 + 1..=input).collect::<VecDeque<_>>();

    while !a.is_empty() {
        // steal from the elf across the circle
        b.pop_front();

        // Because we always have to "steal" from the "left" elf,
        // we have to maintain that the len(A) <= len(B)
        if b.len() >= a.len() {
            let y = b.pop_front().unwrap();
            a.push_back(y);
        }

        // rotate: move the last elf at the back of the queue
        let x = a.pop_front().unwrap();
        b.push_back(x);
    }

    b.pop_front().unwrap()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = input.trim().parse().unwrap();

        let answer = part_one(input);
        assert_eq!(1834471, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = input.trim().parse().unwrap();

        let answer = part_two(input);
        assert_eq!(1420064, answer);
    }
}
