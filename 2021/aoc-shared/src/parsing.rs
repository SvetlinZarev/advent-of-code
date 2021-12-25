use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_line_delimited<I, R>(input: I) -> Vec<R>
where
    I: AsRef<str>,
    R: FromStr,
    <R as FromStr>::Err: Debug,
{
    input
        .as_ref()
        .lines()
        .map(|l| l.trim())
        .filter(|&l| !l.is_empty())
        .map(|l| l.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn parse_csv<I, R>(input: I) -> Vec<R>
where
    I: AsRef<str>,
    R: FromStr,
    <R as FromStr>::Err: Debug,
{
    input
        .as_ref()
        .split(',')
        .map(|v| v.trim())
        .filter(|&v| !v.is_empty())
        .map(|v| v.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn parse_numeric_grid<I: AsRef<str>, T: From<u8> + Copy>(input: I) -> Vec<Vec<T>> {
    input
        .as_ref()
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|&b| (b - b'0').try_into().unwrap())
                .collect()
        })
        .collect()
}

pub fn parse_u8_grid<I: AsRef<str>>(input: I) -> Vec<Vec<u8>> {
    input
        .as_ref()
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect()
}

pub fn parse_u8_numeric_grid<I: AsRef<str>>(input: I) -> Vec<Vec<u8>> {
    let mut grid = parse_u8_grid(input);
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            grid[r][c] -= b'0';
        }
    }
    grid
}

pub fn parse_i8_numeric_grid<I: AsRef<str>>(input: I) -> Vec<Vec<i8>> {
    input
        .as_ref()
        .lines()
        .map(|l| l.as_bytes().iter().map(|&b| (b - b'0') as i8).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_delimited() {
        let lines = "1\n2\r\n3";
        let parsed = parse_line_delimited(lines);
        assert_eq!(&[1u32, 2, 3], parsed.as_slice());
    }

    #[test]
    fn test_parse_line_delimited_with_non_trimmed_input() {
        let lines = "1 \n 2\r\n 3 ";
        let parsed = parse_line_delimited(lines);
        assert_eq!(&[1u32, 2, 3], parsed.as_slice());
    }

    #[test]
    fn test_parse_line_delimited_skip_empty() {
        let lines = "1\n\n\n\n2\n\n\r\n\r\n3";
        let parsed = parse_line_delimited(lines);
        assert_eq!(&[1u32, 2, 3], parsed.as_slice());
    }
}
