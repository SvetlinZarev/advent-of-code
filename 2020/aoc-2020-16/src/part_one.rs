use std::iter;

use crate::Input;

pub fn solve(input: &mut Input) -> u32 {
    let mut error_rate = 0;

    let ranges = input
        .fields
        .iter()
        .map(|r| iter::once(r.1.first.clone()).chain(iter::once(r.1.second.clone())))
        .flatten()
        .collect::<Vec<_>>();

    input.nearby.retain(|ticket| {
        'field: for field in ticket.iter() {
            for range in ranges.iter() {
                if range.contains(field) {
                    continue 'field;
                }
            }

            error_rate += field;
            return false;
        }

        true
    });

    error_rate
}
