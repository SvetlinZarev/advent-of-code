const FACT_A: u64 = 16807;
const FACT_B: u64 = 48271;
const MOD: u64 = 2147483647;

pub fn parse_input(input: &str) -> (u64, u64) {
    let Some((a, b)) = input.trim().split_once('\n') else {
        panic!("invalid input: {:?}", input);
    };

    let a = a.trim_end().split_whitespace().rev().next().unwrap();
    let b = b.trim_end().split_whitespace().rev().next().unwrap();

    (a.parse().unwrap(), b.parse().unwrap())
}

pub fn part_one(a: u64, b: u64) -> usize {
    let mut a = a;
    let mut b = b;
    let mut count = 0;

    for _ in 0..40_000_000 {
        a *= FACT_A;
        a %= MOD;

        b *= FACT_B;
        b %= MOD;

        count += (a & 0xFFFF == b & 0xFFFF) as usize;
    }

    count
}

pub fn part_two(a: u64, b: u64) -> usize {
    let mut a = a;
    let mut b = b;
    let mut count = 0;

    for _ in 0..5_000_000 {
        loop {
            a *= FACT_A;
            a %= MOD;

            if a % 4 == 0 {
                break;
            }
        }

        loop {
            b *= FACT_B;
            b %= MOD;

            if b % 8 == 0 {
                break;
            }
        }

        count += (a & 0xFFFF == b & 0xFFFF) as usize;
    }

    count
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (a, b) = parse_input(&input);

        let answer = part_one(a, b);
        assert_eq!(631, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (a, b) = parse_input(&input);

        let answer = part_two(a, b);
        assert_eq!(279, answer);
    }
}
