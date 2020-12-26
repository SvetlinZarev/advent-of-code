use crate::{Action, Instruction};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

pub fn solve(instructions: &[Instruction]) -> usize {
    let mut grid = [[0u8; WIDTH]; HEIGHT];

    for instruction in instructions {
        for r in instruction.from.y..=instruction.to.y {
            for c in instruction.from.x..=instruction.to.x {
                let x = grid[r as usize][c as usize];
                grid[r as usize][c as usize] = match instruction.action {
                    Action::On => x + 1,
                    Action::Off => x.saturating_sub(1),
                    Action::Toggle => x + 2,
                }
            }
        }
    }

    grid.iter()
        .map(|r| r.iter())
        .flatten()
        .copied()
        .map(|b| b as usize)
        .sum()
}
