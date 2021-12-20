use crate::{Int, Set};

pub(crate) fn solve(
    alg: &[u8],
    img: &Set<(Int, Int)>,
    limits: (usize, usize),
    cycles: usize,
) -> usize {
    let mut clr0 = 0;
    let mut clr1 = alg[0] as usize;

    // The image will expand with 1 cell per iteration on each side
    let rows = limits.0 + cycles * 2;
    let cols = limits.1 + cycles * 2;

    let mut current = vec![0u8; (rows) * (cols)];
    let mut next = vec![0u8; current.len()];

    // Apply the initial image
    for &(r, c) in img.iter() {
        set(&mut current, cols, r + cycles, c + cycles, 1);
    }

    for _cl in 0..cycles {
        for r in 0..rows {
            for c in 0..cols {
                let mut key = 0usize;
                key |= get(&current, rows, cols, r, c, -1, -1).unwrap_or(clr0) << 8;
                key |= get(&current, rows, cols, r, c, -1, 0).unwrap_or(clr0) << 7;
                key |= get(&current, rows, cols, r, c, -1, 1).unwrap_or(clr0) << 6;
                key |= get(&current, rows, cols, r, c, 0, -1).unwrap_or(clr0) << 5;
                key |= get(&current, rows, cols, r, c, 0, 0).unwrap_or(clr0) << 4;
                key |= get(&current, rows, cols, r, c, 0, 1).unwrap_or(clr0) << 3;
                key |= get(&current, rows, cols, r, c, 1, -1).unwrap_or(clr0) << 2;
                key |= get(&current, rows, cols, r, c, 1, 0).unwrap_or(clr0) << 1;
                key |= get(&current, rows, cols, r, c, 1, 1).unwrap_or(clr0) << 0;

                let v = alg[key];
                set(&mut next, cols, r, c, v);
            }
        }

        std::mem::swap(&mut clr0, &mut clr1);
        std::mem::swap(&mut current, &mut next);
    }

    current.iter().filter(|&&v| v > 0).count()
}

fn get(
    img: &[u8],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    rx: isize,
    cx: isize,
) -> Option<usize> {
    let row = (r as isize) + rx;
    if row < 0 || row >= rows as isize {
        return None;
    }

    let col = (c as isize) + cx;
    if col < 0 || col >= cols as isize {
        return None;
    }

    Some(img[(row as usize * cols) + col as usize] as usize)
}

fn set(img: &mut [u8], cols: usize, r: usize, c: usize, val: u8) {
    img[r * cols + c] = val;
}
