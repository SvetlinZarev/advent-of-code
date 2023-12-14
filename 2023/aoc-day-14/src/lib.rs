const EMPTY: u8 = b'.';
const OVAL: u8 = b'O';
const FIXED: u8 = b'#';

mod part_one;
mod part_two;

pub fn part_one(input: &Vec<Vec<u8>>) -> usize {
    part_one::part_one(input)
}

pub fn part_two(input: &Vec<Vec<u8>>) -> usize {
    part_two::part_two(input)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_u8_grid;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_u8_grid(input);

        let answer = part_one(&input);
        assert_eq!(113_456, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_u8_grid(input);

        let answer = part_two(&input);
        assert_eq!(118_747, answer);
    }
}
