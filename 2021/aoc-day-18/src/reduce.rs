use crate::explode::explode;
use crate::split::split;
use crate::Number;

pub(crate) fn reduce(number: &mut Number) {
    loop {
        let (exploded, _, _) = explode(number, 1);
        if exploded {
            continue;
        }

        if split(number) {
            continue;
        }

        break;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_reduce_1() {
        let expected = Number::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        let mut num = Number::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();

        reduce(&mut num);
        assert_eq!(expected, num);
    }
}
