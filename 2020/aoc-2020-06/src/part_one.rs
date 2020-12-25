#[inline(never)]
pub fn solve_iter(input: &str) -> u32 {
    super::solve_iter(input, 0, fold_fn)
}

#[inline(never)]
pub fn solve_loops(input: &[u8]) -> u32 {
    super::solve_loop(input, 0, fold_fn)
}

#[inline(always)]
fn fold_fn(acc: u32, value: u32) -> u32 {
    acc | value
}
