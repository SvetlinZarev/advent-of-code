use aoc_shared::hashing::{HashMap, HashSet};
use std::mem::swap;

// direction adjustments for the 8 neighbours
const DIR: &[(i32, i32)] = &[
    //up
    (-1, -1),
    (-1, 0),
    (-1, 1),
    //sides
    (0, -1),
    (0, 1),
    //down
    (1, -1),
    (1, 0),
    (1, 1),
];

// indexes and direction adjustments for the 4 sides -> up, down, left, right
const PROP: &[(usize, (i32, i32))] = &[(0, (-1, 0)), (1, (1, 0)), (2, (0, -1)), (3, (0, 1))];

enum Either {
    One((i32, i32)),
    Two(((i32, i32), (i32, i32))),
}

impl Either {
    fn append(&mut self, val: (i32, i32)) {
        match self {
            Either::One(old) => *self = Either::Two((*old, val)),
            Either::Two(_) => panic!(
                "it should not be possible to have more than 2 elves proposing the same spot"
            ),
        }
    }
}

pub fn parse_input(input: impl AsRef<str>) -> Vec<(i32, i32)> {
    let input = input.as_ref();
    let mut answer = vec![];

    for (row, line) in input.lines().enumerate() {
        for (col, cell) in line.bytes().enumerate() {
            if cell == b'#' {
                answer.push((row as i32, col as i32));
            }
        }
    }

    answer
}

pub fn part_one(input: &[(i32, i32)]) -> i32 {
    let mut board = HashSet::default();
    let mut next = HashSet::default();
    let mut proposed = HashMap::default();

    for elf in input.iter().copied() {
        board.insert(elf);
    }

    for round in 0..10 {
        // Propose phase
        for (r, c) in board.iter().copied() {
            // Check which neighbours contain an elf around X
            //  0 | 1 | 2
            //  3 | X | 4
            //  5 | 6 | 7
            // And store the result in a boolean array `[bool; 8]`
            let mut neighbours = [false; 8];
            for (idx, (dr, dc)) in DIR.iter().copied().enumerate() {
                if board.contains(&(r + dr, c + dc)) {
                    neighbours[idx] = true;
                }
            }

            // Compress the 8 variables to make up the 4 sides
            let mut sides = [false; 4];
            sides[0] = neighbours[0] | neighbours[1] | neighbours[2]; //up
            sides[1] = neighbours[5] | neighbours[6] | neighbours[7]; //down
            sides[2] = neighbours[0] | neighbours[3] | neighbours[5]; //left
            sides[3] = neighbours[2] | neighbours[4] | neighbours[7]; //right

            if sides.iter().all(|&x| x) || sides.iter().all(|&x| !x) {
                // immediately place the elves that stay in place
                next.insert((r, c));
                continue;
            }

            for (dir, (dr, dc)) in PROP.iter().copied().cycle().skip(round).take(PROP.len()) {
                if !sides[dir] {
                    proposed
                        .entry((r + dr, c + dc))
                        .and_modify(|x: &mut Either| x.append((r, c)))
                        .or_insert(Either::One((r, c)));
                    break;
                }
            }
        }

        // Move phase
        for (new, old) in proposed.drain() {
            match old {
                Either::One(_) => {
                    next.insert(new);
                }

                Either::Two((a, b)) => {
                    next.insert(a);
                    next.insert(b);
                }
            }
        }

        // reset for the next round
        board.clear();
        swap(&mut next, &mut board);
    }

    let (mut minr, mut minc) = (i32::MAX, i32::MAX);
    let (mut maxr, mut maxc) = (i32::MIN, i32::MIN);

    for (r, c) in board {
        minr = minr.min(r);
        minc = minc.min(c);

        maxr = maxr.max(r);
        maxc = maxc.max(c);
    }

    let area = (maxc - minc + 1) * (maxr - minr + 1);
    area - input.len() as i32
}

pub fn part_two_v1(input: &[(i32, i32)]) -> usize {
    let mut board = HashSet::default();
    let mut next = HashSet::default();
    let mut proposed = HashMap::default();

    for elf in input.iter().copied() {
        board.insert(elf);
    }

    let mut round = 0;

    loop {
        let mut moved = false;

        // Propose phase
        for (r, c) in board.iter().copied() {
            // Check which neighbours contain an elf around X
            //  0 | 1 | 2
            //  3 | X | 4
            //  5 | 6 | 7
            // And store the result in a boolean array `[bool; 8]`
            let mut neighbours = [false; 8];
            for (idx, (dr, dc)) in DIR.iter().copied().enumerate() {
                if board.contains(&(r + dr, c + dc)) {
                    neighbours[idx] = true;
                }
            }

            // Compress the 8 variables to make up the 4 sides
            let mut sides = [false; 4];
            sides[0] = neighbours[0] | neighbours[1] | neighbours[2]; //up
            sides[1] = neighbours[5] | neighbours[6] | neighbours[7]; //down
            sides[2] = neighbours[0] | neighbours[3] | neighbours[5]; //left
            sides[3] = neighbours[2] | neighbours[4] | neighbours[7]; //right

            if sides.iter().all(|&x| x) || sides.iter().all(|&x| !x) {
                // immediately place the elves that stay in place
                next.insert((r, c));
                continue;
            }

            moved = true;
            for (dir, (dr, dc)) in PROP.iter().copied().cycle().skip(round).take(PROP.len()) {
                if !sides[dir] {
                    proposed
                        .entry((r + dr, c + dc))
                        .and_modify(|x: &mut Either| x.append((r, c)))
                        .or_insert(Either::One((r, c)));
                    break;
                }
            }
        }

        round += 1;
        if !moved {
            break;
        }

        // Move phase
        for (new, old) in proposed.drain() {
            match old {
                Either::One(_) => {
                    next.insert(new);
                }

                Either::Two((a, b)) => {
                    next.insert(a);
                    next.insert(b);
                }
            }
        }

        // reset for the next round
        board.clear();
        swap(&mut next, &mut board);
    }

    round
}

pub fn part_two_v2(input: &[(i32, i32)]) -> usize {
    let mut board = HashSet::default();
    let mut next = HashSet::default();

    for elf in input.iter().copied() {
        board.insert(elf);
    }

    let mut round = 0;

    loop {
        let mut moved = false;

        // Propose phase
        for curr @ (r, c) in board.iter().copied() {
            // Check which neighbours contain an elf around X
            //  0 | 1 | 2
            //  3 | X | 4
            //  5 | 6 | 7
            // And store the result in a boolean array `[bool; 8]`
            let mut neighbours = [false; 8];
            for (idx, (dr, dc)) in DIR.iter().copied().enumerate() {
                if board.contains(&(r + dr, c + dc)) {
                    neighbours[idx] = true;
                }
            }

            // Compress the 8 variables to make up the 4 sides
            let mut sides = [false; 4];
            sides[0] = neighbours[0] | neighbours[1] | neighbours[2]; //up
            sides[1] = neighbours[5] | neighbours[6] | neighbours[7]; //down
            sides[2] = neighbours[0] | neighbours[3] | neighbours[5]; //left
            sides[3] = neighbours[2] | neighbours[4] | neighbours[7]; //right

            if sides.iter().all(|&x| x) || sides.iter().all(|&x| !x) {
                // immediately place the elves that stay in place
                next.insert(curr);
                continue;
            }

            moved = true;
            for (dir, (dr, dc)) in PROP.iter().copied().cycle().skip(round).take(PROP.len()) {
                let proposed = (r + dr, c + dc);
                if !sides[dir] {
                    if !next.insert(proposed) {
                        // if two elves proposed the same spot, then
                        // do not move. Push the current position for
                        // the current elf, and push back the other elf
                        next.insert(curr);
                        next.insert((r + 2 * dr, c + 2 * dc));

                        // Remove the duplicate proposal, because no-one will stand there
                        next.remove(&proposed);
                    }
                    break;
                }
            }
        }

        round += 1;
        if !moved {
            break;
        }

        // Move phase; reset for the next round
        board.clear();
        swap(&mut next, &mut board);
    }

    round
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{parse_input, part_one, part_two_v1, part_two_v2};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(input);
        let answer = part_one(&parsed);
        assert_eq!(3920, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(input);
        let answer = part_two_v1(&parsed);
        assert_eq!(889, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(input);
        let answer = part_two_v2(&parsed);
        assert_eq!(889, answer);
    }
}
