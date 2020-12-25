pub fn solve(input: &[u64], expected: u64) -> Option<u64> {
    let mut from = 0;
    let mut to = 1;

    let mut sum = input[from] + input[to];
    while to < input.len() {
        if sum > expected {
            sum -= input[from];
            from += 1;
        }

        if sum < expected {
            to += 1;
            sum += input[to];
        }

        if sum == expected && from < to {
            let min = *input[from..=to].iter().min().unwrap();
            let max = *input[from..=to].iter().max().unwrap();
            return Some(min + max);
        }
    }

    None
}
