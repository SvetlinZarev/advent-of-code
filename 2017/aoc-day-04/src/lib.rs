use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    let mut uniq = HashSet::new();
    let mut answer = 0;

    'next: for line in input.lines() {
        uniq.clear();

        for word in line.split_whitespace() {
            if !uniq.insert(word) {
                continue 'next;
            }
        }

        answer += 1;
    }

    answer
}

pub fn part_two(input: &str) -> usize {
    let mut uniq = HashSet::new();
    let mut answer = 0;

    'next: for line in input.lines() {
        uniq.clear();

        for word in line.split_whitespace() {
            let mut word = word.as_bytes().to_vec();
            word.sort_unstable();

            if !uniq.insert(word) {
                continue 'next;
            }
        }

        answer += 1;
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
        let answer = part_one(&input);

        assert_eq!(337, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(&input);

        assert_eq!(231, answer);
    }
}
