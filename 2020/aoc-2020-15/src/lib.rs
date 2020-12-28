use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::parsing::parse_csv;
use aoc_2020_common::timing::measure;

pub const DAY: usize = 15;
pub const MAX_TURNS_PART_ONE: usize = 2020;
pub const MAX_TURNS_PART_TWO: usize = 30000000;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let input = parse_csv(&input);

    let (d1a, _) = measure(DAY, "part 1: v1/vector", || {
        solve_v1(&input, MAX_TURNS_PART_ONE)
    });
    let (d1b, _) = measure(DAY, "part 1: v2/vector", || {
        solve_v2(&input, MAX_TURNS_PART_ONE)
    });
    let (d1c, _) = measure(DAY, "part 1: v3/map", || {
        solve_v3(&input, MAX_TURNS_PART_ONE)
    });

    let (d2a, _) = measure(DAY, "part 2: v1/vector", || {
        solve_v1(&input, MAX_TURNS_PART_TWO)
    });
    let (d2b, _) = measure(DAY, "part 2: v2/vector", || {
        solve_v2(&input, MAX_TURNS_PART_TWO)
    });
    let (d2c, _) = measure(DAY, "part 2: v3/map", || {
        solve_v3(&input, MAX_TURNS_PART_TWO)
    });

    let d1 = d1a.min(d1b).min(d1c);
    let d2 = d2a.min(d2b).min(d2c);
    d1.add(d2)
}

pub fn solve_v1(input: &[usize], max_turns: usize) -> usize {
    assert!(max_turns < usize::max_value());
    assert!(input.len() < max_turns);

    // The cache must be larger than the largest initial number!!!
    let mut cache = vec![0; max_turns];
    for (idx, value) in input.iter().copied().enumerate() {
        cache[value] = idx + 1;
    }

    // The actual turn number is (turn+1) because they are 1 based
    // but on the bright side that saves us some 'x-1' calculations,
    // because we are always working with the previous turn number.
    // The previous turn is the most recent time we could have seen
    // the last said number. Thus the cache contains the "turn before that".
    // From these two values we can compute the next number. Then we have to
    // update the cache, i.e. - the "previous" time we've seen the number -
    // to the previous turn.
    let mut last_said = input[input.len() - 1];
    for turn in input.len()..max_turns {
        let next = match cache[last_said] {
            0 => 0,
            turn_last_seen => turn - turn_last_seen,
        };
        cache[last_said] = turn;
        last_said = next;
    }
    last_said
}

pub fn solve_v2(input: &[usize], max_turns: usize) -> usize {
    assert!(max_turns < usize::max_value());
    assert!(input.len() < max_turns);

    // The cache must be larger than the largest initial number!!!
    let mut cache = vec![usize::max_value(); max_turns];
    for (idx, value) in input.iter().copied().enumerate() {
        cache[value] = idx + 1;
    }

    let mut prev = input[input.len() - 1];
    for turn in input.len()..max_turns {
        let last_turn = std::mem::replace(&mut cache[prev], turn);
        // if the number was not said before, it will 'saturate' at 0
        prev = turn.saturating_sub(last_turn);
    }
    prev
}

pub fn solve_v3(input: &[usize], max_turns: usize) -> usize {
    assert!(max_turns < usize::max_value());
    assert!(input.len() < max_turns + 1);

    let mut cache = HashMap::with_capacity(max_turns / 10);
    for (idx, value) in input.iter().copied().enumerate() {
        cache.insert(value, idx + 1);
    }

    // the actual turn number is (turn+1) because they are 1 based
    // but on the bright side that saves us some 'x-1' calculations
    let mut prev = input[input.len() - 1];
    for turn in input.len()..max_turns {
        let next = match cache.entry(prev) {
            Entry::Vacant(e) => {
                e.insert(turn);
                0
            }

            Entry::Occupied(mut e) => {
                let last_seen = e.insert(turn);
                turn - last_seen
            }
        };
        prev = next;
    }
    prev
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let input = parse_csv(&input);

        let solution = solve_v1(&input, MAX_TURNS_PART_ONE);
        assert_eq!(610, solution);

        let solution = solve_v2(&input, MAX_TURNS_PART_ONE);
        assert_eq!(610, solution);

        let solution = solve_v3(&input, MAX_TURNS_PART_ONE);
        assert_eq!(610, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let input = parse_csv(&input);

        let solution = solve_v1(&input, MAX_TURNS_PART_TWO);
        assert_eq!(1407, solution);

        let solution = solve_v2(&input, MAX_TURNS_PART_TWO);
        assert_eq!(1407, solution);

        let solution = solve_v3(&input, MAX_TURNS_PART_TWO);
        assert_eq!(1407, solution);
    }
}
