use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

pub const DAY: usize = 11;

const MAX_CH: u8 = b'z' - b'a';
const MAX_CH_MOD: u8 = MAX_CH + 1;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let part_one_input = parse_input(&input);
    let (d1, pwd) = measure(DAY, "part 1", || solve(&part_one_input));

    let part_two_input = parse_input(&pwd);
    let (d2, _) = measure(DAY, "part 2", || solve(&part_two_input));

    d1 + d2
}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .trim()
        .as_bytes()
        .iter()
        .copied()
        .map(|b| b - b'a')
        .collect()
}

pub fn solve(input: &[u8]) -> String {
    let mut buffer = input.to_vec();
    loop {
        increment_password(&mut buffer);
        if is_valid(&buffer) {
            break;
        }
    }

    buffer.iter_mut().for_each(|x| *x += b'a');
    unsafe { String::from_utf8_unchecked(buffer) }
}

fn increment_password(passwd: &mut [u8]) {
    let mut increment = true;
    for idx in (0..passwd.len()).rev() {
        if !increment {
            break;
        }

        let mut next = (passwd[idx] + 1) % MAX_CH_MOD;

        // if the next value is a forbidden character, zero all
        // the letters to the right because all those passwords
        // will be incorrect as well
        if next == b'i' - b'a' || next == b'o' - b'a' || next == b'l' - b'a' {
            next += 1;
            for i in idx + 1..passwd.len() {
                passwd[i] = 0;
            }
        }
        passwd[idx] = next;
        increment = next == 0;
    }
}

fn is_valid(pwd: &[u8]) -> bool {
    let mut straight = 1;
    let mut pair_1 = u8::max_value();
    let mut pair_2 = u8::max_value();

    let mut prev = u8::max_value();
    for x in pwd.iter().copied() {
        if straight < 3 {
            if x > prev && x - prev == 1 {
                straight += 1;
            } else {
                straight = 1;
            }
        }

        if pair_1 == u8::max_value() {
            if prev == x {
                pair_1 = x;
            }
        }

        if pair_2 == u8::max_value() {
            if prev == x && pair_1 != x {
                pair_2 = x;
            }
        }

        if x == b'i' - b'a' || x == b'o' - b'a' || x == b'l' - b'a' {
            return false;
        }

        prev = x;
    }

    straight >= 3 && pair_1 != u8::max_value() && pair_2 != u8::max_value()
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_parse_input() {
        let pwd = parse_input("abcxyz");
        assert_eq!(&[0, 1, 2, 23, 24, 25], pwd.as_slice());
    }

    #[test]
    fn test_increment_password() {
        let mut pwd = parse_input("az");
        increment_password(&mut pwd);
        assert_eq!(&[1, 0], pwd.as_slice());

        let mut pwd = parse_input("zz");
        increment_password(&mut pwd);
        assert_eq!(&[0, 0], pwd.as_slice());

        let mut pwd = parse_input("aa");
        increment_password(&mut pwd);
        assert_eq!(&[0, 1], pwd.as_slice());
    }

    #[test]
    fn test_is_valid() {
        assert!(!is_valid(parse_input("hijklmmn").as_slice()));
        assert!(!is_valid(parse_input("abbceffg").as_slice()));
        assert!(!is_valid(parse_input("abbcegjk").as_slice()));

        assert!(is_valid(parse_input("abcdffaa").as_slice()));
        assert!(is_valid(parse_input("ghjaabcc").as_slice()));
    }

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let pwd = solve(&parse_input(&input));
        assert_eq!("vzbxxyzz", pwd);
    }

    #[test]
    fn test_part_two() {
        let pwd = solve(&parse_input("vzbxxyzz"));
        assert_eq!("vzcaabcc", pwd);
    }
}
