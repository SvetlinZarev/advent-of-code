pub mod part_one;
pub mod part_two;

pub fn part_one(input: &str) -> usize {
    part_one::part_one(input)
}

pub fn part_two(input: &str) -> usize {
    part_two::part_two(input)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(2_438, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two(&input);
        assert_eq!(6_658, answer);
    }
}
