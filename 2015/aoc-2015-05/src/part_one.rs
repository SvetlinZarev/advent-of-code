pub fn solve(input: &str) -> usize {
    input.lines().filter(|&s| is_nice(s)).count()
}

fn is_nice(string: &str) -> bool {
    let mut has_double_letter = false;
    let mut prev = 0;
    let mut vowels = 0;

    for ch in string.as_bytes().iter().copied() {
        if !has_double_letter {
            if prev == ch {
                has_double_letter = true;
            }
        }

        match ch {
            b'a' => {
                vowels += 1;
            }

            b'b' => {
                if prev == b'a' {
                    return false;
                }
            }

            b'd' => {
                if prev == b'c' {
                    return false;
                }
            }

            b'e' => {
                vowels += 1;
            }

            b'i' => {
                vowels += 1;
            }

            b'o' => {
                vowels += 1;
            }

            b'q' => {
                if prev == b'p' {
                    return false;
                }
            }

            b'u' => {
                vowels += 1;
            }

            b'y' => {
                if prev == b'x' {
                    return false;
                }
            }

            _ => {}
        }

        prev = ch;
    }

    return has_double_letter & (vowels >= 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        assert_eq!(true, is_nice("ugknbfddgicrmopn"));
        assert_eq!(true, is_nice("aaa"));

        assert_eq!(false, is_nice("jchzalrnumimnmhp"));
        assert_eq!(false, is_nice("haegwjzuvuyypxyu"));
        assert_eq!(false, is_nice("dvszwmarrgswjxmb"));
    }
}
