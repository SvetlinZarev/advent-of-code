use std::collections::{HashMap, HashSet, VecDeque};

use aoc_shared::hashing::{FnvHasher, HashBuilder};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
}

pub fn parse_input(input: &str) -> ([u16; 4], [u16; 4]) {
    let rgx_gen = regex::Regex::new(r#"a (?<type>[a-z]+) generator"#).unwrap();
    let rgx_chip = regex::Regex::new(r#"a (?<type>[a-z]+)\-compatible"#).unwrap();

    let mut generators = [0u16; 4];
    let mut microchips = [0u16; 4];
    let mut name_ids = HashMap::new();

    for (idx, line) in input.lines().enumerate().take(4) {
        for g in rgx_gen.captures_iter(line) {
            let gen_type = g.name("type").unwrap();

            let next_id = name_ids.len();
            let gen_id = *name_ids.entry(gen_type.as_str()).or_insert(next_id);

            generators[idx] |= 1 << gen_id;
        }

        for g in rgx_chip.captures_iter(line) {
            let gen_type = g.name("type").unwrap();

            let next_id = name_ids.len();
            let gen_id = *name_ids.entry(gen_type.as_str()).or_insert(next_id);

            microchips[idx] |= 1 << gen_id;
        }
    }

    (generators, microchips)
}

pub fn part_one(input: ([u16; 4], [u16; 4])) -> u32 {
    solve_bfs(input).expect("no solution")
}

pub fn part_two(mut input: ([u16; 4], [u16; 4])) -> u32 {
    let total: u32 = input.0.iter().copied().map(|x| x.count_ones()).sum();

    input.0[0] |= 1 << total;
    input.0[0] |= 1 << (total + 1);

    input.1[0] |= 1 << total;
    input.1[0] |= 1 << (total + 1);

    solve_bfs(input).expect("no solution")
}

fn solve_bfs(input: ([u16; 4], [u16; 4])) -> Option<u32> {
    let total: u32 = input.0.iter().copied().map(|x| x.count_ones()).sum();

    let mut queue = VecDeque::new();
    queue.push_back((input.0, input.1, 0usize));

    let mut seen = HashSet::with_hasher(HashBuilder::<FnvHasher>::default());
    let mut steps = 0;

    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (gens, chips, elev) = queue.pop_front().unwrap();
            for dir in [Direction::Up, Direction::Down].iter().copied() {
                // cannot go below floor 0 and above floor 3
                match dir {
                    Direction::Up if elev >= 3 => continue,
                    Direction::Down if elev == 0 => continue,
                    _ => {}
                }

                // move either one or two chips
                for first in 0..total {
                    let mask = 1 << first;

                    // we can move only chips that are present
                    if chips[elev] & mask != mask {
                        continue;
                    }

                    // try to move only one chip at first
                    match move_chips(dir, gens, chips, elev, mask) {
                        Some((chips, elev)) => {
                            if seen.insert((gens, chips, elev)) {
                                if is_answer(gens, chips, elev, total) {
                                    return Some(steps + 1);
                                }

                                queue.push_back((gens, chips, elev));
                            }
                        }
                        None => continue,
                    }

                    // if we are successful, try to move two chips up
                    for second in first + 1..total {
                        let mask = mask | (1 << second);

                        // we can move only chips that are present
                        if chips[elev] & mask != mask {
                            continue;
                        }

                        // try to move both chips
                        if let Some((chips, elev)) = move_chips(dir, gens, chips, elev, mask) {
                            if seen.insert((gens, chips, elev)) {
                                if is_answer(gens, chips, elev, total) {
                                    return Some(steps + 1);
                                }

                                queue.push_back((gens, chips, elev));
                            }
                        }
                    }
                }

                // move either one or two generators
                for first in 0..total {
                    let mask = 1 << first;

                    // we can move only generators that are present
                    if gens[elev] & mask != mask {
                        continue;
                    }

                    // try to move only one generator
                    if let Some((gens, elev)) = move_generators(dir, gens, chips, elev, mask) {
                        if seen.insert((gens, chips, elev)) {
                            if is_answer(gens, chips, elev, total) {
                                return Some(steps + 1);
                            }

                            queue.push_back((gens, chips, elev));
                        }
                    }

                    // try to move two generators up
                    for second in first + 1..total {
                        let mask = mask | (1 << second);

                        // we can move only generators that are present
                        if gens[elev] & mask != mask {
                            continue;
                        }

                        // try to move both generators
                        if let Some((gens, elev)) = move_generators(dir, gens, chips, elev, mask) {
                            if seen.insert((gens, chips, elev)) {
                                if is_answer(gens, chips, elev, total) {
                                    return Some(steps + 1);
                                }

                                queue.push_back((gens, chips, elev));
                            }
                        }
                    }
                }

                // try to move both a generator & a chip
                for idx in 0..total {
                    let mask = 1 << idx;

                    // we can only move a matching chip & generator together
                    if chips[elev] & mask != mask || gens[elev] & mask != mask {
                        continue;
                    }

                    if let Some((gens, chips, elev)) = move_both(dir, gens, chips, elev, mask) {
                        if seen.insert((gens, chips, elev)) {
                            if is_answer(gens, chips, elev, total) {
                                return Some(steps + 1);
                            }

                            queue.push_back((gens, chips, elev));
                        }
                    }
                }
            }
        }

        steps += 1;
    }

    None
}

fn is_answer(gens: [u16; 4], chips: [u16; 4], elev: usize, total: u32) -> bool {
    elev == 3 && (gens[3] & chips[3]).count_ones() == total
}

fn move_chips(
    dir: Direction,
    gens: [u16; 4],
    mut chips: [u16; 4],
    elev: usize,
    mask: u16,
) -> Option<([u16; 4], usize)> {
    // and move them to the next
    let next_elev = match dir {
        Direction::Up => elev + 1,
        Direction::Down => elev - 1,
    };

    chips[elev] &= !mask;
    chips[next_elev] |= mask;

    // we can move a chip only if the floor does not have any
    // generators, or if the chip's matching generator is there
    if gens[next_elev] != 0 && (gens[next_elev] & chips[next_elev]) != chips[next_elev] {
        return None;
    }

    debug_assert!(check_state(gens, chips));
    Some((chips, next_elev))
}

fn move_generators(
    dir: Direction,
    mut gens: [u16; 4],
    chips: [u16; 4],
    elev: usize,
    mask: u16,
) -> Option<([u16; 4], usize)> {
    // and move them to the next
    let next_elev = match dir {
        Direction::Up => elev + 1,
        Direction::Down => elev - 1,
    };

    gens[elev] &= !mask;
    gens[next_elev] |= mask;

    // we can move a generator only if the floor does not have any
    // chips, or if all chips' matching generators are there
    if chips[next_elev] & gens[next_elev] != chips[next_elev] {
        return None;
    }

    // when we move a generator, we must not leave an unprotected chip
    // on the current floor
    if gens[elev] != 0 && chips[elev] & gens[elev] != chips[elev] {
        return None;
    }

    debug_assert!(check_state(gens, chips));
    Some((gens, next_elev))
}

fn move_both(
    dir: Direction,
    mut gens: [u16; 4],
    mut chips: [u16; 4],
    elev: usize,
    mask: u16,
) -> Option<([u16; 4], [u16; 4], usize)> {
    //determine the next floor
    let next_elev = match dir {
        Direction::Up => elev + 1,
        Direction::Down => elev - 1,
    };

    gens[elev] &= !mask;
    chips[elev] &= !mask;

    gens[next_elev] |= mask;
    chips[next_elev] |= mask;

    // when we move a generator, we must not endanger an unprotected chip
    if chips[next_elev] & gens[next_elev] != chips[next_elev] {
        return None;
    }

    debug_assert!(check_state(gens, chips));
    Some((gens, chips, next_elev))
}

fn print_state(gens: [u16; 4], chips: [u16; 4]) {
    println!(
        "G: {:05b} {:05b} {:05b} {:05b}",
        gens[0], gens[1], gens[2], gens[3]
    );
    println!(
        "C: {:05b} {:05b} {:05b} {:05b}",
        chips[0], chips[1], chips[2], chips[3]
    );
    println!();
}

fn check_state(gens: [u16; 4], chips: [u16; 4]) -> bool {
    for floor in 0..3 {
        if gens[floor] > 0 && gens[floor] & chips[floor] != chips[floor] {
            println!("INVALID STATE");
            print_state(gens, chips);
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{parse_input, part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let items = parse_input(&input);

        let answer = part_one(items);
        assert_eq!(33, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let items = parse_input(&input);

        let answer = part_two(items);
        assert_eq!(57, answer);
    }
}
