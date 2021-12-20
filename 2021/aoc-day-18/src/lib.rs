use std::fmt::{Display, Formatter};

mod explode;
mod parsing;
mod part_one;
mod part_two;
mod reduce;
mod split;

pub use part_one::part_one;
pub use part_two::part_two;

type Numeric = u32;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Number {
    Value(Numeric),
    Tuple(Box<(Number, Number)>),
}

impl Number {
    pub fn magnitude(&self) -> Numeric {
        match self {
            Number::Value(x) => *x,
            Number::Tuple(tuple) => tuple.0.magnitude() * 3 + tuple.1.magnitude() * 2,
        }
    }

    pub fn reduce(&mut self) {
        reduce::reduce(self);
    }

    pub fn add(&mut self, n: Number) {
        let mut temp = Number::Value(0);
        std::mem::swap(self, &mut temp);

        *self = Number::Tuple(Box::new((temp, n)))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Value(v) => write!(f, "{}", *v)?,
            Number::Tuple(t) => write!(f, "[{},{}]", &t.0, &t.1)?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_magnitude_simple() {
        let num = Number::from_str("[1,2]").unwrap();
        assert_eq!(7, num.magnitude());
    }

    #[test]
    fn test_magnitude_complex() {
        let num = Number::from_str("[[1,2],[3,4]]").unwrap();
        assert_eq!(55, num.magnitude());
    }

    #[test]
    fn test_add_1() {
        let exp = Number::from_str("[[1,1],[2,2]]").unwrap();
        let mut a = Number::from_str("[1,1]").unwrap();
        let b = Number::from_str("[2,2]").unwrap();

        a.add(b);
        assert_eq!(exp, a);
    }
}
