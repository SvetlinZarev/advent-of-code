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
    let mut sum = 0;

    let mut nums = vec![];
    let mut next = vec![];
    let mut last = vec![];

    for line in input.iter() {
        nums.clear();
        nums.extend_from_slice(&line);

        last.clear();
        last.push(nums[nums.len() - 1]);

        loop {
            let mut more_to_process = false;
            next.clear();

            for w in nums.windows(2) {
                let diff = w[1] - w[0];
                more_to_process |= diff != 0;
                next.push(diff);
            }

            last.push(next[next.len() - 1]);
            std::mem::swap(&mut nums, &mut next);

            if !more_to_process {
                break;
            }
        }

        sum += extrapolate_last(&last);
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

pub fn part_two(input: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;

    let mut nums = vec![];
    let mut next = vec![];
    let mut first = vec![];

    for line in input.iter() {
        nums.clear();
        nums.extend_from_slice(&line);

        first.clear();
        first.push(nums[0]);

        loop {
            let mut more_to_process = false;
            next.clear();

            for w in nums.windows(2).rev() {
                let diff = w[1] - w[0];
                more_to_process |= diff != 0;
                next.push(diff);
            }

            first.push(next[next.len() - 1]);

            next.reverse();
            std::mem::swap(&mut nums, &mut next);

            if !more_to_process {
                break;
            }
        }

        sum += extrapolate_first(&first);
    }

    sum
}

fn extrapolate_first(nums: &[i64]) -> i64 {
    let mut ext = 0;

    for idx in (0..nums.len() - 1).rev() {
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
