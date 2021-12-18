use crate::Number;

pub(crate) fn split(number: &mut Number) -> bool {
    match number {
        Number::Value(v) => {
            if *v >= 10 {
                let l = *v / 2;
                let r = *v - l;

                *number = Number::Tuple(Box::new((Number::Value(l), Number::Value(r))));
                return true;
            }
            false
        }

        Number::Tuple(t) => {
            if split(&mut t.0) {
                return true;
            }

            split(&mut t.1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_split_left_even() {
        let mut num = Number::from_str("[10,1]").unwrap();
        let expected = Number::from_str("[[5,5],1]").unwrap();

        let ok = split(&mut num);
        assert!(ok, "The number should have been split!");
        assert_eq!(expected, num);
    }

    #[test]
    fn test_split_right_even() {
        let mut num = Number::from_str("[1,10]").unwrap();
        let expected = Number::from_str("[1,[5,5]]").unwrap();

        let ok = split(&mut num);
        assert!(ok, "The number should have been split!");
        assert_eq!(expected, num);
    }

    #[test]
    fn test_split_left_odd() {
        let mut num = Number::from_str("[11,1]").unwrap();
        let expected = Number::from_str("[[5,6],1]").unwrap();

        let ok = split(&mut num);
        assert!(ok, "The number should have been split!");
        assert_eq!(expected, num);
    }

    #[test]
    fn test_split_right_odd() {
        let mut num = Number::from_str("[1,11]").unwrap();
        let expected = Number::from_str("[1,[5,6]]").unwrap();

        let ok = split(&mut num);
        assert!(ok, "The number should have been split!");
        assert_eq!(expected, num);
    }
}
