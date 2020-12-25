use crate::{IMAGE_HEIGHT, IMAGE_WIDTH};

const HASHTAGS_IN_MONSTER: usize = 15;
const MONSTER_WIDTH: usize = 20;
const MONSTER_HEIGHT: usize = 3;
const MONSTER_PATTERN: [[u8; MONSTER_WIDTH]; MONSTER_HEIGHT] = [
    [
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' ', b' ', b'#', b' ',
    ],
    [
        b'#', b' ', b' ', b' ', b' ', b'#', b'#', b' ', b' ', b' ', b' ', b'#', b'#', b' ', b' ',
        b' ', b' ', b'#', b'#', b'#',
    ],
    [
        b' ', b'#', b' ', b' ', b'#', b' ', b' ', b'#', b' ', b' ', b'#', b' ', b' ', b'#', b' ',
        b' ', b'#', b' ', b' ', b' ',
    ],
];

pub fn solve(input: &mut Vec<u8>) -> usize {
    assert_eq!(IMAGE_WIDTH * IMAGE_HEIGHT, input.len());
    let monsters = scan_image(input);
    input.iter().copied().filter(|&v| v == b'#').count() - monsters * HASHTAGS_IN_MONSTER
}

fn scan_image(input: &mut [u8]) -> usize {
    // Assume that monsters appear only in the correct orientation

    // It would be faster to flip & rotate the monster's pattern
    for _ in 0..4 {
        rotate_left(input);
        let monsters = find_monsters(input);
        if 0 != monsters {
            return monsters;
        }
    }

    flip_horizontally(input);

    for _ in 0..4 {
        rotate_left(input);
        let monsters = find_monsters(input);
        if 0 != monsters {
            return monsters;
        }
    }

    0
}

fn rotate_left(image: &mut [u8]) {
    let original = image.to_vec();

    for r in 0..IMAGE_HEIGHT {
        for c in 0..IMAGE_WIDTH {
            let src_idx = r * IMAGE_WIDTH + c;

            let dst_row = IMAGE_HEIGHT - 1 - c;
            let dst_col = r;
            let dst_idx = dst_row * IMAGE_WIDTH + dst_col;

            image[dst_idx] = original[src_idx];
        }
    }
}

fn flip_horizontally(image: &mut [u8]) {
    for r in 0..IMAGE_HEIGHT {
        let mut from = r * IMAGE_HEIGHT;
        let mut to = (r + 1) * IMAGE_HEIGHT - 1;

        while from < to {
            let a = image[from];
            image[from] = image[to];
            image[to] = a;

            from += 1;
            to -= 1;
        }
    }
}

fn find_monsters(input: &[u8]) -> usize {
    // Assume monsters do not overlap

    let mut monsters = 0;
    for r in 0..=IMAGE_HEIGHT - MONSTER_HEIGHT {
        for c in 0..=IMAGE_WIDTH - MONSTER_WIDTH {
            let r1f = (r + 0) * IMAGE_WIDTH + c;
            if !is_matching(&input[r1f..r1f + MONSTER_WIDTH], &MONSTER_PATTERN[0]) {
                continue;
            }

            let r2f = (r + 1) * IMAGE_WIDTH + c;
            if !is_matching(&input[r2f..r2f + MONSTER_WIDTH], &MONSTER_PATTERN[1]) {
                continue;
            }

            let r3f = (r + 2) * IMAGE_WIDTH + c;
            if !is_matching(&input[r3f..r3f + MONSTER_WIDTH], &MONSTER_PATTERN[2]) {
                continue;
            }

            monsters += 1;
        }
    }
    monsters
}

fn is_matching(image: &[u8], pattern: &[u8]) -> bool {
    assert_eq!(image.len(), pattern.len());

    for i in 0..pattern.len() {
        if pattern[i] == b'#' {
            if image[i] != b'#' {
                return false;
            }
        }
    }

    true
}
