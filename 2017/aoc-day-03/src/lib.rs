use std::collections::HashMap;

use aoc_shared::hashing::{FnvHasher, HashBuilder};

pub fn part_one(target: u64) -> u64 {
    // Find the width of the square that contains the
    // target number in its outermost layer
    let mut lo = 1;
    let mut hi = target;

    while lo < hi {
        let mid = (hi - lo) / 2 + lo;

        if target <= mid * mid {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    let width = hi + (hi % 2 == 0) as u64;

    // As the spiral draws a square, the number of items in it
    // is equal to `side * side`
    let largest_num = width * width;

    // Find the number at the center at each of the 4 sides of the square
    let bottom_center = largest_num - 0 * (width - 1) - width / 2;
    let left_center = largest_num - 1 * (width - 1) - width / 2;
    let top_center = largest_num - 2 * (width - 1) - width / 2;
    let right_center = largest_num - 3 * (width - 1) - width / 2;

    // Find the distance to the center that is closest to the target number
    let a = bottom_center.abs_diff(target);
    let b = left_center.abs_diff(target);
    let c = top_center.abs_diff(target);
    let d = right_center.abs_diff(target);

    // The manhattan distance is always half of the width + the distance
    // to the center of side the number is located at
    width / 2 + a.min(b).min(c).min(d)
}

pub fn part_two(target: u64) -> u64 {
    let mut sums = HashMap::with_hasher(HashBuilder::<FnvHasher>::default());
    sums.insert((0, 0), 1);

    let mut row = 0;
    let mut col = 0;

    let mut max_row = 0;
    let mut min_row = 0;

    let mut max_col = 0;
    let mut min_col = 0;

    loop {
        max_row += 1;
        min_row -= 1;
        max_col += 1;
        min_col -= 1;

        // go right
        while col < max_col {
            col += 1;

            let value = calculate_cell_value(&mut sums, row, col);
            if value > target {
                return value;
            }
        }

        // go up
        while row < max_row {
            row += 1;

            let value = calculate_cell_value(&mut sums, row, col);
            if value > target {
                return value;
            }
        }

        // go left
        while col > min_col {
            col -= 1;

            let value = calculate_cell_value(&mut sums, row, col);
            if value > target {
                return value;
            }
        }

        // go down
        while row > min_row {
            row -= 1;

            let value = calculate_cell_value(&mut sums, row, col);
            if value > target {
                return value;
            }
        }
    }
}

fn calculate_cell_value(
    sums: &mut HashMap<(i32, i32), u64, HashBuilder<FnvHasher>>,
    row: i32,
    col: i32,
) -> u64 {
    const NEIGHBOURS: &[(i32, i32)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut cell_value = 0;
    for (dr, dc) in NEIGHBOURS.iter().copied() {
        let x_row = row + dr;
        let x_col = col + dc;

        if let Some(value) = sums.get(&(x_row, x_col)).copied() {
            cell_value += value;
        }
    }
    sums.insert((row, col), cell_value);

    cell_value
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = input.trim().parse().unwrap();

        let answer = part_one(input);

        assert_eq!(430, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = input.trim().parse().unwrap();

        let answer = part_two(input);

        assert_eq!(312453, answer);
    }
}
