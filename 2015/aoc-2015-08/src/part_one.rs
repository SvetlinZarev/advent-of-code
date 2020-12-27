pub fn solve(input: &str) -> usize {
    let mut in_mem = 0;
    let mut whitespace = 0;
    let mut state = State::Start;

    // Assume ASCII: check the input byte by byte
    for (idx, ch) in input.as_bytes().iter().copied().enumerate() {
        match state {
            State::Start => {
                if ch == b'"' {
                    state = State::String;
                    continue;
                }

                panic!("unexpected symbol at index {}: {}", idx, ch as char);
            }
            State::String => {
                if ch == b'"' {
                    state = State::StringEnd;
                    continue;
                }

                if ch == b'\\' {
                    state = State::EscapeChar;
                    continue;
                }

                if ch == b'\n' {
                    panic!("unclosed string at index {}: ", idx);
                }

                in_mem += 1;
            }
            State::StringEnd => {
                if ch == b'\n' {
                    state = State::Start;
                    whitespace += 1;
                    continue;
                }

                panic!("unexpected character at index {}: {}", idx, ch as char);
            }
            State::EscapeChar => {
                if ch == b'\\' || ch == b'"' {
                    in_mem += 1;
                    state = State::String;
                    continue;
                }

                if ch == b'x' {
                    state = State::HexCh1;
                    continue;
                }

                panic!("unexpected symbol at index {}: {}", idx, ch as char);
            }

            State::HexCh1 => {
                if (b'a'..=b'f').contains(&ch) || (b'0'..=b'9').contains(&ch) {
                    state = State::HexCh2;
                    continue;
                }

                panic!("unexpected symbol at index {}: {}", idx, ch as char);
            }
            State::HexCh2 => {
                if (b'a'..=b'f').contains(&ch) || (b'0'..=b'9').contains(&ch) {
                    state = State::String;
                    in_mem += 1;
                    continue;
                }

                panic!("unexpected symbol at index {}: {}", idx, ch as char);
            }
        }
    }

    // assumes ASCII
    input.len() - in_mem - whitespace
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Start,
    String,
    StringEnd,
    EscapeChar,
    HexCh1,
    HexCh2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_empty_str() {
        let cnt = solve(r#""""#);
        assert_eq!(2, cnt);
    }

    #[test]
    fn test_sample_simple_str() {
        let cnt = solve(r#""abc""#);
        assert_eq!(2, cnt);
    }

    #[test]
    fn test_sample_escaped_quote() {
        let cnt = solve(r#""aaa\"aaa""#);
        assert_eq!(3, cnt);
    }

    #[test]
    fn test_sample_hex_escape() {
        let cnt = solve(r#""\x27""#);
        assert_eq!(5, cnt);
    }
}
