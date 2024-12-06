pub fn part_one(input: &str) -> u64 {
    solve(input, check_num_1)
}

pub fn part_two(input: &str) -> u64 {
    solve(input, check_num_2)
}

pub fn solve<C: Fn(u64, &[u64]) -> bool>(input: &str, check_num: C) -> u64 {
    let mut sum = 0;

    let mut buffer = vec![];
    for line in input.lines() {
        buffer.clear();

        let (expected, rest) = line.split_once(':').unwrap();
        let expected = expected.parse::<u64>().unwrap();
        for val in rest.trim_ascii_start().split_ascii_whitespace() {
            buffer.push(val.parse::<u64>().unwrap());
        }

        if check_num(expected, &buffer) {
            sum += expected;
        }
    }

    sum
}

fn check_num_1(expected: u64, nums: &[u64]) -> bool {
    check_sum_dfs_1(expected, &nums[1..], nums[0])
}

fn check_sum_dfs_1(expected: u64, nums: &[u64], val: u64) -> bool {
    if nums.is_empty() {
        return expected == val;
    }

    let a = val + nums[0];
    let b = val * nums[0];

    if a <= expected {
        if check_sum_dfs_1(expected, &nums[1..], a) {
            return true;
        }
    }

    if b <= expected {
        if check_sum_dfs_1(expected, &nums[1..], b) {
            return true;
        }
    }

    false
}

fn check_num_2(expected: u64, nums: &[u64]) -> bool {
    check_sum_dfs_2(expected, &nums[1..], nums[0])
}

fn check_sum_dfs_2(expected: u64, nums: &[u64], val: u64) -> bool {
    if nums.is_empty() {
        return expected == val;
    }

    let a = val + nums[0];
    let b = val * nums[0];

    let log = nums[0].ilog10() + 1;
    let c = val * 10u64.pow(log) + nums[0];

    if a <= expected {
        if check_sum_dfs_2(expected, &nums[1..], a) {
            return true;
        }
    }

    if b <= expected {
        if check_sum_dfs_2(expected, &nums[1..], b) {
            return true;
        }
    }

    if c <= expected {
        if check_sum_dfs_2(expected, &nums[1..], c) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(3598800864292, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two(&input);
        assert_eq!(340362529351427, answer);
    }
}
