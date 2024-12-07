use aoc_shared::util::BitSet;
use std::error::Error;

pub fn parse_input(
    input: &str,
) -> Result<(Vec<Vec<(isize, isize)>>, usize, usize), Box<dyn Error>> {
    let mut result = vec![vec![]; (b'z' - b'0' + 1) as usize];
    let mut rows = 0;
    let mut cols = 0;

    for (row, line) in input.lines().enumerate() {
        rows = row;
        cols = line.len();

        for (col, &ch) in line.as_bytes().iter().enumerate() {
            if ch != b'.' {
                result[(ch - b'0') as usize].push((row as isize, col as isize));
            }
        }
    }

    Ok((result, rows + 1, cols))
}

pub fn part_one(antennas: &[Vec<(isize, isize)>], rows: usize, cols: usize) -> usize {
    let mut antinodes = BitSet::new(rows * cols);

    for ants in antennas {
        if ants.is_empty() {
            continue;
        }

        for i in 0..ants.len() - 1 {
            let (r, c) = ants[i];

            for j in i + 1..ants.len() {
                let (y, x) = ants[j];

                let a = r - y;
                let b = c - x;

                let p = r + a;
                let q = c + b;

                let m = y - a;
                let n = x - b;

                if (0..rows as isize).contains(&p) && (0..cols as isize).contains(&q) {
                    let bit = (p as usize) * cols + (q as usize);
                    antinodes.set(bit);
                }

                if (0..rows as isize).contains(&m) && (0..cols as isize).contains(&n) {
                    let bit = (m as usize) * cols + (n as usize);
                    antinodes.set(bit);
                }
            }
        }
    }

    antinodes.count_ones()
}

pub fn part_two(antennas: &[Vec<(isize, isize)>], rows: usize, cols: usize) -> usize {
    let mut antinodes = BitSet::new(rows * cols);

    for ants in antennas {
        if ants.is_empty() {
            continue;
        }

        for i in 0..ants.len() - 1 {
            let (r, c) = ants[i];

            for j in i + 1..ants.len() {
                let (mut y, mut x) = ants[j];
                let (mut r, mut c) = (r, c);

                let a = r - y;
                let b = c - x;

                while (0..rows as isize).contains(&r) && (0..cols as isize).contains(&c) {
                    antinodes.set((r as usize) * cols + (c as usize));

                    r += a;
                    c += b;
                }

                while (0..rows as isize).contains(&y) && (0..cols as isize).contains(&x) {
                    antinodes.set((y as usize) * cols + (x as usize));

                    y -= a;
                    x -= b;
                }
            }
        }
    }

    antinodes.count_ones()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (antennas, rows, cols) = parse_input(&input).unwrap();

        let answer = part_one(&antennas, rows, cols);
        assert_eq!(371, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (antennas, rows, cols) = parse_input(&input).unwrap();

        let answer = part_two(&antennas, rows, cols);
        assert_eq!(1229, answer);
    }
}
