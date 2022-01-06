pub const ROWS: usize = 6;
pub const COLS: usize = 25;

pub fn parse_input<S: AsRef<str>>(input: S) -> Vec<u8> {
    input
        .as_ref()
        .as_bytes()
        .iter()
        .copied()
        .filter(|b| b.is_ascii_digit())
        .map(|b| b - b'0')
        .collect::<Vec<_>>()
}

pub fn part_one(input: &[u8], r: usize, c: usize) -> u64 {
    let layer_size = r * c;

    let mut counts = [0, 0, 0];
    let mut score = 0;
    let mut zeroes = u32::MAX;

    for (idx, x) in input.iter().copied().enumerate() {
        if x < 3 {
            counts[x as usize] += 1;
        }

        if idx != 0 && idx % layer_size == 0 {
            if counts[0] < zeroes {
                zeroes = counts[0];
                score = counts[1] as u64 * counts[2] as u64;
            }
            counts.fill(0);
        }
    }
    score
}

pub fn part_two(input: &[u8], r: usize, c: usize) -> String {
    assert_eq!(input.len() % (r * c), 0);
    let mut grid = vec![vec![2; c]; r];

    let mut idx = 0;
    while idx < input.len() {
        for row in 0..r {
            for col in 0..c {
                if grid[row][col] == 2 {
                    grid[row][col] = input[idx];
                }

                idx += 1;
            }
        }
    }

    grid.into_iter()
        .map(|s| {
            s.into_iter()
                .map(|b| match b {
                    0 => '\u{2591}',
                    1 => '\u{2588}',
                    _ => '@',
                })
                .collect::<String>()
                + "\n"
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared_2019::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(&input, ROWS, COLS);
        assert_eq!(1463, answer);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(&input, ROWS, COLS);
        let expected = "\
            ░██░░█░░█░░██░░█░░█░█░░█░\n\
            █░░█░█░█░░█░░█░█░█░░█░░█░\n\
            █░░░░██░░░█░░░░██░░░████░\n\
            █░██░█░█░░█░░░░█░█░░█░░█░\n\
            █░░█░█░█░░█░░█░█░█░░█░░█░\n\
            ░███░█░░█░░██░░█░░█░█░░█░\n\
        ";
        assert_eq!(expected, answer);
    }
}
