use std::fmt::Write;

pub fn part_one(input: &str) -> u32 {
    generate_grid(input)
        .into_iter()
        .fold(0, |acc, val| acc + val.count_ones())
}

pub fn part_two(input: &str) -> u32 {
    let mut grid = generate_grid(input);
    let mut groups = 0;

    for row in 0..128 {
        for col in 0..128 {
            if grid[row] & 1 << col != 0 {
                groups += 1;
                dfs(&mut grid, row, col);
            }
        }
    }

    groups
}

fn generate_grid(input: &str) -> [u128; 128] {
    let mut grid = [0; 128];

    let mut buf = String::with_capacity(input.len() + 4);
    for x in 0..128 {
        buf.clear();
        write!(&mut buf, "{}-{}", input, x).unwrap();
        grid[x] = knot_hash(buf.as_bytes());
    }

    grid
}

fn knot_hash(input: &[u8]) -> u128 {
    let len = input.len() + 5;
    let input = input
        .iter()
        .copied()
        .map(|x| x as usize)
        .chain([17, 31, 73, 47, 23].into_iter())
        .cycle();

    let mut data = [0u8; 256];
    for idx in 0..256 {
        data[idx] = idx as u8;
    }

    let mut skip = 0;
    let mut from = 0;

    for length in input.take(len * 64) {
        reverse(&mut data, from, length);

        from = (from + skip + length) % data.len();
        skip += 1;
    }

    data.chunks(16)
        .map(|x| x.into_iter().fold(0, |acc, &val| acc ^ val))
        .fold(0u128, |acc, val| (acc << 8) | (val as u128))
}

fn reverse(arr: &mut [u8], from: usize, length: usize) {
    let len = arr.len();

    for idx in 0..(length / 2) {
        arr.swap((from + idx) % len, (from + length - idx - 1) % len);
    }
}

fn dfs(grid: &mut [u128], row: usize, col: usize) {
    grid[row] &= !(1 << col);

    for (dr, dc) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
        let Some(rx) = row.checked_add_signed(dr) else {
            continue;
        };
        let Some(cx) = col.checked_add_signed(dc) else {
            continue;
        };
        if rx >= 128 || cx >= 128 {
            continue;
        }

        if grid[rx] & (1 << cx) != 0 {
            dfs(grid, rx, cx);
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(input.trim());

        assert_eq!(8292, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(input.trim());

        assert_eq!(1069, answer);
    }
}
