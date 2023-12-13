const EMPTY_SPACE: u8 = b'.';
const GALAXY: u8 = b'#';

pub fn part_one(input: &[Vec<u8>]) -> u64 {
    solve(input, 2)
}

pub fn part_two(input: &[Vec<u8>]) -> u64 {
    solve(input, 1_000_000)
}

pub fn solve(input: &[Vec<u8>], expansion_factor: u64) -> u64 {
    let mut galaxies = vec![];

    // rows and cols are sorted in ascending order
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];

    for r in 0..input.len() {
        if input[r].iter().all(|&x| x == EMPTY_SPACE) {
            empty_rows.push(r);
            continue;
        }

        for c in 0..input[r].len() {
            if input[r][c] == GALAXY {
                galaxies.push((r, c));
            }
        }
    }

    for c in 0..input[0].len() {
        let mut empty_col = true;

        for r in 0..input.len() {
            if input[r][c] != EMPTY_SPACE {
                empty_col = false;
                break;
            }
        }

        if empty_col {
            empty_cols.push(c);
        }
    }

    let mut answer = 0;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let min_row = galaxies[i].0.min(galaxies[j].0);
            let max_row = galaxies[i].0.max(galaxies[j].0);

            let min_col = galaxies[i].1.min(galaxies[j].1);
            let max_col = galaxies[i].1.max(galaxies[j].1);

            let expanded_rows_l = empty_rows.partition_point(|&x| x < min_row);
            let expanded_rows_r = empty_rows.partition_point(|&x| x < max_row);
            let expanded_rows = (expanded_rows_r - expanded_rows_l) as u64;

            let expanded_cols_l = empty_cols.partition_point(|&x| x < min_col);
            let expanded_cols_r = empty_cols.partition_point(|&x| x < max_col);
            let expanded_cols = (expanded_cols_r - expanded_cols_l) as u64;

            answer += (max_row - min_row + max_col - min_col) as u64
                + expanded_rows * (expansion_factor - 1)
                + expanded_cols * (expansion_factor - 1);
        }
    }

    answer
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
        assert_eq!(9_795_148, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_u8_grid(input);

        let answer = part_two(&input);
        assert_eq!(650_672_493_820, answer);
    }
}
