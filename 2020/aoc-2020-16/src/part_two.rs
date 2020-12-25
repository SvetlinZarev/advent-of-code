use std::collections::HashMap;

use crate::Input;

const FIELD_PREFIX: &str = "departure";

pub fn solve_v2(input: &Input) -> u64 {
    let mut to_fix = input.fields.len();
    let mut fixed_fields: Vec<Option<usize>> = vec![None; input.ticket.len()];
    let mut fixed_positions = vec![false; input.ticket.len()];

    while to_fix > 0 {
        //TODO detect non-solvable input

        'p: for pos in 0..input.ticket.len() {
            if fixed_positions[pos] {
                continue;
            }

            let mut matching_field = None;
            let mut matching_fields = 0;

            'f: for (f_idx, (_, range)) in input.fields.iter().enumerate() {
                if fixed_fields[f_idx].is_some() {
                    continue;
                }

                for ticket in input.nearby.iter() {
                    let value = ticket[pos];
                    if !range.first.contains(&value) && !range.second.contains(&value) {
                        continue 'f;
                    }
                }

                matching_field = Some(f_idx);
                matching_fields += 1;
                if matching_fields > 1 {
                    continue 'p;
                }
            }

            match matching_fields {
                1 => {
                    fixed_fields[matching_field.unwrap()] = Some(pos);
                    fixed_positions[pos] = true;
                    to_fix -= 1;
                }

                0 => panic!("Input contains invalid tickets!"),
                _ => continue,
            }
        }
    }

    let mut solution = 1;
    for (idx, (name, _)) in input.fields.iter().enumerate() {
        if name.starts_with(FIELD_PREFIX) {
            solution *= input.ticket[fixed_fields[idx].unwrap()] as u64;
        }
    }

    solution
}

pub fn solve_v1(input: &Input) -> u64 {
    let number_of_positions = input.ticket.len();
    let mut number_of_fixed_positions = 0;

    let mut fixed_fields = HashMap::new();
    let mut fixed_positions = vec![false; number_of_positions];

    while number_of_fixed_positions < number_of_positions {
        //TODO unsolvable input detection

        'pos: for pos in 0..number_of_positions {
            if fixed_positions[pos] {
                continue;
            }

            let mut fixed_field = "";
            let mut matching_fields = 0;

            'field: for (field, rule) in input.fields.iter() {
                if fixed_fields.contains_key(field) {
                    continue;
                }

                for ticket in input.nearby.iter() {
                    let field_value = ticket[pos];
                    if !rule.first.contains(&field_value) && !rule.second.contains(&field_value) {
                        continue 'field;
                    }
                }

                fixed_field = field;
                matching_fields += 1;
                if matching_fields > 1 {
                    continue 'pos;
                }
            }

            match matching_fields {
                0 => panic!("No matching field!"),
                1 => {
                    fixed_fields.insert(fixed_field, pos);
                    fixed_positions[pos] = true;
                    number_of_fixed_positions += 1;
                }
                _ => continue,
            }
        }
    }

    if cfg!(debug_assertions) {
        println!("{:#?}", fixed_fields);
    }

    let mut solution = 1;
    for (f, idx) in fixed_fields {
        if f.starts_with("departure") {
            solution *= input.ticket[idx] as u64;
        }
    }
    solution
}
