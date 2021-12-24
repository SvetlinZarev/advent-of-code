#[cfg(test)]
pub(crate) mod naive;

const X: [i64; 14] = [12, 12, 12, -9, -9, 14, 14, -10, 15, -2, 11, -15, -9, -3];
const Y: [i64; 14] = [9, 4, 2, 5, 1, 6, 11, 15, 7, 12, 15, 9, 12, 12];
const DIV: [i64; 14] = [1, 1, 1, 26, 26, 1, 1, 26, 1, 26, 1, 26, 26, 26];

const MAX_Z: [i64; 14] = [
    7722894400, 7722894400, 7722894400, 7722894400, 297034400, 11424400, 11424400, 11424400,
    439400, 439400, 16900, 16900, 650, 25,
];

pub fn part_one() -> u64 {
    let mut x = 0;
    if let Some(solution) = largest_monad() {
        x = solution_to_number(solution)
    }
    x
}

pub fn part_two() -> u64 {
    let mut x = 0;
    if let Some(solution) = smallest_monad() {
        x = solution_to_number(solution)
    }
    x
}

fn largest_monad() -> Option<[u8; 14]> {
    search([0; 14], 0, 0, (1..10).rev())
}

fn smallest_monad() -> Option<[u8; 14]> {
    search([0; 14], 0, 0, 1..10)
}

fn search<R: Iterator<Item = u8> + Clone>(
    mut solution: [u8; 14],
    position: usize,
    z: i64,
    r: R,
) -> Option<[u8; 14]> {
    if position == 14 {
        if z == 0 {
            return Some(solution);
        }

        return None;
    }

    if z > MAX_Z[position] {
        return None;
    }

    for i in r.clone() {
        solution[position] = i;
        let next_z = monad_stage(z, position, i);
        if let Some(result) = search(solution, position + 1, next_z, r.clone()) {
            return Some(result);
        }
    }

    None
}

fn monad_stage(mut z: i64, pos: usize, w: u8) -> i64 {
    let x = z % 26 + X[pos];
    z /= DIV[pos];

    if x != w as i64 {
        z *= 26;
        z += w as i64 + Y[pos];
    }

    z
}

fn solution_to_number(solution: [u8; 14]) -> u64 {
    let mut x = 0;
    for v in solution {
        x *= 10;
        x += v as u64;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_line_delimited_input_from_file;

    #[test]
    fn test_part_one() {
        let answer = part_one();
        assert_eq!(39924989499969, answer);
    }

    #[test]
    fn test_part_two() {
        let answer = part_two();
        assert_eq!(16811412161117, answer);
    }

    #[test]
    fn verify_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let solution = largest_monad().unwrap();
        assert!(naive::monad(&input, &solution));
    }

    #[test]
    fn verify_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let solution = smallest_monad().unwrap();
        assert!(naive::monad(&input, &solution));
    }
}
