use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    input.lines().filter(|&s| is_nice(s)).count()
}

fn is_nice(string: &str) -> bool {
    let mut pairs_cache = HashSet::new();

    // previous letters cache
    let mut o = 0;
    let mut a = 0;
    let mut b = 0;

    let mut has_xox = false;
    let mut has_pair = false;

    for ch in string.as_bytes().iter().copied() {
        if !has_xox {
            if a == ch {
                has_xox = true;
            }
        }

        // no need to do additional checks if we already have a repeating pair
        if !has_pair {
            if !(ch == a && ch == b) || (ch == a && ch == b && ch == o) {
                has_pair = !pairs_cache.insert((b, ch));
            }
        }

        if has_xox & has_pair {
            return true;
        }

        o = a;
        a = b;
        b = ch;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::part_two::is_nice;

    #[test]
    fn test_sample_input() {
        assert_eq!(true, is_nice("qjhvhtzxzqqjkmpb"), "1");
        assert_eq!(true, is_nice("xxyxx"), "2");

        assert_eq!(false, is_nice("uurcxstgmygtbstg"), "3");
        assert_eq!(false, is_nice("ieodomkazucvgmuy"), "4");
    }
}
