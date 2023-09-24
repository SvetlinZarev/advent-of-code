const KEY_PAD_1: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
const KEY_PAD_2: [[char; 5]; 5] = [
    [' ', ' ', '1', ' ', ' '],
    [' ', '2', '3', '4', ' '],
    ['5', '6', '7', '8', '9'],
    [' ', 'A', 'B', 'C', ' '],
    [' ', ' ', 'D', ' ', ' '],
];
const EMPTY: char = ' ';

pub fn part_one(input: &str) -> i32 {
    let mut code = 0;
    let mut r = 1usize;
    let mut c = 1usize;

    for line in input.lines() {
        for ch in line.bytes() {
            match ch {
                b'L' => c = c.saturating_sub(1),
                b'U' => r = r.saturating_sub(1),
                b'R' => c = 2.min(c + 1),
                b'D' => r = 2.min(r + 1),
                _ => panic!("unexpected input: {}", ch as char),
            }
        }

        code *= 10;
        code += KEY_PAD_1[r][c];
    }

    code
}

pub fn part_two(input: &str) -> String {
    let mut code = String::new();
    let mut r = 2usize;
    let mut c = 0usize;

    for line in input.lines() {
        for ch in line.bytes() {
            match ch {
                b'L' => {
                    if c == 0 || KEY_PAD_2[r][c - 1] == EMPTY {
                        continue;
                    }
                    c -= 1;
                }
                b'U' => {
                    if r == 0 || KEY_PAD_2[r - 1][c] == EMPTY {
                        continue;
                    }
                    r -= 1;
                }
                b'R' => {
                    if c + 1 >= KEY_PAD_2[r].len() || KEY_PAD_2[r][c + 1] == EMPTY {
                        continue;
                    }
                    c += 1
                }
                b'D' => {
                    if r + 1 >= KEY_PAD_2.len() || KEY_PAD_2[r + 1][c] == EMPTY {
                        continue;
                    }
                    r += 1
                }
                _ => panic!("unexpected input: {}", ch as char),
            }
        }

        code.push(KEY_PAD_2[r][c])
    }

    code
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let answer = part_one(&load_text_input_from_file("inputs/input.txt"));
        assert_eq!(14894, answer);
    }

    #[test]
    fn test_part_two() {
        let answer = part_two(&load_text_input_from_file("inputs/input.txt"));
        assert_eq!("26B96", answer);
    }
}
