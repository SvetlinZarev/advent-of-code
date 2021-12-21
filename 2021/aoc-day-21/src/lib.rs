use aoc_shared::hashing::{FnvHasher, HashBuilder};
use std::collections::HashMap;

const PRACTICE_WIN_SCORE: u64 = 1000;
const QUANTUM_WIN_SCORE: u64 = 21;

// Because the board size is only 10, then every roll of the dice will
// eventually get MOD 10, // thus "1+2+3 == 6" and so is "11+12+13 == 36"
// because "36 % 10 == 6". So there is no need to precalculate all rolls
// of the 100-sided die, because it is equivalent to a 10 sided die
const DETERMINISTIC_DIE_UNIQUE_ROLLS: usize = 10;
const DETERMINISTIC_DIE: [u64; DETERMINISTIC_DIE_UNIQUE_ROLLS] = init_deterministic_die();

// The dirac die has only 3 sides, thus 3 rolls will result in 27 different
// states. But some states are not unique, for instance 1 + 2 + 3 == 2 + 2 + 2
// So we'll calculate the unique rolls and how many times they are repeated
// in the 27 total states: `(roll, repetitions)`
const QUANTUM_DIE_UNIQUE_ROLLS: usize = 7;
const QUANTUM_DIE: [(usize, u64); QUANTUM_DIE_UNIQUE_ROLLS] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

// Type aliases, so we can easily change the actual type, without modifying
// all references in the code. Useful for trying out different hash functions.
type HashFnFactory = HashBuilder<FnvHasher>;
type Map<K, V> = HashMap<K, V, HashFnFactory>;

// Let's try out the compile-time evaluation :)
const fn init_deterministic_die() -> [u64; DETERMINISTIC_DIE_UNIQUE_ROLLS] {
    let mut dice = [0; DETERMINISTIC_DIE_UNIQUE_ROLLS];

    let mut roll = 0;
    let mut value = 1;
    while roll < DETERMINISTIC_DIE_UNIQUE_ROLLS {
        dice[roll] = (value + (value + 1) + (value + 2)) as u64;
        value += 3;
        roll += 1;
    }

    dice
}

pub fn parse_input<S: AsRef<str>>(input: S) -> (usize, usize) {
    let input = input.as_ref();

    let (a, b) = input.split_once('\n').unwrap();
    let (_, pos_a) = a.rsplit_once(' ').unwrap();
    let (_, pos_b) = b.rsplit_once(' ').unwrap();

    let p1 = pos_a.trim().parse().unwrap();
    let p2 = pos_b.trim().parse().unwrap();

    (p1, p2)
}

pub fn part_one(a: usize, b: usize) -> u64 {
    let a = a as u64;
    let b = b as u64;

    let mut pos_a = (a + 9) % 10;
    let mut pos_b = (b + 9) % 10;

    let mut score_a = 0;
    let mut score_b = 0;
    let mut rolls = 0;

    loop {
        let mut rolled = DETERMINISTIC_DIE[rolls % DETERMINISTIC_DIE.len()];
        rolls += 1;

        pos_a = (pos_a + rolled) % 10;
        score_a += pos_a + 1; // +1 because the board is 1-based, while the code is 0-based
        if score_a >= PRACTICE_WIN_SCORE {
            break score_b * (rolls as u64) * 3;
        }

        rolled = DETERMINISTIC_DIE[rolls % DETERMINISTIC_DIE.len()];
        rolls += 1;

        pos_b = (pos_b + rolled) % 10;
        score_b += pos_b + 1; // +1 because the board is 1-based, while the code is 0-based
        if score_b >= PRACTICE_WIN_SCORE {
            break score_a * (rolls as u64) * 3;
        }
    }
}

pub fn part_two(a: usize, b: usize) -> u64 {
    let mut cache = Map::default();

    // Because the board is in the range from 1 to 10 (inclusive), subtract 1
    // so we can use a simple % operation
    let (a, b) = play_dirac(&mut cache, a - 1, b - 1, 0, 0);

    a.max(b)
}

fn play_dirac(
    cache: &mut Map<((usize, u64), (usize, u64)), (u64, u64)>,
    pos_a: usize,
    pos_b: usize,
    score_a: u64,
    score_b: u64,
) -> (u64, u64) {
    // Check for Player-B first, because we are swapping them in the loop below,
    // (the result is also swapped), thus if B has a sore of 21, it means that A
    // has won.
    if score_b >= QUANTUM_WIN_SCORE {
        return (0, 1);
    }

    if let Some(&score) = cache.get(&((pos_a, score_a), (pos_b, score_b))) {
        return score;
    }

    let mut wins_for_a = 0;
    let mut wins_for_b = 0;

    for (roll, freq) in QUANTUM_DIE.iter().copied() {
        let pos_ax = (pos_a + roll) % 10;
        let score_ax = score_a + pos_ax as u64 + 1;

        let (b_wins, a_wins) = play_dirac(cache, pos_b, pos_ax, score_b, score_ax);

        wins_for_a += a_wins * freq;
        wins_for_b += b_wins * freq;
    }

    cache.insert(
        ((pos_a, score_a), (pos_b, score_b)),
        (wins_for_a, wins_for_b),
    );

    (wins_for_a, wins_for_b)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let (a, b) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(a, b);
        assert_eq!(797160, answer);
    }

    #[test]
    fn test_part_two() {
        let (a, b) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(a, b);
        assert_eq!(27464148626406, answer);
    }
}
