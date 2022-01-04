use std::cmp::Ordering;

pub fn parse_input<S: AsRef<str>>(input: S) -> (u32, u32) {
    let (lo, hi) = input.as_ref().split_once('-').unwrap();
    (lo.parse().unwrap(), hi.parse().unwrap())
}

pub fn part_one(lo: u32, hi: u32) -> u32 {
    solve(lo, hi, is_valid_part_one)
}

pub fn part_two(lo: u32, hi: u32) -> u32 {
    solve(lo, hi, is_valid_part_two)
}

fn solve<F: Fn([u8; 6]) -> bool>(lo: u32, hi: u32, is_valid: F) -> u32 {
    let mut count = 0;
    for n in lo..=hi {
        let digits = to_digits(n);
        if is_valid(digits) {
            count += 1;
        }
    }

    count
}

fn to_digits(n: u32) -> [u8; 6] {
    let mut digits = [0; 6];

    let mut n = n;
    for idx in (0..6).rev() {
        digits[idx] = (n % 10) as u8;
        n = n / 10;
    }
    digits
}

fn is_valid_part_one(digits: [u8; 6]) -> bool {
    let mut has_double = false;

    for idx in 1..6 {
        if digits[idx - 1] > digits[idx] {
            return false;
        }

        // It appears that the rules are not precise - it's
        // possible to have more than 2 equal numbers. I.e.
        // 122234 is valid, just as 112233
        has_double |= digits[idx - 1] == digits[idx];
    }

    has_double
}

fn is_valid_part_two(digits: [u8; 6]) -> bool {
    let mut sequence = 0;
    let mut has_double = false;

    for idx in 1..6 {
        match digits[idx - 1].cmp(&digits[idx]) {
            Ordering::Less => {
                has_double |= sequence == 1;
                sequence = 0;
            }
            Ordering::Equal => sequence += 1,
            Ordering::Greater => return false,
        }
    }

    has_double |= sequence == 1;
    has_double
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared_2019::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let (lo, hi) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(lo, hi);
        assert_eq!(1729, answer);
    }

    #[test]
    fn test_part_two() {
        let (lo, hi) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(lo, hi);
        assert_eq!(1172, answer);
    }
}
