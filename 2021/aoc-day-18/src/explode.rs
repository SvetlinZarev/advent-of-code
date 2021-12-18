use crate::{Number, Numeric};

pub(crate) fn explode(
    number: &mut Number,
    depth: usize,
) -> (bool, Option<Numeric>, Option<Numeric>) {
    match number {
        Number::Value(_) => return (false, None, None),
        Number::Tuple(t) => {
            if depth == 5 {
                return match (&mut t.0, &mut t.1) {
                    (Number::Value(l), Number::Value(r)) => {
                        let l = *l;
                        let r = *r;

                        *number = Number::Value(0);
                        (true, Some(l), Some(r))
                    }
                    _ => panic!("Maximum depth exceeded!"),
                };
            }

            let (exploded, l, r) = explode(&mut t.0, depth + 1);
            if exploded {
                if let Some(value) = r {
                    add_to_left(&mut t.1, value);
                    return (true, l, None);
                }
                return (true, l, r);
            } else {
                let (exploded, l, r) = explode(&mut t.1, depth + 1);
                if exploded {
                    if let Some(value) = l {
                        add_to_right(&mut t.0, value);
                        return (true, None, r);
                    }
                    return (true, l, r);
                }

                return (false, None, None);
            }
        }
    }
}

fn add_to_left(n: &mut Number, v: Numeric) {
    match n {
        Number::Value(x) => *x += v,
        Number::Tuple(t) => add_to_left(&mut t.0, v),
    }
}

fn add_to_right(number: &mut Number, v: Numeric) {
    match number {
        Number::Value(x) => *x += v,
        Number::Tuple(t) => add_to_right(&mut t.1, v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_explode_no_left() {
        let expected = Number::from_str("[[[[0,9],2],3],4]").unwrap();
        let mut num = Number::from_str("[[[[[9,8],1],2],3],4]").unwrap();

        explode(&mut num, 1);
        assert_eq!(expected, num);
    }

    #[test]
    fn test_explode_no_right_1() {
        let expected = Number::from_str("[7,[6,[5,[7,0]]]]").unwrap();
        let mut num = Number::from_str("[7,[6,[5,[4,[3,2]]]]]").unwrap();

        explode(&mut num, 1);
        assert_eq!(expected, num);
    }

    #[test]
    fn test_explode_no_right_2() {
        let expected = Number::from_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").unwrap();
        let mut num = Number::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();

        explode(&mut num, 1);
        assert_eq!(expected, num);
    }

    #[test]
    fn test_explode_left_right_1() {
        let expected = Number::from_str("[[6,[5,[7,0]]],3]").unwrap();
        let mut num = Number::from_str("[[6,[5,[4,[3,2]]]],1]").unwrap();

        explode(&mut num, 1);
        assert_eq!(expected, num);
    }

    #[test]
    fn test_explode_left_right_2() {
        let expected = Number::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
        let mut num = Number::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();

        explode(&mut num, 1);
        assert_eq!(expected, num);
    }
}
