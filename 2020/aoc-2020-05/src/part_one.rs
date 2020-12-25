const MAX_ROW: usize = 128;
const MAX_COL: usize = 8;

const NEW_LINE: u8 = b'\n';
const HALF_FRONT: u8 = b'F';
const HALF_BACK: u8 = b'B';
const HALF_LEFT: u8 = b'L';
const HALF_RIGHT: u8 = b'R';

pub fn solve(input: &[u8]) -> usize {
    let mut max_seat_id = 0;
    let mut min_row = 0;
    let mut max_row = MAX_ROW;
    let mut min_col = 0;
    let mut max_col = MAX_COL;

    for &opcode in input.iter() {
        match opcode {
            NEW_LINE => {
                let seat = seat_id(min_row, min_col);
                max_seat_id = max_seat_id.max(seat);

                min_row = 0;
                max_row = MAX_ROW;

                min_col = 0;
                max_col = MAX_COL;
            }

            HALF_FRONT => max_row = middle_of_range(min_row, max_row),
            HALF_BACK => min_row = middle_of_range(min_row, max_row),
            HALF_LEFT => max_col = middle_of_range(min_col, max_col),
            HALF_RIGHT => min_col = middle_of_range(min_col, max_col),
            _ => unreachable!(),
        }
    }

    max_seat_id
}

#[inline(always)]
fn seat_id(r: usize, c: usize) -> usize {
    r * 8 + c
}

#[inline(always)]
fn middle_of_range(l: usize, r: usize) -> usize {
    (l + r) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(567, seat_id(70, 7));
        assert_eq!(119, seat_id(14, 7));
        assert_eq!(820, seat_id(102, 4));
    }
}
