use std::collections::HashMap;

use crate::{Mask, OpCode};

const LIMIT_36_BITS: u64 = u64::max_value() >> (64 - 36);

pub fn solve(input: &[OpCode]) -> u64 {
    let mut memory = HashMap::new();

    let mut mask = Mask::no_op();
    for opc in input.iter().copied() {
        match opc {
            OpCode::Mem(m) => {
                // Apply bitwise AND with the floating mask
                // in order to set all floating bits to zero.
                // This will let us to use simple +1 to set them
                // one by one later
                let base_addr = (m.address | mask.or) & mask.floating();

                let mut floating = mask.floating();
                loop {
                    let floating_inverted = (!floating) & LIMIT_36_BITS;
                    let floating_address = base_addr | floating_inverted;
                    memory.insert(floating_address, m.value);

                    if floating & LIMIT_36_BITS == LIMIT_36_BITS {
                        break;
                    }

                    /*
                        An inverted mask looks like 111010,where the 0 bits
                        are the one we have to set. The 1 bits are not "floating",
                        thus they must not change!. A plus operation sets a
                        single bit:
                        ```
                        111010
                        +
                        000001
                        ---
                        111011.
                        ```
                        But if the bit was already one,it will cause an overflow
                        to the next bit:
                        ```
                        111011
                        +
                        000001
                        ---
                        111100
                        ```
                        Thus changing the mask semantics, because it set a
                        "non-floating" bit to zero, thus making it a "floating"
                        one.

                        As this will produce an incorrect result, we have to OR
                        the resulting sum with the original inverted mask, thus
                        restoring the "damaged" bits:

                        111100
                        OR
                        111010
                        ---
                        111110
                    */

                    floating += 1;
                    floating |= mask.floating();
                }
            }
            OpCode::Mask(m) => mask = m,
        }
    }

    memory.values().sum()
}
