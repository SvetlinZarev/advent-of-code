const TRAP: u8 = b'^';
const SAFE: u8 = b'.';

// We need to have N rows, but our first row is the input,
// so we have to generate only N-1 rows
const ROWS_P1: usize = 40 - 1;
const ROWS_P2: usize = 400_000 - 1;

pub fn part_one(input: impl AsRef<[u8]>) -> usize {
    solve_v1(input, ROWS_P1)
}

pub fn part_two(input: impl AsRef<[u8]>) -> usize {
    solve_v1(input, ROWS_P2)
}

fn solve_v1(input: impl AsRef<[u8]>, rows: usize) -> usize {
    let input = input.as_ref();
    let len = input.len();

    let mut a = input.to_vec();
    let mut b = vec![0; len];

    let mut safe = input.iter().copied().filter(|&x| x == SAFE).count();

    for _ in 0..rows {
        for (idx, w) in a.windows(3).enumerate() {
            let cell = match w {
                [TRAP, TRAP, SAFE] => TRAP,
                [SAFE, TRAP, TRAP] => TRAP,
                [SAFE, SAFE, TRAP] => TRAP,
                [TRAP, SAFE, SAFE] => TRAP,
                _ => SAFE,
            };

            b[idx + 1] = cell;
            safe += (cell == SAFE) as usize;
        }

        b[0] = a[1];
        b[len - 1] = a[len - 2];

        safe += (b[0] == SAFE) as usize;
        safe += (b[len - 1] == SAFE) as usize;

        std::mem::swap(&mut a, &mut b);
    }

    safe
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(input.trim());

        assert_eq!(1989, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(input.trim());

        assert_eq!(19999894, answer);
    }
}
