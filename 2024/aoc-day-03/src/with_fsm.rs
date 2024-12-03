#[derive(Copy, Clone)]
enum S1 {
    Nothing,
    M,
    U,
    L,
    OB,
    D1,
    C,
    D2,
}

pub fn part_one_v2(input: &str) -> u64 {
    let mut state = S1::Nothing;

    let mut sum = 0;
    let mut a = 0;
    let mut b = 0;

    for ch in input.bytes() {
        state = match state {
            S1::Nothing => {
                if ch == b'm' {
                    S1::M
                } else {
                    S1::Nothing
                }
            }

            S1::M => {
                if ch == b'u' {
                    S1::U
                } else {
                    S1::Nothing
                }
            }

            S1::U => {
                if ch == b'l' {
                    S1::L
                } else {
                    S1::Nothing
                }
            }

            S1::L => {
                if ch == b'(' {
                    S1::OB
                } else {
                    S1::Nothing
                }
            }

            S1::OB => {
                if (b'0'..=b'9').contains(&ch) {
                    b = (ch - b'0') as u64;
                    S1::D1
                } else {
                    S1::Nothing
                }
            }

            S1::D1 => {
                if (b'0'..=b'9').contains(&ch) {
                    b *= 10;
                    b += (ch - b'0') as u64;
                    S1::D1
                } else if ch == b',' {
                    S1::C
                } else {
                    S1::Nothing
                }
            }

            S1::C => {
                if (b'0'..=b'9').contains(&ch) {
                    a = b;

                    b = (ch - b'0') as u64;
                    S1::D2
                } else {
                    S1::Nothing
                }
            }

            S1::D2 => {
                if (b'0'..=b'9').contains(&ch) {
                    b *= 10;
                    b += (ch - b'0') as u64;
                    S1::D2
                } else {
                    if ch == b')' {
                        sum += a * b;
                    }

                    S1::Nothing
                }
            }
        }
    }

    sum
}

#[derive(Copy, Clone)]
enum S2 {
    Enabled,
    Disabled,
    DoD,    // DO/DON'T(): D
    DoO,    // DO/DON'T(): O
    DoOB,   // DO(): (
    DontN,  // DON'T(): N
    DontAP, // DON'T(): '
    DontT,  // DON'T(): T
    DontOB, // DON'T(): (
    M,      // MUL: M
    U,      // MUL: U
    L,      // MUL: L
    OB,     // MUL opening bracket
    D1,     // first number
    C,      // comma
    D2,     // second dumber
}

pub fn part_two_v2(input: &str) -> u64 {
    let mut state = S2::Enabled;

    let mut sum = 0;
    let mut enabled = true;
    let mut a = 0;
    let mut b = 0;

    for ch in input.bytes() {
        state = match state {
            S2::Enabled => {
                if ch == b'm' {
                    S2::M
                } else if ch == b'd' {
                    S2::DoD
                } else {
                    S2::Enabled
                }
            }

            S2::M => {
                if ch == b'u' {
                    S2::U
                } else {
                    S2::Enabled
                }
            }

            S2::U => {
                if ch == b'l' {
                    S2::L
                } else {
                    S2::Enabled
                }
            }

            S2::L => {
                if ch == b'(' {
                    S2::OB
                } else {
                    S2::Enabled
                }
            }

            S2::OB => {
                if (b'0'..=b'9').contains(&ch) {
                    b = (ch - b'0') as u64;
                    S2::D1
                } else {
                    S2::Enabled
                }
            }

            S2::D1 => {
                if (b'0'..=b'9').contains(&ch) {
                    b *= 10;
                    b += (ch - b'0') as u64;
                    S2::D1
                } else if ch == b',' {
                    S2::C
                } else {
                    S2::Enabled
                }
            }

            S2::C => {
                if (b'0'..=b'9').contains(&ch) {
                    a = b;

                    b = (ch - b'0') as u64;
                    S2::D2
                } else {
                    S2::Enabled
                }
            }

            S2::D2 => {
                if (b'0'..=b'9').contains(&ch) {
                    b *= 10;
                    b += (ch - b'0') as u64;
                    S2::D2
                } else {
                    if ch == b')' {
                        sum += a * b;
                    }

                    S2::Enabled
                }
            }

            S2::DoD => {
                if ch == b'o' {
                    S2::DoO
                } else {
                    if enabled { S2::Enabled } else { S2::Disabled }
                }
            }
            S2::DoO => {
                if ch == b'(' {
                    S2::DoOB
                } else if ch == b'n' {
                    S2::DontN
                } else {
                    if enabled { S2::Enabled } else { S2::Disabled }
                }
            }

            S2::DoOB => {
                if ch == b')' {
                    S2::Enabled
                } else {
                    if enabled { S2::Enabled } else { S2::Disabled }
                }
            }

            S2::DontN => {
                if ch == b'\'' {
                    S2::DontAP
                } else {
                    if enabled { S2::Enabled } else { S2::Disabled }
                }
            }
            S2::DontAP => {
                if ch == b't' {
                    S2::DontT
                } else {
                    if enabled { S2::Enabled } else { S2::Disabled }
                }
            }
            S2::DontT => {
                if ch == b'(' {
                    S2::DontOB
                } else {
                    if enabled { S2::Enabled } else { S2::Disabled }
                }
            }
            S2::DontOB => {
                if ch == b')' {
                    enabled = false;
                    S2::Disabled
                } else {
                    if enabled { S2::Enabled } else { S2::Disabled }
                }
            }
            S2::Disabled => {
                if ch == b'd' {
                    S2::DoD
                } else {
                    if enabled { S2::Enabled } else { S2::Disabled }
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one_v2(&input);
        assert_eq!(175_615_763, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_v2(&input);
        assert_eq!(74_361_272, answer);
    }
}
