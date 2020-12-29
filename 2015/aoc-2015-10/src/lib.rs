use std::fmt::Write;
use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

pub const DAY: usize = 10;

const INT_TO_CH: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let input = parse_input(&input);
    let (d1, _) = measure(DAY, "part 1", || look_and_say(&input, 40));
    let (d2, _) = measure(DAY, "part 2", || look_and_say(&input, 50));

    d1 + d2
}

pub fn parse_input(input: &str) -> &str {
    input.trim()
}

pub fn look_and_say(input: &str, times: u32) -> usize {
    let mut dst_buf = String::with_capacity(1024 * 64);
    let mut src_buf = String::with_capacity(1024 * 64);
    src_buf.push_str(input);

    let mut dst = &mut dst_buf;
    let mut src = &mut src_buf;

    for _ in 0..times {
        process_string(src, dst);
        std::mem::swap(&mut src, &mut dst);
    }

    src.len()
}

fn process_string(src: &str, dst: &mut String) {
    let mut count = 0;
    let mut prev = '\0';

    dst.clear();

    for ch in src.chars() {
        if ch != prev && prev != '\0' {
            write_ls_num(dst, count, prev);

            prev = ch;
            count = 1;
            continue;
        }

        count += 1;
        prev = ch;
    }

    write_ls_num(dst, count, prev);
}

fn write_ls_num(dst: &mut String, count: usize, ch: char) {
    // Formatting an integer/char to a string is much slower,
    // than a str.push() with a looked up value

    if count > 9 {
        write!(dst, "{}", count).unwrap();
    } else {
        // For some reason, the look up on my machine
        // is faster than `count + '0'`)
        dst.push(INT_TO_CH[count]);
    }

    dst.push(ch);
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_process_sample_input() {
        let mut buffer = String::new();
        process_string("1", &mut buffer);
        assert_eq!("11", buffer);

        process_string("11", &mut buffer);
        assert_eq!("21", buffer);

        process_string("21", &mut buffer);
        assert_eq!("1211", buffer);

        process_string("1211", &mut buffer);
        assert_eq!("111221", buffer);

        process_string("111221", &mut buffer);
        assert_eq!("312211", buffer);
    }

    #[test]
    pub fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let input = parse_input(&input);

        let answer = look_and_say(input, 40);
        assert_eq!(492982, answer);
    }

    #[test]
    pub fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let input = parse_input(&input);

        let answer = look_and_say(input, 50);
        assert_eq!(6989950, answer);
    }
}
