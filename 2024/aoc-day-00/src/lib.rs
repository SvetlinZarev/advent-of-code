pub fn parse_input(input: &str) -> &str {
    input
}

pub fn part_one(input: &str) -> u32 {
    todo!()
}

pub fn part_two(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input);

        let answer = part_one(&parsed);
        assert_eq!(0, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(input.trim());

        let answer = part_two(&parsed);
        assert_eq!(0, answer);
    }
}
