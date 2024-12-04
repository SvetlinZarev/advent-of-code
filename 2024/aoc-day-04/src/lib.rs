use aoc_shared::grid::DIR8;
use std::error::Error;

pub fn parse_input<'a>(input: &'a str) -> Result<Vec<&'a [u8]>, Box<dyn Error>> {
    Ok(input.lines().map(|l| l.as_bytes()).collect())
}

pub fn part_one_v1(input: &[&[u8]]) -> u32 {
    let mut sum = 0;

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            for (dr, dc) in DIR8 {
                sum += find_word(&input, b"XMAS", row, col, dr, dc);
            }
        }
    }

    sum
}

fn find_word(grid: &[&[u8]], word: &[u8], row: usize, col: usize, dr: isize, dc: isize) -> u32 {
    if grid[row][col] == word[0] {
        if word.len() == 1 {
            return 1;
        }

        let Some(r) = row.checked_add_signed(dr) else {
            return 0;
        };
        let Some(c) = col.checked_add_signed(dc) else {
            return 0;
        };
        if r >= grid.len() || c >= grid[r].len() {
            return 0;
        }

        return find_word(grid, &word[1..], r, c, dr, dc);
    }

    0
}

pub fn part_one_v2(input: &[&[u8]]) -> u32 {
    const XMAS: &[u8] = b"XMAS";
    let mut sum = 0;

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            'next: for (dr, dc) in DIR8 {
                let mut r = row;
                let mut c = col;

                for idx in 0..XMAS.len() - 1 {
                    if input[r][c] != XMAS[idx] {
                        continue 'next;
                    }

                    r = match r.checked_add_signed(dr) {
                        None => continue 'next,
                        Some(x) => x,
                    };

                    c = match c.checked_add_signed(dc) {
                        None => continue 'next,
                        Some(x) => x,
                    };

                    if r >= input.len() || c >= input[r].len() {
                        continue 'next;
                    }
                }

                sum += (input[r][c] == XMAS[XMAS.len() - 1]) as u32;
            }
        }
    }

    sum
}

pub fn part_two_v1(input: &[&[u8]]) -> u32 {
    let mut sum = 0;

    for row in 1..input.len() - 1 {
        for col in 1..input[row].len() - 1 {
            if input[row][col] == b'A' {
                sum += find_x_mas_v1(&input, row, col);
            }
        }
    }

    sum
}

pub fn part_two_v2(input: &[&[u8]]) -> u32 {
    let mut sum = 0;

    for row in 1..input.len() - 1 {
        for col in 1..input[row].len() - 1 {
            if input[row][col] == b'A' {
                sum += find_x_mas_v2(&input, row, col);
            }
        }
    }

    sum
}

fn find_x_mas_v1(grid: &[&[u8]], row: usize, col: usize) -> u32 {
    let tl = grid[row - 1][col - 1];
    let tr = grid[row - 1][col + 1];
    let bl = grid[row + 1][col - 1];
    let br = grid[row + 1][col + 1];

    // M_M
    // _A_
    // S_S
    if (tl == b'M') & (br == b'S') & (tr == b'M') & (bl == b'S') {
        return 1;
    }

    // M_S
    // _A_
    // M_S
    if (tl == b'M') & (br == b'S') & (tr == b'S') & (bl == b'M') {
        return 1;
    }

    // S_M
    // _A_
    // S_M
    if (tl == b'S') & (br == b'M') & (tr == b'M') & (bl == b'S') {
        return 1;
    }

    // S_S
    // _A_
    // M_M
    if (tl == b'S') & (br == b'M') & (tr == b'S') & (bl == b'M') {
        return 1;
    }

    0
}

fn find_x_mas_v2(grid: &[&[u8]], row: usize, col: usize) -> u32 {
    const X1: u32 = 0x4D4D5353; //MMSS
    const X2: u32 = 0x4D534D53; //MSMS
    const X3: u32 = 0x534D534D; //SMSM
    const X4: u32 = 0x53534D4D; //SSMM

    let tl = grid[row - 1][col - 1] as u32;
    let tr = grid[row - 1][col + 1] as u32;
    let bl = grid[row + 1][col - 1] as u32;
    let br = grid[row + 1][col + 1] as u32;
    let x = tl << 24 | tr << 16 | bl << 8 | br;

    ((X1 == x) | (X2 == x) | (X3 == x) | (X4 == x)) as u32
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_one_v1(&parsed);
        assert_eq!(2536, answer);
    }

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_one_v2(&parsed);
        assert_eq!(2536, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two_v1(&parsed);
        assert_eq!(1875, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let parsed = parse_input(&input).unwrap();

        let answer = part_two_v2(&parsed);
        assert_eq!(1875, answer);
    }
}
