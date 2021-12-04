pub fn part_one(input: &[u32]) -> usize {
    let part_one = input.windows(2).filter(|w| w[1] > w[0]).count();
    part_one
}

pub fn part_two(input: &[u32]) -> usize {
    let part_two = input.windows(4).filter(|w| w[3] > w[0]).count();
    part_two
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};
    use aoc_shared::input::file_line_delimited;

    #[test]
    fn test_part_one() {
        let input = file_line_delimited("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(1832, answer)
    }

    #[test]
    fn test_part_two() {
        let input = file_line_delimited("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(1858, answer)
    }
}
