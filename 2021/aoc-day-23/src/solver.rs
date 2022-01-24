use crate::{Hall, Rooms, FREE_SPOT, HALL_LEN, ROOMS};
use ahash::AHashMap;

const COSTS: [u64; ROOMS] = [1, 10, 100, 1000];

// Distances to each free space in the hall from each room
// Each row contains the distances from this room. A distance
// of 0 means that it's forbidden to stand at that place
const HALL_DIST: [[u8; HALL_LEN]; ROOMS] = [
    [2, 1, 0, 1, 0, 3, 0, 5, 0, 7, 8], // distance from room 1
    [4, 3, 0, 1, 0, 1, 0, 3, 0, 5, 6], // distance from room 2
    [6, 5, 0, 3, 0, 1, 0, 1, 0, 3, 4], // distance from room 3
    [8, 7, 0, 5, 0, 3, 0, 1, 0, 1, 2], // distance from room 4
];

type Map<K, V> = AHashMap<K, V>;

pub(crate) fn solve<const N: usize>(input: Rooms<N>, hall: Hall) -> u64 {
    let mut cache = Map::default();
    {
        let end_game_hall = [FREE_SPOT; HALL_LEN];
        let mut end_game_rooms = [[0u8; N]; ROOMS];
        for (room_idx, room) in end_game_rooms.iter_mut().enumerate() {
            room.iter_mut().for_each(|v| *v = room_idx as u8);
        }
        cache.insert((end_game_rooms, end_game_hall), Some(0));
    }

    round(input, hall, &mut cache).unwrap()
}

fn round<const N: usize>(
    rooms: Rooms<N>,
    hall: Hall,
    cache: &mut Map<(Rooms<N>, Hall), Option<u64>>,
) -> Option<u64> {
    if let Some(&cost) = cache.get(&(rooms, hall)) {
        return cost;
    }

    let mut min_cost: Option<u64> = None;

    // move from room to hall
    for (room_idx, room) in rooms.iter().copied().enumerate() {
        for occ_idx in (0..room.len()).rev() {
            if room[occ_idx] == FREE_SPOT {
                continue;
            }

            if should_move_to_hall(room, occ_idx, room_idx as u8) {
                // to the left of room
                for hall_idx in (0..room_to_hall_idx(room_idx)).rev() {
                    if HALL_DIST[room_idx][hall_idx] == 0 {
                        continue;
                    }

                    if hall[hall_idx] == FREE_SPOT {
                        min_cost =
                            move_to_hall(rooms, hall, cache, min_cost, room_idx, occ_idx, hall_idx);
                        continue;
                    }

                    break;
                }

                // to the right of room
                for hall_idx in room_to_hall_idx(room_idx) + 1..hall.len() {
                    if HALL_DIST[room_idx][hall_idx] == 0 {
                        continue;
                    }

                    if hall[hall_idx] == FREE_SPOT {
                        min_cost =
                            move_to_hall(rooms, hall, cache, min_cost, room_idx, occ_idx, hall_idx);
                        continue;
                    }

                    break;
                }
            }

            break;
        }
    }

    // move from hall to room
    for room_idx in 0..rooms.len() {
        // from left of the room
        let mut left_side = None;
        for hall_idx in (0..room_to_hall_idx(room_idx)).rev() {
            if hall[hall_idx] == FREE_SPOT {
                continue;
            }

            if hall[hall_idx] == room_idx as u8 {
                left_side = Some(hall_idx);
            }

            break;
        }
        if let Some(hall_idx) = left_side {
            min_cost = move_to_room(rooms, hall, cache, min_cost, room_idx, hall_idx);
        }

        // from right of the room
        let mut right_side = None;
        for hall_idx in room_to_hall_idx(room_idx) + 1..hall.len() {
            if hall[hall_idx] == FREE_SPOT {
                continue;
            }

            if hall[hall_idx] == room_idx as u8 {
                right_side = Some(hall_idx);
            }

            break;
        }
        if let Some(hall_idx) = right_side {
            min_cost = move_to_room(rooms, hall, cache, min_cost, room_idx, hall_idx);
        }
    }

    cache.insert((rooms, hall), min_cost);
    min_cost
}

fn move_to_hall<const N: usize>(
    rooms: Rooms<N>,
    hall: Hall,
    cache: &mut Map<(Rooms<N>, Hall), Option<u64>>,
    min_cost: Option<u64>,
    room_idx: usize,
    occ_idx: usize,
    hall_idx: usize,
) -> Option<u64> {
    let occupant = rooms[room_idx][occ_idx];

    let steps_from_front_to_pos = HALL_DIST[room_idx][hall_idx] as u64;
    let steps_from_room_to_front = (N - occ_idx) as u64;
    let cost = COSTS[occupant as usize] * (steps_from_room_to_front + steps_from_front_to_pos);

    let mut rooms_next = rooms.clone();
    rooms_next[room_idx][occ_idx] = FREE_SPOT;

    let mut hall_next = hall.clone();
    hall_next[hall_idx] = occupant;

    match round(rooms_next, hall_next, cache) {
        None => min_cost,
        Some(cst) => {
            let total = cst + cost;
            match min_cost {
                None => Some(total),
                Some(best) => Some(best.min(total)),
            }
        }
    }
}

fn move_to_room<const N: usize>(
    rooms: Rooms<N>,
    hall: Hall,
    cache: &mut Map<(Rooms<N>, Hall), Option<u64>>,
    min_cost: Option<u64>,
    room_idx: usize,
    hall_idx: usize,
) -> Option<u64> {
    if let Some(idx) = find_room_idx(rooms, room_idx) {
        let occupant = hall[hall_idx];

        let steps_from_hall_to_front = HALL_DIST[room_idx][hall_idx] as u64;
        let steps_from_front_to_room = (N - idx) as u64;
        let cost = COSTS[occupant as usize] * (steps_from_hall_to_front + steps_from_front_to_room);

        let mut rooms_next = rooms.clone();
        rooms_next[room_idx][idx] = occupant;

        let mut hall_next = hall.clone();
        hall_next[hall_idx] = FREE_SPOT;

        return match round(rooms_next, hall_next, cache) {
            None => min_cost,
            Some(cst) => {
                let total = cst + cost;
                match min_cost {
                    None => Some(total),
                    Some(best) => Some(best.min(total)),
                }
            }
        };
    }

    min_cost
}

fn room_to_hall_idx(room_idx: usize) -> usize {
    (room_idx + 1) * 2
}

fn should_move_to_hall<const N: usize>(room: [u8; N], idx: usize, expected: u8) -> bool {
    for pos in (0..=idx).rev() {
        if room[pos] != expected {
            return true;
        }
    }

    false
}

fn find_room_idx<const N: usize>(rooms: Rooms<N>, room_idx: usize) -> Option<usize> {
    let mut position = rooms[room_idx].len();
    for idx in (0..rooms[room_idx].len()).rev() {
        if rooms[room_idx][idx] != FREE_SPOT {
            break;
        }
        position = idx;
    }

    for idx in (0..position).rev() {
        if rooms[room_idx][idx] != room_idx as u8 {
            return None;
        }
    }

    if position < rooms[room_idx].len() {
        return Some(position);
    }

    None
}
