use aoc_shared::hashing::FxHashMap;
use std::collections::VecDeque;

pub fn part_one_v1(input: &str) -> u64 {
    let mut buf = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<VecDeque<u64>>();

    for _ in 0..25 {
        for _ in 0..buf.len() {
            let n = buf.pop_front().unwrap();

            if n == 0 {
                buf.push_back(1);
            } else if digits(n) % 2 == 0 {
                buf.push_back(n / 10u64.pow(digits(n) / 2));
                buf.push_back(n % 10u64.pow(digits(n) / 2));
            } else {
                buf.push_back(n * 2024);
            }
        }
    }

    buf.len() as u64
}

pub fn part_one_v2(input: &str) -> u64 {
    let mut answer = 0;

    let mut cache = Default::default();
    for n in input.split_ascii_whitespace().map(|x| x.parse::<u64>()) {
        answer += dfs(&mut cache, 25, n.unwrap());
    }

    answer
}

pub fn part_two(input: &str) -> u64 {
    let mut answer = 0;

    let mut cache = Default::default();
    for n in input.split_ascii_whitespace().map(|x| x.parse::<u64>()) {
        answer += dfs(&mut cache, 75, n.unwrap());
    }

    answer
}

// Instead of using cache key like `(remaining, number)`,we can compress it in
// a single number `number * 100 + remaining`, because `remaining` has a low
// value that is always less than 100
fn dfs(cache: &mut FxHashMap<u64, u64>, remaining: u64, number: u64) -> u64 {
    // The large numbers do not appear that often, so caching
    // only a portion of the numbers yields a big speed improvement
    // (73% on my machine)
    const MAX_NUM_TO_CACHE: u64 = 1024;

    if remaining == 0 {
        return 1;
    }

    if number <= MAX_NUM_TO_CACHE {
        if let Some(&answer) = cache.get(&(number * 100 + remaining)) {
            return answer;
        }
    }

    let stones = if number == 0 {
        dfs(cache, remaining - 1, 1)
    } else {
        let digits = digits(number);
        if digits % 2 == 0 {
            let mask = 10u64.pow(digits / 2);
            let p = number / mask;
            let q = number % mask;

            let a = dfs(cache, remaining - 1, p);
            let b = dfs(cache, remaining - 1, q);

            a + b
        } else {
            dfs(cache, remaining - 1, number * 2024)
        }
    };

    if number <= MAX_NUM_TO_CACHE {
        cache.insert(number * 100 + remaining, stones);
    }

    stones
}

const fn digits(x: u64) -> u32 {
    match x {
        0..10 => 1,
        10..100 => 2,
        100..1000 => 3,
        1000..10_000 => 4,
        _ => x.ilog10() + 1,
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
        assert_eq!(222_461, answer);
    }

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one_v2(&input);
        assert_eq!(222_461, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two(&input);
        assert_eq!(264_350_935_776_416, answer);
    }
}
