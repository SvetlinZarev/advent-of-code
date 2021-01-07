use std::cmp::Ordering;

#[macro_export]
macro_rules! min {
    ($x:expr) => ($x);
    ($x:expr, $($xs:expr),+) => {{
        std::cmp::min($x, min!($($xs),+))
    }};
}

#[inline(always)]
pub fn min_by<T, F>(a: T, b: T, comparator: F) -> T
where
    T: Ord + Copy,
    F: Copy + Fn(T, T) -> Ordering,
{
    match comparator(a, b) {
        Ordering::Less => a,
        Ordering::Equal => a,
        Ordering::Greater => b,
    }
}

#[inline(always)]
pub fn max_by<T, F>(a: T, b: T, comparator: F) -> T
where
    T: Ord + Copy,
    F: Copy + Fn(T, T) -> Ordering,
{
    match comparator(a, b) {
        Ordering::Less => b,
        Ordering::Equal => a,
        Ordering::Greater => a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min() {
        assert_eq!(3, min!(3));
        assert_eq!(3, min!(5, 3));
        assert_eq!(3, min!(3, 5));
        assert_eq!(3, min!(9, 3, 5));
        assert_eq!(3, min!(5, 8, 3, 9));
    }

    #[test]
    fn test_min_by() {
        let a = 0;
        let b = u32::max_value();

        let x = min_by(a, b, |a, b| a.cmp(&b));
        assert_eq!(a, x);
    }

    #[test]
    fn test_max_by() {
        let a = 0;
        let b = u32::max_value();

        let x = max_by(a, b, |a, b| a.cmp(&b));
        assert_eq!(b, x);
    }
}
