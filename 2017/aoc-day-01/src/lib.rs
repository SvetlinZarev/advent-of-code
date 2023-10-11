pub fn part_one(input: &str) -> u32 {
    let input = input.as_bytes();
    input
        .windows(2)
        .chain(std::iter::once(
            [input[0], input[input.len() - 1]].as_slice(),
        ))
        .filter(|w| w[0] == w[1])
        .map(|w| (w[0] - b'0') as u32)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut answer = 0;

    for a in 0..input.len() {
        let b = (a + input.len() / 2) % input.len();
        if input[a] == input[b] {
            answer += (input[a] - b'0') as u32;
        }
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
        let answer = part_one(input.trim_end());

        assert_eq!(1182, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(input.trim_end());

        assert_eq!(1152, answer);
    }
}
