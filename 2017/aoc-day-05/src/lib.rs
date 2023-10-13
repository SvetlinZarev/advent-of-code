pub fn part_one(input: &[isize]) -> u32 {
    let mut input = input.to_vec();
    let mut idx = 0isize;

    let mut steps = 0;
    while idx >= 0 && (idx as usize) < input.len() {
        input[idx as usize] += 1;
        idx += input[idx as usize] - 1;
        steps += 1;
    }

    steps
}

pub fn part_two(input: &[isize]) -> u32 {
    let mut input = input.to_vec();
    let mut idx = 0isize;

    let mut steps = 0;
    while idx >= 0 && (idx as usize) < input.len() {
        let diff = if input[idx as usize] < 3 { 1 } else { -1 };
        input[idx as usize] += diff;
        idx += input[idx as usize] - diff;
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);

        assert_eq!(325922, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);

        assert_eq!(24490906, answer);
    }
}
