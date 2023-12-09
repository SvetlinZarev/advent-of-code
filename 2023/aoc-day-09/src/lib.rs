pub fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &Vec<Vec<i64>>) -> i64 {
    solve(input, |n| n[n.len() - 1], extrapolate_last)
}

pub fn part_two(input: &Vec<Vec<i64>>) -> i64 {
    solve(input, |n| n[0], extrapolate_first)
}

fn solve(input: &Vec<Vec<i64>>, select: fn(&[i64]) -> i64, extrapolate: fn(&[i64]) -> i64) -> i64 {
    let mut sum = 0;

    let mut nums = vec![];
    let mut next = vec![];
    let mut vals = vec![];

    for line in input.iter() {
        vals.clear();
        nums.clear();
        nums.extend_from_slice(&line);

        loop {
            next.clear();
            vals.push(select(&nums));

            let mut more_to_process = false;
            for w in nums.windows(2) {
                let diff = w[1] - w[0];
                more_to_process |= diff != 0;
                next.push(diff);
            }

            if !more_to_process {
                break;
            }

            std::mem::swap(&mut nums, &mut next);
        }

        sum += extrapolate(&vals);
    }

    sum
}

fn extrapolate_last(nums: &[i64]) -> i64 {
    let mut ext = 0;

    for idx in (0..nums.len()).rev() {
        ext = nums[idx] + ext;
    }

    ext
}

fn extrapolate_first(nums: &[i64]) -> i64 {
    let mut ext = 0;

    for idx in (0..nums.len()).rev() {
        ext = nums[idx] - ext
    }

    ext
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input);

        let answer = part_one(&input);
        assert_eq!(1_772_145_754, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input);

        let answer = part_two(&input);
        assert_eq!(867, answer);
    }
}
