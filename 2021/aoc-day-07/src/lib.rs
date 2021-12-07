pub fn part_one(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.sort_unstable();

    let median = (input[(input.len() - 1) / 2] + input[input.len() / 2]) / 2;
    input.iter().fold(0, |acc, &v| {
        acc + if median >= v { median - v } else { v - median }
    })
}

pub fn part_two(input: &[usize]) -> usize {
    let avg = input.iter().sum::<usize>() / input.len();
    input.iter().fold(0, |acc, &v| {
        let positions_to_move = if avg > v { avg - v } else { v - avg };
        let fuel_requirements = positions_to_move * (1 + positions_to_move) / 2;
        acc + fuel_requirements
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_csv;

    #[test]
    fn test_part_one() {
        let input = parse_csv(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(&input);
        assert_eq!(348996, answer);
    }

    #[test]
    fn test_part_two() {
        let input = parse_csv(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(&input);
        assert_eq!(98231647, answer);
    }
}
