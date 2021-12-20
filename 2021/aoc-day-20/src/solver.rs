use crate::{Int, Set};

const IDX_9_ONES: usize = (1 << 9) - 1;

pub(crate) fn solve(
    alg: &[u8],
    img: &Set<(Int, Int)>,
    limits: (usize, usize),
    cycles: usize,
) -> usize {
    // The current state of the "infinite" region. Initially all pixels are 0
    let mut state = 0;

    // The state of the "infinite" region after the first enhancement
    let s1 = alg[0] as usize;

    // The state of the "infinite" region after the second enhancement
    let s2 = if s1 == 0 { alg[0] } else { alg[IDX_9_ONES] } as usize;

    // The image will expand with 1 cell per iteration on each side
    let rows = limits.0 + cycles * 2;
    let cols = limits.1 + cycles * 2;

    let mut this_img = vec![0u8; (rows) * (cols)];
    let mut next_img = vec![0u8; this_img.len()];

    // Apply the initial image
    for &(r, c) in img.iter() {
        set(&mut this_img, cols, r + cycles, c + cycles, 1);
    }

    for cycle in 0..cycles {
        let off = cycles - cycle;
        let lim = off - 1;

        for r in lim..rows - lim {
            for c in lim..cols - lim {
                let mut key = 0usize;
                key |= get(&this_img, rows, cols, r, c, off, -1, -1).unwrap_or(state) << 8;
                key |= get(&this_img, rows, cols, r, c, off, -1, 0).unwrap_or(state) << 7;
                key |= get(&this_img, rows, cols, r, c, off, -1, 1).unwrap_or(state) << 6;
                key |= get(&this_img, rows, cols, r, c, off, 0, -1).unwrap_or(state) << 5;
                key |= get(&this_img, rows, cols, r, c, off, 0, 0).unwrap_or(state) << 4;
                key |= get(&this_img, rows, cols, r, c, off, 0, 1).unwrap_or(state) << 3;
                key |= get(&this_img, rows, cols, r, c, off, 1, -1).unwrap_or(state) << 2;
                key |= get(&this_img, rows, cols, r, c, off, 1, 0).unwrap_or(state) << 1;
                key |= get(&this_img, rows, cols, r, c, off, 1, 1).unwrap_or(state) << 0;

                let v = alg[key];
                set(&mut next_img, cols, r, c, v);
            }
        }

        state = if cycle & 1 == 0 { s1 } else { s2 };
        std::mem::swap(&mut this_img, &mut next_img);
    }

    this_img.iter().filter(|&&v| v > 0).count()
}

#[inline(always)]
fn get(
    img: &[u8],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    o: usize,
    rx: isize,
    cx: isize,
) -> Option<usize> {
    let offset = o as isize;
    let rows = rows as isize;
    let cols = cols as isize;

    let row = (r as isize) + rx;
    if row < offset || row >= rows - offset {
        return None;
    }

    let col = (c as isize) + cx;
    if col < offset || col >= cols - offset {
        return None;
    }

    Some(img[(row as usize * cols as usize) + col as usize] as usize)
}

#[inline(always)]
fn set(img: &mut [u8], cols: usize, r: usize, c: usize, val: u8) {
    img[r * cols + c] = val;
}
