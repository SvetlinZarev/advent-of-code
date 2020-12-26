use crate::{Action, Instruction};

const BITS: usize = 64;
const COLUMNS: usize = 1000;
const HEIGHT: usize = 1000;

pub fn solve(instructions: &[Instruction]) -> u32 {
    let width = (COLUMNS as f64 / BITS as f64).ceil() as usize;

    // Bit 0 means OFF, 1 means ON
    let mut grid = vec![0; width * HEIGHT];
    let mut mask = vec![0; width];

    for instruction in instructions.iter().copied() {
        let from_mask = mask_idx(instruction.from.x);
        let to_mask = mask_idx(instruction.to.x);

        if from_mask != to_mask {
            mask[from_mask] = clip_left(instruction.from.x);
            for idx in from_mask + 1..to_mask {
                mask[idx] = u64::max_value();
            }
            mask[to_mask] = clip_right(instruction.to.x);
        } else {
            let a = clip_left(instruction.from.x);
            let b = clip_right(instruction.to.x);
            mask[from_mask] = a & b;
        }

        for row in instruction.from.y..=instruction.to.y {
            let offset = row as usize * width;

            for col in from_mask..=to_mask {
                let idx = offset + col;
                let x = grid[idx];
                let m = mask[col];

                grid[idx] = match instruction.action {
                    Action::On => turn_on(x, m),
                    Action::Off => turn_off(x, m),
                    Action::Toggle => toggle(x, m),
                }
            }
        }
    }

    grid.iter().copied().map(|v| v.count_ones()).sum()
}

fn mask_idx(bit: u16) -> usize {
    bit as usize / BITS
}

fn clip_left(from_bit: u16) -> u64 {
    u64::max_value() >> from_bit as usize % BITS
}

fn clip_right(to_bit: u16) -> u64 {
    let to_bit = to_bit as usize % BITS;
    u64::max_value() << BITS - 1 - to_bit
}

fn toggle(x: u64, mask: u64) -> u64 {
    x ^ mask
}

fn turn_on(x: u64, mask: u64) -> u64 {
    x | mask
}

fn turn_off(x: u64, mask: u64) -> u64 {
    x & !mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_left_0() {
        let m = clip_left(0);
        assert_eq!(u64::max_value(), m, "Mask: {:064b}", m);
    }

    #[test]
    fn test_clip_left_3() {
        let m = clip_left(3);
        assert_eq!(u64::max_value() >> 3, m, "Mask: {:064b}", m);
    }

    #[test]
    fn test_clip_left_63() {
        let m = clip_left(63);
        assert_eq!(u64::max_value() >> 63, m, "Mask: {:064b}", m);
    }

    #[test]
    fn test_clip_right_0() {
        let m = clip_right(0);
        assert_eq!(1u64 << 63, m, "Mask: {:064b}", m);
    }

    #[test]
    fn test_clip_right_3() {
        let m = clip_right(3);
        assert_eq!(u64::max_value() << BITS - 4, m, "Mask: {:064b}", m);
    }

    #[test]
    fn test_clip_right_63() {
        let m = clip_right(63);
        assert_eq!(u64::max_value(), m, "Mask: {:064b}", m);
    }

    #[test]
    fn test_toggle() {
        let x = toggle(0, u64::max_value());
        assert_eq!(u64::max_value(), x);

        let x = toggle(u64::max_value(), u64::max_value());
        assert_eq!(0, x);
    }

    #[test]
    fn test_turn_on() {
        let x = turn_on(0, u64::max_value());
        assert_eq!(u64::max_value(), x);

        let x = turn_on(u64::max_value(), u64::max_value());
        assert_eq!(u64::max_value(), x);
    }

    #[test]
    fn test_turn_off() {
        let x = turn_off(0, u64::max_value());
        assert_eq!(0, x);

        let x = turn_off(u64::max_value(), u64::max_value());
        assert_eq!(0, x);
    }
}
