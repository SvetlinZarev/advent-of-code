pub fn solve(input: &str) -> usize {
    let mut new_lines = 0;
    let mut encoded_len = 0;
    let mut state = State::Initial;

    for (idx, ch) in input.as_bytes().iter().copied().enumerate() {
        match state {
            State::Initial => {
                // strings should be encoded to start with a quote
                encoded_len += 1;

                if ch == b'"' {
                    // '"' =>'\"'
                    encoded_len += 2;
                    state = State::String;
                    continue;
                }

                panic!("unexpected symbol at index {}: {}", idx, ch as char);
            }
            State::String => {
                if ch == b'\\' {
                    state = State::Escaped;
                    continue;
                }

                if ch == b'"' {
                    // the final '"' becomes '\""'
                    encoded_len += 3;
                    state = State::End;
                    continue;
                }

                if ch == b'\n' {
                    panic!("unexpected symbol at index {}: {}", idx, ch as char);
                }

                encoded_len += 1;
            }

            State::End => {
                if ch == b'\n' {
                    new_lines += 1;
                    state = State::Initial;
                    continue;
                }

                panic!("unexpected symbol at index {}: {}", idx, ch as char);
            }
            State::Escaped => {
                if ch == b'"' {
                    // '\"' => '\\\"'
                    encoded_len += 4;
                    state = State::String;
                    continue;
                }

                if ch == b'\\' {
                    // '\\' => '\\\\'
                    encoded_len += 4;
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
                    encoded_len += 5;
                    state = State::String;
                    continue;
                }

                panic!("unexpected symbol at index {}: {}", idx, ch as char);
            }
        }
    }

    encoded_len + new_lines - input.len()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Initial,
    String,
    Escaped,
    HexCh1,
    HexCh2,
    End,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_empty_str() {
        let cnt = solve(r#""""#);
        assert_eq!(4, cnt);
    }

    #[test]
    fn test_sample_plain_str() {
        let cnt = solve(r#""abc""#);
        assert_eq!(4, cnt);
    }

    #[test]
    fn test_sample_escaped_quote() {
        let cnt = solve(r#""aaa\"aaa""#);
        assert_eq!(6, cnt);
    }

    #[test]
    fn test_sample_hex_escape() {
        let cnt = solve(r#""\x27""#);
        assert_eq!(5, cnt);
    }

    #[test]
    fn test_sample_hex_escape_slash() {
        let cnt = solve(r#""\\""#);
        assert_eq!(6, cnt);
    }

    #[test]
    fn test_sample_hex_escape_slashes() {
        let cnt = solve(r#""\\\""#);
        assert_eq!(6, cnt);
    }

    #[test]
    fn test_sample_complex() {
        let cnt = solve(r#""byc\x9dyxuafof\\\xa6uf\\axfozomj\\olh\x6a""#);
        assert_eq!(13, cnt);
    }
}
