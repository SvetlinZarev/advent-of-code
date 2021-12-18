use crate::{Number, Numeric};

pub fn part_two(input: &[Number]) -> Numeric {
    assert!(!input.is_empty());

    let mut best = 0;

    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }

            let mut num = input[i].clone();
            num.add(input[j].clone());
            num.reduce();
            best = best.max(num.magnitude());
        }
    }

    best
}
