const MAX_ROW: usize = 128;
const MAX_COL: usize = 8;

const NEW_LINE: u8 = b'\n';
const HALF_FRONT: u8 = b'F';
const HALF_BACK: u8 = b'B';
const HALF_LEFT: u8 = b'L';
const HALF_RIGHT: u8 = b'R';

pub fn solve_v1(input: &[u8]) -> Option<usize> {
    let mut seats = [[false; 8]; 128];
    let mut min_row = 0;
    let mut max_row = MAX_ROW;
    let mut min_col = 0;
    let mut max_col = MAX_COL;

    for &opcode in input.iter() {
        match opcode {
            NEW_LINE => {
                seats[min_row][min_col] = true;
                max_row = MAX_ROW;
                max_col = MAX_COL;
                min_row = 0;
                min_col = 0;
            }

            HALF_FRONT => max_row = middle_of_range(min_row, max_row),
            HALF_BACK => min_row = middle_of_range(min_row, max_row),
            HALF_LEFT => max_col = middle_of_range(min_col, max_col),
            HALF_RIGHT => min_col = middle_of_range(min_col, max_col),
            _ => unreachable!(),
        }
    }

    let mut solution = None;
    //skip the very first row and very last rows, as we know our seat is not there
    'all: for (row_idx, row) in seats.iter().enumerate().skip(1).take(seats.len() - 2) {
        for (col_idx, &taken) in row.iter().enumerate() {
            if !taken {
                if !seats[row_idx - 1][col_idx] {
                    continue; //the -1 seat should be taken
                }

                if !seats[row_idx + 1][col_idx] {
                    continue; //the +1 seat should be taken
                }

                if cfg!(debug_assertions) {
                    println!(
                        "{:03}/{:02}: {}",
                        row_idx,
                        col_idx,
                        seat_id(row_idx, col_idx)
                    );
                }

                solution = Some(seat_id(row_idx, col_idx));
                break 'all;
            }
        }
    }
    solution
}

#[inline(always)]
fn seat_id(r: usize, c: usize) -> usize {
    r * 8 + c
}

#[inline(always)]
fn middle_of_range(l: usize, r: usize) -> usize {
    (l + r) / 2
}

pub fn solve_v2_xor(input: &[u8]) -> usize {
    let mut checksum = 0;
    let (mut min_seat, mut max_seat) = (usize::max_value(), usize::min_value());
    let (mut min_row, mut max_row) = (0usize, MAX_ROW);
    let (mut min_col, mut max_col) = (0usize, MAX_COL);

    for &opcode in input.iter() {
        match opcode {
            NEW_LINE => {
                let seat = seat_id(min_row, min_col);
                min_seat = min_seat.min(seat);
                max_seat = max_seat.max(seat);
                checksum ^= seat;

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

    /*
       # How it works:
       -------------------
       The seat id is calculated as follows: `row * 8 + col`
       a) We know that we have 128 rows - from 0 to 127 - requires 7 bits
       b) We know that we have 8 columns - from 0 to 7 - requires 3 bits

       Multiplying by 8 is equivalent to shifting left by 3 positions. Adding
       3 bit number to that is equivalent to doing a bitwise OR operation.

       So we can rewrite the seat id formula as `(row << 3) | col`. In other
       words the formula is designed to create a 10 bit ID. This limits the
       number of valid IDs to 2^10 which is 1024 - from 0 to 1023

       If we XOR avery number from 0 to 1023 will get 0 as a result:
       ```
       let mut checksum = 0;
       for x in 0..1024 {
           checksum ^= x;
       }
       assert_eq!(0, checksum);
       ```

       Given that only our ID is missing we can XOR all ids and the result
       will contain our ID. But the plane has less than 1024 seats. Luckily
       all seat IDs are consecutive numbers due to formula used to calculate
       them. So we need to find the seat IDs that are not present on the plane.
       Basically these are the seats which ID is smaller than the smallest ID
       we've calculated and the IDs which are larger than the largest ID we've
       found. Luckily we know that `min_id < our_id < max_id` because we are
       not seated on the first/last rows.

       As a result we can find our ID in two ways:
       a) loop over min_id..=max_id and XOR the values
          This will work because XORing the same value twice equals 0,
          and we know that our seat ID is within this range, but was not
          xor-ed. So we basically have to solve `checksum XOR our_id = 0`
       b) loop over 0..min_id and max_id+1..1024 and XOR the values
          This will work because if we XOR all values from 0..1024 we'll get 0,
          so again we are basically solving `checksum XOR our_id = 0`
    */

    for x in 0..min_seat {
        checksum ^= x;
    }

    for x in max_seat + 1..1024 {
        checksum ^= x;
    }

    checksum
}

pub fn solve_v3_bitwise(input: &[u8]) -> usize {
    let mut seat = 0;
    let (mut min_seat, mut max_seat) = (usize::max_value(), usize::min_value());
    let mut checksum = 0;

    /*
        The magic is in the following:

        We have a 10 bit number (and 10 letter code). Each letter of the code
        corresponds to a bit in the number. The letters `B` and `R` are `1`
        and `F` and `L` are `0`. So each iteration we set one bit and shift
        left in order to prepare for the next bit.
    */

    for &opcode in input.iter() {
        match opcode {
            NEW_LINE => {
                seat >>= 1; //shr because of the last, unnecessary shl
                min_seat = min_seat.min(seat);
                max_seat = max_seat.max(seat);
                checksum ^= seat;
                seat = 0;
            }

            HALF_BACK => {
                seat |= 1;
            }

            HALF_RIGHT => {
                seat |= 1;
            }

            _ => {
                //noop
            }
        }

        seat <<= 1;
    }

    /*
       # How it works:
       -------------------
       The seat id is calculated as follows: `row * 8 + col`
       a) We know that we have 128 rows - from 0 to 127 - requires 7 bits
       b) We know that we have 8 columns - from 0 to 7 - requires 3 bits

       Multiplying by 8 is equivalent to shifting left by 3 positions. Adding
       3 bit number to that is equivalent to doing a bitwise OR operation.

       So we can rewrite the seat id formula as `(row << 3) | col`. In other
       words the formula is designed to create a 10 bit ID. This limits the
       number of valid IDs to 2^10 which is 1024 - from 0 to 1023

       If we XOR avery number from 0 to 1023 will get 0 as a result:
       ```
       let mut checksum = 0;
       for x in 0..1024 {
           checksum ^= x;
       }
       assert_eq!(0, checksum);
       ```

       Given that only our ID is missing we can XOR all ids and the result
       will contain our ID. But the plane has less than 1024 seats. Luckily
       all seat IDs are consecutive numbers due to formula used to calculate
       them. So we need to find the seat IDs that are not present on the plane.
       Basically these are the seats which ID is smaller than the smallest ID
       we've calculated and the IDs which are larger than the largest ID we've
       found. Luckily we know that `min_id < our_id < max_id` because we are
       not seated on the first/last rows.

       As a result we can find our ID in two ways:
       a) loop over min_id..=max_id and XOR the values
          This will work because XORing the same value twice equals 0,
          and we know that our seat ID is within this range, but was not
          xor-ed. So we basically have to solve `checksum XOR our_id = 0`
       b) loop over 0..min_id and max_id+1..1024 and XOR the values
          This will work because if we XOR all values from 0..1024 we'll get 0,
          so again we are basically solving `checksum XOR our_id = 0`
    */

    for x in 0..min_seat {
        checksum ^= x;
    }

    for x in max_seat + 1..1024 {
        checksum ^= x;
    }

    checksum
}
