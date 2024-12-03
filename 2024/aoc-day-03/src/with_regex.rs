use regex::Regex;
use std::sync::LazyLock;

static REGEX_1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"mul\(\d+,\d+\)"#).unwrap());
static REGEX_2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"mul\(\d+,\d+\)|do\(\)|don't\(\)"#).unwrap());

pub fn part_one_v1(input: &str) -> u64 {
    let mut sum = 0;

    for m in REGEX_1.find_iter(input) {
        let s = m.as_str();
        let (a, b) = s[4..s.len() - 1].split_once(',').unwrap();
        let x = a.parse::<u64>().unwrap();
        let y = b.parse::<u64>().unwrap();

        sum += x * y;
    }

    sum
}

pub fn part_two_v1(input: &str) -> u64 {
    let mut sum = 0;
    let mut enabled = true;

    for m in REGEX_2.find_iter(input) {
        match m.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            s => {
                if enabled {
                    let (a, b) = s[4..s.len() - 1].split_once(',').unwrap();
                    let x = a.parse::<u64>().unwrap();
                    let y = b.parse::<u64>().unwrap();
                    sum += x * y;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one_v1() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one_v1(&input);
        assert_eq!(175_615_763, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_v1(&input);
        assert_eq!(74_361_272, answer);
    }
}
