const ASCII_LEN: usize = (b'z' - b'a' + 1) as usize;

pub fn part_one(input: &[String]) -> String {
    let mut columns = vec![[0u32; ASCII_LEN]; input[0].len()];

    for line in input {
        for (col, ch) in line.bytes().enumerate() {
            columns[col][(ch - b'a') as usize] += 1;
        }
    }

    columns
        .iter()
        .map(|vals| {
            vals.iter()
                .copied()
                .enumerate()
                .reduce(|(acc_val, acc_cnt), (val, cnt)| {
                    if cnt > acc_cnt {
                        (val, cnt)
                    } else {
                        (acc_val, acc_cnt)
                    }
                })
                .map(|(ch, _)| (ch as u8 + b'a') as char)
        })
        .flatten()
        .collect()
}

pub fn part_two(input: &[String]) -> String {
    let mut columns = vec![[0u32; ASCII_LEN]; input[0].len()];

    for line in input {
        for (col, ch) in line.bytes().enumerate() {
            columns[col][(ch - b'a') as usize] += 1;
        }
    }

    columns
        .iter()
        .map(|vals| {
            vals.iter().copied().enumerate().fold(
                (usize::MAX, u32::MAX),
                |(acc_val, acc_cnt), (val, cnt)| {
                    if cnt < acc_cnt {
                        (val, cnt)
                    } else {
                        (acc_val, acc_cnt)
                    }
                },
            )
        })
        .map(|(ch, _)| (ch as u8 + b'a') as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!("cyxeoccr", answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!("batwpask", answer);
    }
}
