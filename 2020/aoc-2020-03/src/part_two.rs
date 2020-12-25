use crate::{COLUMNS, MARK_TREE, NEW_LINE, WIDTH};

const SLOPES: &[(usize, usize)] = &[
    // column, row
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
];

pub fn solve(input: &[u8]) -> usize {
    let mut state = [
        //row, col, trees
        (0usize, 0usize, 0usize); SLOPES.len()
    ];

    for (idx, &x) in input.iter().enumerate() {
        // The application cannot process new lines
        // so we have to skip those characters
        if x == NEW_LINE {
            continue;
        }

        let row = idx / WIDTH;
        let col = idx % WIDTH;

        for (slope, &(sc, sr)) in SLOPES.iter().enumerate() {
            let (r, c, t) = &mut state[slope];

            if row == *r {
                if col == *c {
                    if x == MARK_TREE {
                        *t += 1;
                    }

                    *r += sr;
                    *c = (*c + sc) % COLUMNS;
                }
            }
        }
    }

    state.iter().map(|s| s.2).product()
}
