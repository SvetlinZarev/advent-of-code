use std::collections::VecDeque;

pub fn solve(a: &[usize], b: &[usize]) -> usize {
    let mut first = VecDeque::with_capacity(a.len() + b.len());
    let mut second = VecDeque::with_capacity(a.len() + b.len());

    a.iter().copied().for_each(|v| first.push_back(v));
    b.iter().copied().for_each(|v| second.push_back(v));

    while !first.is_empty() && !second.is_empty() {
        let x = first.pop_front().unwrap();
        let y = second.pop_front().unwrap();

        if x > y {
            first.push_back(x);
            first.push_back(y);
        } else {
            second.push_back(y);
            second.push_back(x);
        }
    }

    first
        .iter()
        .rev()
        .chain(second.iter().rev())
        .copied()
        .enumerate()
        .fold(0, |acc, (idx, v)| acc + (idx + 1) * v)
}
