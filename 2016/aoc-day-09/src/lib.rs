const MARKER_START: u8 = b'(';
const MARKER_X: u8 = b'x';
const MARKER_END: u8 = b')';

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Character,
    MarkerLen,
    MarkerRep,
}

pub fn part_one(input: impl AsRef<[u8]>) -> u64 {
    let input = input.as_ref();
    if input.is_empty() {
        return 0;
    }

    let mut lenght = 0u64;
    let mut index = 0;

    let mut marker_len = 0;
    let mut marker_rep = 0;

    let mut state = State::Character;
    while index < input.len() {
        match state {
            State::Character => {
                if input[index] == MARKER_START {
                    state = State::MarkerLen;
                    marker_len = 0;
                    marker_rep = 0;
                } else {
                    lenght += 1;
                }
            }

            State::MarkerLen => {
                if input[index].is_ascii_digit() {
                    marker_len *= 10;
                    marker_len += (input[index] - b'0') as usize;
                } else if input[index] == MARKER_X {
                    assert!(marker_len > 0);
                    state = State::MarkerRep;
                } else {
                    panic!(
                        "unexpected input at index {}: {}",
                        index, input[index] as char
                    );
                }
            }

            State::MarkerRep => {
                if input[index].is_ascii_digit() {
                    marker_rep *= 10;
                    marker_rep += (input[index] - b'0') as u64;
                } else if input[index] == MARKER_END {
                    assert!(marker_rep > 0);
                    lenght += (marker_len as u64) * marker_rep;
                    index += marker_len;
                    state = State::Character;
                } else {
                    panic!(
                        "unexpected input at index {}: {}",
                        index, input[index] as char
                    );
                }
            }
        }

        index += 1;
    }

    lenght
}

pub fn part_two(input: impl AsRef<[u8]>) -> u64 {
    let input = input.as_ref();
    if input.is_empty() {
        return 0;
    }

    let mut lenght = 0u64;
    let mut index = 0;

    let mut marker_len = 0;
    let mut marker_rep = 0u64;

    let mut state = State::Character;
    while index < input.len() {
        match state {
            State::Character => {
                if input[index] == MARKER_START {
                    state = State::MarkerLen;
                    marker_len = 0;
                    marker_rep = 0;
                } else {
                    lenght += 1;
                }
            }
            State::MarkerLen => {
                if input[index].is_ascii_digit() {
                    marker_len *= 10;
                    marker_len += (input[index] - b'0') as usize;
                } else if input[index] == MARKER_X {
                    assert!(marker_len > 0);
                    state = State::MarkerRep;
                } else {
                    panic!(
                        "unexpected input at index {}: {}",
                        index, input[index] as char
                    );
                }
            }
            State::MarkerRep => {
                if input[index].is_ascii_digit() {
                    marker_rep *= 10;
                    marker_rep += (input[index] - b'0') as u64;
                } else if input[index] == MARKER_END {
                    assert!(marker_rep > 0);
                    let child_len = part_two(&input[index + 1..index + 1 + marker_len]);
                    lenght += marker_rep * child_len;
                    index += marker_len;
                    state = State::Character;
                } else {
                    panic!(
                        "unexpected input at index {}: {}",
                        index, input[index] as char
                    );
                }
            }
        }

        index += 1;
    }

    lenght
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(input.trim_end());
        assert_eq!(120765, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(input.trim_end());
        assert_eq!(11658395076, answer);
    }
}
