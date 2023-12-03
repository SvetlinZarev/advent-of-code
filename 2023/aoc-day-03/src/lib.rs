use std::collections::HashMap;

const DIR: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_one(input: &Vec<Vec<u8>>) -> u64 {
    let rows = input.len();
    let cols = input[0].len();
    let mut answer = 0;

    for r in 0..rows {
        let mut num = 0;
        let mut is_valid = false;

        for c in 0..cols {
            if !input[r][c].is_ascii_digit() {
                if is_valid {
                    answer += num;
                    is_valid = false;
                }

                num = 0;
                continue;
            }

            num *= 10;
            num += (input[r][c] - b'0') as u64;

            if !is_valid {
                for (dr, dc) in DIR.iter().copied() {
                    let Some(rx) = r.checked_add_signed(dr) else {
                        continue;
                    };
                    let Some(cx) = c.checked_add_signed(dc) else {
                        continue;
                    };
                    if rx >= rows || cx >= cols {
                        continue;
                    }

                    if input[rx][cx] != b'.' && !input[rx][cx].is_ascii_digit() {
                        is_valid = true;
                    }
                }
            }
        }

        // leftover number at the end of the row
        if is_valid {
            answer += num;
        }
    }

    answer
}

pub fn part_two(input: &Vec<Vec<u8>>) -> u64 {
    let rows = input.len();
    let cols = input[0].len();
    let mut gear_rations = HashMap::new();

    for r in 0..rows {
        let mut num = 0;
        let mut is_valid = false;
        let (mut gear_row, mut gear_col) = (0, 0);

        for c in 0..cols {
            if !input[r][c].is_ascii_digit() {
                if is_valid {
                    is_valid = false;

                    gear_rations
                        .entry((gear_row, gear_col))
                        .or_insert(vec![])
                        .push(num);
                }

                num = 0;
                continue;
            }

            num *= 10;
            num += (input[r][c] - b'0') as u64;

            if !is_valid {
                for (dr, dc) in DIR.iter().copied() {
                    let Some(rx) = r.checked_add_signed(dr) else {
                        continue;
                    };
                    let Some(cx) = c.checked_add_signed(dc) else {
                        continue;
                    };
                    if rx >= rows || cx >= cols {
                        continue;
                    }

                    if input[rx][cx] == b'*' {
                        gear_row = rx;
                        gear_col = cx;
                        is_valid = true;
                    }
                }
            }
        }

        // leftover number at the end of the row
        if is_valid {
            gear_rations
                .entry((gear_row, gear_col))
                .or_insert(vec![])
                .push(num);
        }
    }

    gear_rations
        .into_values()
        .filter(|x| x.len() == 2)
        .map(|x| x[0] * x[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_u8_grid;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_u8_grid(input);

        let answer = part_one(&input);
        assert_eq!(527144, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_u8_grid(input);

        let answer = part_two(&input);
        assert_eq!(81463996, answer);
    }
}
