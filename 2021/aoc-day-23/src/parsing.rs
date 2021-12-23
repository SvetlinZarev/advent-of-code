use crate::{Hall, Rooms, FREE_SPOT, HALL_LEN};

pub fn parse_input<S: AsRef<str>, const N: usize>(input: S) -> (Rooms<N>, Hall) {
    let input = input.as_ref();
    let lines = input.lines().collect::<Vec<_>>();

    let mut hall = [0u8; HALL_LEN];
    lines[1]
        .as_bytes()
        .iter()
        .copied()
        .skip(1)
        .take(HALL_LEN)
        .enumerate()
        .for_each(|(idx, v)| hall[idx] = if v == b'.' { FREE_SPOT } else { v - b'A' });

    let mut rooms = [[0u8; N]; 4];

    for pos_in_room in (0..N).rev() {
        let line = lines[2 + N - pos_in_room - 1];
        let mut iter = line
            .as_bytes()
            .iter()
            .copied()
            .filter(|&ch| ch.is_ascii_uppercase() || ch == b'.');

        for room_idx in 0..rooms.len() {
            let value = iter.next().unwrap();

            rooms[room_idx][pos_in_room] = if value == b'.' {
                FREE_SPOT
            } else {
                value - b'A'
            }
        }
    }

    (rooms, hall)
}
