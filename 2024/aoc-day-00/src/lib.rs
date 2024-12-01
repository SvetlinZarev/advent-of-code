use std::error::Error;

pub fn parse_input(input: &str) -> Result<&str, Box<dyn Error>> {
    Ok(input.trim())
}

pub fn part_one(_input: &str) -> u32 {
    todo!()
}

pub fn part_two(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_one(&parsed);
        assert_eq!(0, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two(&parsed);
        assert_eq!(0, answer);
    }
}
