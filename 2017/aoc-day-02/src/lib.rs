pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .fold((u32::MAX, u32::MIN), |(min, max), val| {
                    (min.min(val), max.max(val))
                })
        })
        .map(|(min, max)| max - min)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let mut nums = Vec::new();
    input
        .lines()
        .map(|line| {
            nums.clear();
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .for_each(|n| nums.push(n));

            for i in 0..nums.len() - 1 {
                for j in i + 1..nums.len() {
                    let a = nums[i].max(nums[j]);
                    let b = nums[i].min(nums[j]);

                    if a % b == 0 {
                        return a / b;
                    }
                }
            }

            unreachable!("a solution is guaranteed by the problem statement")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(&input);

        assert_eq!(48357, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(&input);

        assert_eq!(351, answer);
    }
}
