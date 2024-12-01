pub fn part_one(input: &str) -> u32 {
    let mut safe = 0;

    for line in input.lines() {
        let nums = fast_parse_line(line);

        if is_safe_1(&nums, |a, b| a > b, |a, b| (1..4).contains(&(a - b))) {
            safe += 1;
        } else if is_safe_1(&nums, |a, b| a < b, |a, b| (1..4).contains(&(b - a))) {
            safe += 1;
        }
    }

    safe
}

fn is_safe_1<CMP: Fn(u32, u32) -> bool, DIFF: Fn(u32, u32) -> bool>(
    nums: &[u32],
    cmp_fn: CMP,
    diff_fn: DIFF,
) -> bool {
    nums.windows(2)
        .all(|w| cmp_fn(w[0], w[1]) && diff_fn(w[0], w[1]))
}

pub fn part_two(input: &str) -> u32 {
    let mut safe = 0;

    for line in input.lines() {
        let nums = fast_parse_line(line);

        if is_safe_2(&nums, |a, b| a > b, |a, b| (1..4).contains(&(a - b))) {
            safe += 1;
        } else if is_safe_2(&nums, |a, b| a < b, |a, b| (1..4).contains(&(b - a))) {
            safe += 1;
        }
    }

    safe
}

fn is_safe_2<CMP: Fn(u32, u32) -> bool, DIFF: Fn(u32, u32) -> bool>(
    nums: &[u32],
    cmp_fn: CMP,
    diff_fn: DIFF,
) -> bool {
    let mut bitset = 0u32;

    nums.windows(2)
        .map(|w| !(cmp_fn(w[0], w[1]) && diff_fn(w[0], w[1])) as u32)
        .enumerate()
        .for_each(|(i, v)| bitset |= v << i);

    if bitset == 0 {
        return true;
    }

    if bitset.count_ones() <= 3 {
        let pos = bitset.trailing_zeros() as usize;

        for idx in [pos, pos + 1] {
            let a = idx == 0
                || idx == nums.len() - 1
                || (cmp_fn(nums[idx - 1], nums[idx + 1]) && diff_fn(nums[idx - 1], nums[idx + 1]));

            let b = a
                && (idx == 0
                    || nums[..idx]
                        .windows(2)
                        .all(|w| cmp_fn(w[0], w[1]) && diff_fn(w[0], w[1])));

            let c = b
                && nums[idx + 1..]
                    .windows(2)
                    .all(|w| cmp_fn(w[0], w[1]) && diff_fn(w[0], w[1]));

            if c {
                return true;
            }
        }
    }

    false
}

fn fast_parse_line(line: &str) -> Vec<u32> {
    let nums = line
        .split_ascii_whitespace()
        .map(|x| match x.as_bytes() {
            &[a] => (a - b'0') as u32,
            &[a, b] => (a - b'0') as u32 * 10 + (b - b'0') as u32,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    nums
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(591, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two(&input);
        assert_eq!(621, answer);
    }
}
