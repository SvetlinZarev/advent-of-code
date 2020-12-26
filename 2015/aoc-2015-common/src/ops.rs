#[macro_export]
macro_rules! min {
    ($x:expr) => ($x);
    ($x:expr, $($xs:expr),+) => {{
        std::cmp::min($x, min!($($xs),+))
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_min() {
        assert_eq!(3, min!(3));
        assert_eq!(3, min!(5, 3));
        assert_eq!(3, min!(3, 5));
        assert_eq!(3, min!(9, 3, 5));
        assert_eq!(3, min!(5, 8, 3, 9));
    }
}
