use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Winner {
    First,
    Second,
}

pub fn solve(a: &[usize], b: &[usize]) -> usize {
    let mut first = VecDeque::with_capacity(a.len() + b.len());
    let mut second = VecDeque::with_capacity(a.len() + b.len());

    a.iter().copied().for_each(|v| first.push_back(v));
    b.iter().copied().for_each(|v| second.push_back(v));

    let result = match play(&mut first, &mut second) {
        Winner::First => &first,
        Winner::Second => &second,
    };

    result
        .iter()
        .rev()
        .copied()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + (i + 1) * v)
}

fn play(first: &mut VecDeque<usize>, second: &mut VecDeque<usize>) -> Winner {
    let mut hands = HashSet::new();

    while !first.is_empty() && !second.is_empty() {
        if !hands.insert((first.clone(), second.clone())) {
            return Winner::First;
        }

        let x = first.pop_front().unwrap();
        let y = second.pop_front().unwrap();

        if x <= first.len() && y <= second.len() {
            let mut first_copy = first.iter().copied().take(x).collect();
            let mut second_copy = second.iter().copied().take(y).collect();

            match play(&mut first_copy, &mut second_copy) {
                Winner::First => {
                    first.push_back(x);
                    first.push_back(y);
                }
                Winner::Second => {
                    second.push_back(y);
                    second.push_back(x);
                }
            }
        } else if x > y {
            first.push_back(x);
            first.push_back(y);
        } else {
            second.push_back(y);
            second.push_back(x);
        }
    }

    if first.is_empty() {
        Winner::Second
    } else {
        Winner::First
    }
}
