use aoc_shared::hashing::HashMap;

const LEFT: u8 = b'<';
const RIGHT: u8 = b'>';

const BIT1: u8 = 0b0000_0010;
const BIT7: u8 = 0b1000_0000;

#[rustfmt::skip]
const SHAPE_1: u32 = u32::from_be_bytes([
    0b00000000,
    0b00000000,
    0b00000000,
    0b00111100,
]);

#[rustfmt::skip]
const SHAPE_2: u32 = u32::from_be_bytes([
    0b00000000,
    0b00010000,
    0b00111000,
    0b00010000,
]);

#[rustfmt::skip]
const SHAPE_3: u32 = u32::from_be_bytes([
    0b00000000,
    0b00001000,
    0b00001000,
    0b00111000,
]);

#[rustfmt::skip]
const SHAPE_4: u32 = u32::from_be_bytes( [
    0b00100000,
    0b00100000,
    0b00100000,
    0b00100000,
]);

#[rustfmt::skip]
const SHAPE_5:u32 = u32::from_be_bytes( [
    0b00000000,
    0b00000000,
    0b00110000,
    0b00110000,
]);

const SHAPES: [u32; 5] = [SHAPE_1, SHAPE_2, SHAPE_3, SHAPE_4, SHAPE_5];

const NUMBER_OF_ROCKS: usize = 2022;

pub fn part_one(input: &[u8]) -> usize {
    let mut wind = input.iter().copied().cycle();

    let mut stack: Vec<u8> = Vec::with_capacity(NUMBER_OF_ROCKS * 3);
    'next: for shape in SHAPES.iter().copied().cycle().take(NUMBER_OF_ROCKS) {
        let mut shape = shape.to_be_bytes();

        for _ in 0..4 {
            match wind.next() {
                Some(LEFT) => {
                    if (shape[0] | shape[1] | shape[2] | shape[3]) & BIT7 != 0 {
                        continue;
                    }

                    shape.iter_mut().for_each(|b| *b <<= 1);
                }

                Some(RIGHT) => {
                    if (shape[0] | shape[1] | shape[2] | shape[3]) & BIT1 != 0 {
                        continue;
                    }

                    shape.iter_mut().for_each(|b| *b >>= 1);
                }

                _ => unreachable!(),
            }
        }

        for low in (0..stack.len()).rev() {
            let intersect = (stack.len() - low).min(shape.len());

            let mut overlap = false;
            for pos in 0..intersect {
                overlap |= (shape[shape.len() - 1 - pos] & stack[low + pos]) != 0;
            }

            if overlap {
                let intersect = (stack.len() - low - 1).min(shape.len());
                for pos in 0..intersect {
                    stack[low + pos + 1] |= shape[shape.len() - 1 - pos];
                }

                for pos in intersect..shape.len() {
                    if shape[shape.len() - 1 - pos] != 0 {
                        stack.push(shape[shape.len() - 1 - pos]);
                    }
                }

                continue 'next;
            }

            match wind.next() {
                Some(LEFT) => {
                    let mut cannot_shift = (shape[0] | shape[1] | shape[2] | shape[3]) & BIT7 != 0;
                    for pos in 0..intersect {
                        cannot_shift |=
                            ((shape[shape.len() - 1 - pos] << 1) & stack[low + pos]) != 0;
                    }

                    if !cannot_shift {
                        shape.iter_mut().for_each(|b| *b <<= 1);
                    }
                }

                Some(RIGHT) => {
                    let mut cannot_shift = (shape[0] | shape[1] | shape[2] | shape[3]) & BIT1 != 0;
                    for pos in 0..intersect {
                        cannot_shift |=
                            ((shape[shape.len() - 1 - pos] >> 1) & stack[low + pos]) != 0;
                    }

                    if !cannot_shift {
                        shape.iter_mut().for_each(|b| *b >>= 1);
                    }
                }

                _ => unreachable!(),
            }
        }

        let intersect = stack.len().min(shape.len());
        for pos in 0..intersect {
            stack[pos] |= shape[shape.len() - 1 - pos];
        }

        for pos in intersect..shape.len() {
            if shape[shape.len() - 1 - pos] != 0 {
                stack.push(shape[shape.len() - 1 - pos]);
            }
        }
    }

    stack.len()
}

pub fn part_two(wind: &[u8]) -> u64 {
    const TARGET_ROCKS: u64 = 1_000_000_000_000;
    const CACHE_ROWS: usize = 4;

    let mut next_rock = 0;
    let mut next_wind = 0;
    let mut cache = HashMap::default();

    let mut stack: Vec<u8> = vec![];
    let mut fallen = 0u64;
    let mut height = 0u64;

    'next: while fallen < TARGET_ROCKS {
        let shape = SHAPES[next_rock];
        next_rock = (next_rock + 1) % SHAPES.len();

        let mut shape = shape.to_be_bytes();

        for _ in 0..4 {
            match wind[next_wind] {
                LEFT => {
                    if (shape[0] | shape[1] | shape[2] | shape[3]) & BIT7 == 0 {
                        shape.iter_mut().for_each(|b| *b <<= 1);
                    }
                }

                RIGHT => {
                    if (shape[0] | shape[1] | shape[2] | shape[3]) & BIT1 == 0 {
                        shape.iter_mut().for_each(|b| *b >>= 1);
                    }
                }

                _ => unreachable!(),
            }

            next_wind = (next_wind + 1) % wind.len();
        }

        for low in (0..stack.len()).rev() {
            let intersect = (stack.len() - low).min(shape.len());

            let mut overlap = false;
            for pos in 0..intersect {
                overlap |= (shape[shape.len() - 1 - pos] & stack[low + pos]) != 0;
            }

            if overlap {
                let old_height = stack.len();
                let intersect = (stack.len() - low - 1).min(shape.len());
                for pos in 0..intersect {
                    stack[low + pos + 1] |= shape[shape.len() - 1 - pos];
                }

                for pos in intersect..shape.len() {
                    if shape[shape.len() - 1 - pos] != 0 {
                        stack.push(shape[shape.len() - 1 - pos]);
                    }
                }

                let height_diff = (stack.len() - old_height) as u64;
                height += height_diff;

                fallen += 1;

                if height >= CACHE_ROWS as u64 && fallen % SHAPES.len() as u64 == 0 {
                    // CHECK for cycles by memorizing the last N rows adn wind index
                    if let Some((prev_fallen, prev_height)) = cache.insert(
                        (next_wind, stack[stack.len() - CACHE_ROWS..].to_vec()),
                        (fallen, height),
                    ) {
                        let cycle_len = fallen - prev_fallen;
                        let cycles = (TARGET_ROCKS - fallen) / cycle_len;
                        fallen += cycles * cycle_len;

                        let height_diff = height - prev_height;
                        height += cycles * height_diff;
                        cache.clear();
                    }
                }

                continue 'next;
            }

            match wind[next_wind] {
                LEFT => {
                    let mut cannot_shift = (shape[0] | shape[1] | shape[2] | shape[3]) & BIT7 != 0;
                    for pos in 0..intersect {
                        cannot_shift |=
                            ((shape[shape.len() - 1 - pos] << 1) & stack[low + pos]) != 0;
                    }

                    if !cannot_shift {
                        shape.iter_mut().for_each(|b| *b <<= 1);
                    }
                }

                RIGHT => {
                    let mut cannot_shift = (shape[0] | shape[1] | shape[2] | shape[3]) & BIT1 != 0;
                    for pos in 0..intersect {
                        cannot_shift |=
                            ((shape[shape.len() - 1 - pos] >> 1) & stack[low + pos]) != 0;
                    }

                    if !cannot_shift {
                        shape.iter_mut().for_each(|b| *b >>= 1);
                    }
                }

                _ => unreachable!(),
            }
            next_wind = (next_wind + 1) % wind.len();
        }

        let old_height = stack.len();
        let intersect = stack.len().min(shape.len());
        for pos in 0..intersect {
            stack[pos] |= shape[shape.len() - 1 - pos];
        }

        for pos in intersect..shape.len() {
            if shape[shape.len() - 1 - pos] != 0 {
                stack.push(shape[shape.len() - 1 - pos]);
            }
        }

        let height_diff = (stack.len() - old_height) as u64;
        height += height_diff;

        fallen += 1;
    }

    height
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(input.trim_end().as_bytes());
        assert_eq!(3114, answer)
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(input.trim_end().as_bytes());
        assert_eq!(1540804597682, answer)
    }
}
