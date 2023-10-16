#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Initial,
    InGroup,
    InGarbage,
    CancelNext,
    Error,
}

pub fn part_one_and_two(input: &str) -> (i32, i32) {
    let mut state = State::Initial;
    let mut depth = 0;
    let mut group_score = 0;
    let mut garbage_size = 0;

    for ch in input.bytes() {
        state = match state {
            State::Initial => match ch {
                b'{' => {
                    depth += 1;
                    group_score += depth;
                    State::InGroup
                }
                _ => State::Error,
            },
            State::InGroup => match ch {
                b'{' => {
                    depth += 1;
                    group_score += depth;
                    State::InGroup
                }
                b'}' => {
                    depth -= 1;
                    State::InGroup
                }
                b'<' => State::InGarbage,
                b',' => State::InGroup,
                _ => State::Error,
            },

            State::InGarbage => match ch {
                b'>' => State::InGroup,
                b'!' => State::CancelNext,
                _ => {
                    garbage_size += 1;
                    State::InGarbage
                }
            },
            State::CancelNext => State::InGarbage,
            State::Error => panic!("something bad happened"),
        }
    }

    assert_ne!(State::Error, state);
    (group_score, garbage_size)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one_and_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one_and_two(input.trim());
        assert_eq!((14421, 6817), answer);
    }
}
