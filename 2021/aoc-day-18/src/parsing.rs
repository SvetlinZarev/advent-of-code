use crate::Number;
use std::str::FromStr;

impl FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') || !s.ends_with(']') {
            return Err(format!("Expecting '[' or ']': {:?}", s));
        }
        let s = &s[1..s.len() - 1];

        let (left, right) =
            if s.as_bytes()[0].is_ascii_digit() && s.as_bytes()[s.len() - 1].is_ascii_digit() {
                let (l, r) = s
                    .split_once(',')
                    .ok_or_else(|| format!("Expected ',' between digits: {:?}", s))?;
                let ln = l
                    .parse()
                    .map_err(|e| format!("Failed to parse left number: {:?} -> {:?}", e, s))?;

                let rn = r
                    .parse()
                    .map_err(|e| format!("Failed to parse right number: {:?} -> {:?}", e, s))?;

                (Number::Value(ln), Number::Value(rn))
            } else if s.as_bytes()[0].is_ascii_digit() {
                let (n, rest) = s
                    .split_once(',')
                    .ok_or_else(|| format!("Expected ',' after left digit: {:?}", s))?;

                let number = n
                    .parse()
                    .map_err(|e| format!("Failed to parse left number: {:?} -> {:?}", e, s))?;
                let remaining = Number::from_str(rest)?;

                (Number::Value(number), remaining)
            } else if s.as_bytes()[s.len() - 1].is_ascii_digit() {
                let (rest, n) = s
                    .rsplit_once(',')
                    .ok_or_else(|| format!("Expected ',' before right digit: {:?}", s))?;

                let number = n
                    .parse()
                    .map_err(|e| format!("Failed to parse right number: {:?} -> {:?}", e, s))?;
                let remaining = Number::from_str(rest)?;

                (remaining, Number::Value(number))
            } else {
                let mut brackets = 0i32;
                let mut split_pos = 0;
                for (idx, ch) in s.as_bytes().iter().enumerate() {
                    match ch {
                        b'[' => brackets += 1,
                        b']' => brackets -= 1,
                        _ => { /*do nothing*/ }
                    }

                    if brackets < 0 {
                        return Err(format!("Unbalanced brackets: {:?}", s));
                    }

                    if brackets == 0 {
                        split_pos = idx + 1;
                        break;
                    }
                }

                if brackets != 0 {
                    return Err(format!("Unbalanced brackets: {:?}", s));
                }

                let (left, right) = s.split_at(split_pos);
                let left = Number::from_str(left)?;
                let right = Number::from_str(&right[1..])?;

                (left, right)
            };

        Ok(Number::Tuple(Box::new((left, right))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_numeric_pair() {
        let actual = Number::from_str("[1,2]").unwrap();
        let expected = Number::Tuple(Box::new((Number::Value(1), Number::Value(2))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parsing_value_and_tuple() {
        let actual = Number::from_str("[1,[2,3]]").unwrap();
        let expected = Number::Tuple(Box::new((
            Number::Value(1),
            Number::Tuple(Box::new((Number::Value(2), Number::Value(3)))),
        )));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parsing_tuple_and_value() {
        let actual = Number::from_str("[[1,2],3]").unwrap();
        let expected = Number::Tuple(Box::new((
            Number::Tuple(Box::new((Number::Value(1), Number::Value(2)))),
            Number::Value(3),
        )));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parsing_tuple_and_tuple() {
        let actual = Number::from_str("[[1,2],[3,4]]").unwrap();
        let expected = Number::Tuple(Box::new((
            Number::Tuple(Box::new((Number::Value(1), Number::Value(2)))),
            Number::Tuple(Box::new((Number::Value(3), Number::Value(4)))),
        )));

        assert_eq!(expected, actual);
    }
}
