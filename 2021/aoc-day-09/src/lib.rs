pub fn parse_input<I: AsRef<str>>(input: I) -> Vec<Vec<u8>> {
    input
        .as_ref()
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .map(|mut v| {
            v.iter_mut().for_each(|x| *x -= b'0');
            v
        })
        .collect()
}

pub fn part_one(input: &[Vec<u8>]) -> usize {
    let mut sum = 0;

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let cell_value = input[row][col];

            if col > 0 && cell_value >= input[row][col - 1] {
                continue;
            }

            if col < input[row].len() - 1 && cell_value >= input[row][col + 1] {
                continue;
            }

            if row > 0 && cell_value >= input[row - 1][col] {
                continue;
            }

            if row < input.len() - 1 && cell_value >= input[row + 1][col] {
                continue;
            }

            sum += (cell_value + 1) as usize;
        }
    }

    sum
}

pub fn part_two(input: &[Vec<u8>]) -> usize {
    let mut input = input.to_vec();

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let cell_value = input[row][col];
            if cell_value >= 9 {
                continue;
            }

            let size = dfs(&mut input, row, col);
            if size > a {
                c = b;
                b = a;
                a = size;
            } else if size > b {
                c = b;
                b = size;
            } else if size > c {
                c = size;
            }
        }
    }

    a * b * c
}

fn dfs(board: &mut [Vec<u8>], r: usize, c: usize) -> usize {
    let mut size = 1;
    board[r][c] = 9;

    //top
    if r > 0 && board[r - 1][c] < 9 {
        size += dfs(board, r - 1, c);
    }

    //left
    if c > 0 && board[r][c - 1] < 9 {
        size += dfs(board, r, c - 1);
    }

    //right
    if c < board[r].len() - 1 && board[r][c + 1] < 9 {
        size += dfs(board, r, c + 1);
    }

    //bottom
    if r < board.len() - 1 && board[r + 1][c] < 9 {
        size += dfs(board, r + 1, c);
    }

    size
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = parse_input(&load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(&input);
        assert_eq!(498, answer);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(&load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(&input);
        assert_eq!(1071000, answer);
    }
}
