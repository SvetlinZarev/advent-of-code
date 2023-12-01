use std::collections::HashSet;

pub fn part_one(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn part_two(input: &[i32]) -> i32 {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    for x in input.iter().copied().cycle() {
        freq += x;
        if !seen.insert(freq) {
            break;
        }
    }

    freq
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(435, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");

        let answer = part_two(&input);
        assert_eq!(245, answer);
    }
}
