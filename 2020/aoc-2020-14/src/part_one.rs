use std::collections::HashMap;

use crate::{Mask, OpCode};

pub fn solve(input: &[OpCode]) -> u64 {
    let mut memory = HashMap::new();

    let mut mask = Mask::no_op();
    for opc in input.iter().copied() {
        match opc {
            OpCode::Mem(m) => {
                let value = (m.value & mask.and) | mask.or;
                memory.insert(m.address, value);
            }
            OpCode::Mask(m) => mask = m,
        }
    }

    memory.values().sum()
}
