pub fn solve(input: &[u8]) -> Option<usize> {
    let mut floor = 0;

    for (idx, x) in input.iter().copied().enumerate() {
        match x {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => panic!("unexpected input: {}", x as char),
        }

        if floor < 0 {
            return Some(idx + 1);
        }
    }

    None
}
