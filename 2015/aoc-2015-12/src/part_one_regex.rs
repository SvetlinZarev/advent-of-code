use regex::Regex;

const REGEX_NUMBER: &str = r#"-?\d+"#;

pub fn solve(input: &str) -> i64 {
    let pattern = Regex::new(REGEX_NUMBER).unwrap();

    let mut sum = 0;
    for found in pattern.find_iter(input) {
        let number: i64 = found.as_str().parse().unwrap();
        sum += number;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_positive_numbers() {
        let pattern = Regex::new(REGEX_NUMBER).unwrap();

        let found = pattern.find("123456789").unwrap();
        assert_eq!("123456789", found.as_str());

        let found = pattern.find("test: 123456789").unwrap();
        assert_eq!("123456789", found.as_str());
    }

    #[test]
    fn test_regex_negative_numbers() {
        let pattern = Regex::new(REGEX_NUMBER).unwrap();

        let found = pattern.find("-123456789").unwrap();
        assert_eq!("-123456789", found.as_str());

        let found = pattern.find("test: -123456789").unwrap();
        assert_eq!("-123456789", found.as_str());
    }
}
