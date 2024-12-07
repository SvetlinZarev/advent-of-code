use std::error::Error;

pub fn part_one_v1(input: &str) -> u64 {
    solve(input, |expected, nums| {
        check_num_fwd::<false>(expected, &nums[1..], nums[0])
    })
    .unwrap()
}

pub fn part_one_v2(input: &str) -> u64 {
    solve(input, |expected, nums| {
        check_num_rev::<false>(expected, nums)
    })
    .unwrap()
}

pub fn part_two_v1(input: &str) -> u64 {
    solve(input, |expected, nums| {
        check_num_fwd::<true>(expected, &nums[1..], nums[0])
    })
    .unwrap()
}

pub fn part_two_v2(input: &str) -> u64 {
    solve(input, |expected, nums| {
        check_num_rev::<true>(expected, nums)
    })
    .unwrap()
}

pub fn solve<C: Fn(u64, &[u64]) -> bool>(input: &str, check_num: C) -> Result<u64, Box<dyn Error>> {
    let mut sum = 0;

    let mut buffer = vec![];
    for line in input.lines() {
        buffer.clear();

        let (expected, rest) = line.split_once(':').unwrap();
        let expected = expected.parse::<u64>()?;
        for val in rest.trim_ascii_start().split_ascii_whitespace() {
            buffer.push(val.parse::<u64>()?);
        }

        if check_num(expected, &buffer) {
            sum += expected;
        }
    }

    Ok(sum)
}

fn check_num_fwd<const CONCAT: bool>(expected: u64, nums: &[u64], val: u64) -> bool {
    if nums.is_empty() {
        return expected == val;
    }

    let b = val * nums[0];
    if b <= expected {
        if check_num_fwd::<CONCAT>(expected, &nums[1..], b) {
            return true;
        }
    }

    if CONCAT {
        let log = nums[0].ilog10() + 1;
        let c = val * 10u64.pow(log) + nums[0];

        if c <= expected {
            if check_num_fwd::<CONCAT>(expected, &nums[1..], c) {
                return true;
            }
        }
    }

    let a = val + nums[0];
    if a <= expected {
        if check_num_fwd::<CONCAT>(expected, &nums[1..], a) {
            return true;
        }
    }

    false
}

fn check_num_rev<const CONCAT: bool>(current: u64, nums: &[u64]) -> bool {
    if current == 0 || nums.is_empty() {
        return current == 0 && nums.is_empty();
    }

    let last = nums[nums.len() - 1];
    if current < last {
        return false;
    }

    let diff = current - last;
    let next_nums = &nums[..nums.len() - 1];

    // Inverse: multiplication
    if current % last == 0 {
        if check_num_rev::<CONCAT>(current / last, next_nums) {
            return true;
        }
    }

    // Inverse: concatenation
    if CONCAT {
        let div = fast_divisor_calc(last);
        if diff >= div && diff % div == 0 {
            let next = diff / div;
            if check_num_rev::<CONCAT>(next, next_nums) {
                return true;
            }
        }
    }

    // Inverse: addition
    check_num_rev::<CONCAT>(diff, next_nums)
}

fn fast_divisor_calc(n: u64) -> u64 {
    match n {
        0..10 => 10,
        10..100 => 100,
        100..1000 => 1000,
        _ => 10u64.pow(n.ilog10() + 1),
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one_v1() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one_v1(&input);
        assert_eq!(3598800864292, answer);
    }

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one_v2(&input);
        assert_eq!(3598800864292, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_v1(&input);
        assert_eq!(340362529351427, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_v2(&input);
        assert_eq!(340362529351427, answer);
    }
}
