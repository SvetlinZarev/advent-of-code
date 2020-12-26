pub fn solve(input: &[u8]) -> isize {
    let mut floor = 0;

    for x in input.iter().copied() {
        match x {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => panic!("unexpected input: {}", x as char),
        }
    }

    floor
}
