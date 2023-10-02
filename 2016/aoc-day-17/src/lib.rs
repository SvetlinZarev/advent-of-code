use std::collections::VecDeque;

use md5::Digest;

// (row, column, raw_hex[index], bit-shift, path-letter)
const DIR: &[(i32, i32, usize, u8, u8)] = &[
    (-1, 0, 0, 4, b'U'),
    (1, 0, 0, 0, b'D'),
    (0, -1, 1, 4, b'L'),
    (0, 1, 1, 0, b'R'),
];

pub fn part_one(input: &str) -> String {
    let mut queue = VecDeque::new();
    let mut hasher = md5::Md5::default();
    let mut raw_hash = Default::default();

    queue.push_back((0u32, 0u32, vec![]));
    while let Some((r, c, path)) = queue.pop_front() {
        hasher.update(input);
        hasher.update(&path);
        hasher.finalize_into_reset(&mut raw_hash);

        for (dr, dc, idx, shift, p) in DIR.iter().copied() {
            let Some(rx) = r.checked_add_signed(dr) else {
                continue;
            };
            let Some(cx) = c.checked_add_signed(dc) else {
                continue;
            };
            if rx > 3 || cx > 3 {
                continue;
            }

            let value = (raw_hash[idx] >> shift) & 0x0F;
            if !(0x0B..=0x0F).contains(&value) {
                continue;
            }

            if rx == 3 && cx == 3 {
                let mut path = path;
                path.push(p);
                return String::from_utf8(path).unwrap();
            }

            let mut new_path = Vec::with_capacity(path.len() + 1);
            new_path.extend_from_slice(&path);
            new_path.push(p);

            queue.push_back((rx, cx, new_path));
        }
    }

    "no solution".into()
}

pub fn part_two(input: &str) -> usize {
    const MAX_ATTEMPTS: u32 = 1_000_000;
    let mut longest_path = 0;
    let mut attempts = MAX_ATTEMPTS;

    let mut queue = VecDeque::new();
    let mut hasher = md5::Md5::default();
    let mut raw_hash = Default::default();

    queue.push_back((0u32, 0u32, vec![]));
    while let Some((r, c, path)) = queue.pop_front() {
        hasher.update(input);
        hasher.update(&path);
        hasher.finalize_into_reset(&mut raw_hash);

        for (dr, dc, idx, shift, p) in DIR.iter().copied() {
            let Some(rx) = r.checked_add_signed(dr) else {
                continue;
            };
            let Some(cx) = c.checked_add_signed(dc) else {
                continue;
            };
            if rx > 3 || cx > 3 {
                continue;
            }

            let value = (raw_hash[idx] >> shift) & 0x0F;
            if !(0x0B..=0x0F).contains(&value) {
                continue;
            }

            if rx == 3 && cx == 3 {
                longest_path = longest_path.max(path.len() + 1);
                continue;
            }

            let mut new_path = Vec::with_capacity(path.len() + 1);
            new_path.extend_from_slice(&path);
            new_path.push(p);

            queue.push_back((rx, cx, new_path));
        }

        attempts -= 1;
        if attempts == 0 {
            break;
        }
    }

    longest_path
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(input.trim());
        assert_eq!("DDRRUDLRRD", answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(input.trim());
        assert_eq!(488, answer);
    }
}
