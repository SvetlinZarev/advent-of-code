pub fn parse_csv_as_usize(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn parse_csv_as_u32(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn parse_lines_as_i32(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn parse_lines_as_u64(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|l| l.trim())
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn parse_lines_as_usize(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| l.trim())
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}
