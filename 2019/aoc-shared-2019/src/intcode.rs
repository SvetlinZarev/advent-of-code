pub const OP_ADD: usize = 1;
pub const OP_MUL: usize = 2;
pub const OP_HLT: usize = 99;

pub const OPERANDS: [usize; 3] = [usize::MAX, 3, 3];

#[inline(always)]
pub fn instruction(mem: &[usize], idx: usize) -> usize {
    mem[idx]
}

#[inline(always)]
pub fn addr(mem: &[usize], idx: usize) -> usize {
    mem[idx]
}
