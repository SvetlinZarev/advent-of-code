use aoc_shared::grid::Direction;

#[inline(always)]
pub fn step(
    input: &[u8],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    d: Direction,
    s: usize,
) -> Option<(usize, usize, u16)> {
    let (mut rx, mut cx) = (r as isize, c as isize);
    let mut cost = 0;

    for _ in 0..s {
        let (nr, nc) = d.apply_signed(rx, cx);
        if nr < 0 || nc < 0 || nr >= rows as isize || nc >= (cols - 1) as isize {
            return None;
        }

        (rx, cx) = (nr, nc);
        cost += (input[rx as usize * cols + cx as usize] - b'0') as u16;
    }

    Some((rx as usize, cx as usize, cost))
}
