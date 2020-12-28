use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_csv_as_usize(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn parse_csv<I, R, E>(input: I) -> Vec<R>
where
    I: AsRef<str>,
    E: Debug,
    R: FromStr<Err = E>,
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

pub fn parse_line_delimited<I, R, E>(input: I) -> Vec<R>
where
    I: AsRef<str>,
    E: Debug,
    R: FromStr<Err = E>,
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
