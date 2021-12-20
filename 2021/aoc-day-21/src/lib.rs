use aoc_shared::hashing::{FnvHasher, HashBuilder};
use std::collections::HashMap;

const PRACTICE_WIN_SCORE: u64 = 1000;
const QUANTUM_WIN_SCORE: u64 = 21;

const DETERMINISTIC_DIE: [u64; 200] = init_deterministic_die();
const QUANTUM_DIE: [(usize, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

type HashFnFactory = HashBuilder<FnvHasher>;
type Map<K, V> = HashMap<K, V, HashFnFactory>;

const fn init_deterministic_die() -> [u64; 200] {
    let mut dice = [0; 200];

    let mut roll = 0;
    let mut value = 1;
    while roll < 200 {
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
        let rolled = DETERMINISTIC_DIE[rolls % DETERMINISTIC_DIE.len()];
        rolls += 1;

        if rolls % 2 != 0 {
            pos_a = (pos_a + rolled) % 10;
            score_a += pos_a + 1;
            if score_a >= PRACTICE_WIN_SCORE {
                break;
            }
        } else {
            pos_b = (pos_b + rolled) % 10;
            score_b += pos_b + 1;
            if score_b >= PRACTICE_WIN_SCORE {
                break;
            }
        }
    }

    if score_a < 1000 {
        score_a * (rolls as u64) * 3
    } else {
        score_b * (rolls as u64) * 3
    }
}

pub fn part_two(a: usize, b: usize) -> u64 {
    let mut cache = Map::default();
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
