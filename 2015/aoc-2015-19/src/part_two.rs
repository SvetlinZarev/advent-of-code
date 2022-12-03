use std::collections::HashMap;
use rand::prelude::SliceRandom;

pub(crate) fn solve(molecule: &str, replacements: &HashMap<String, Vec<String>>) -> u32 {
    let mut inverse = vec![];
    for (src, dst) in replacements {
        for rep in dst {
            inverse.push((rep.as_str(), src.as_str()));
        }
    }

    let mut rng = rand::thread_rng();
    let mut reduced = molecule.to_owned();
    let mut steps = 0;

    'next: while reduced.len() > 1 {
        for (from, to) in inverse.iter().copied() {
            let next = reduced.replacen(from, to, 1);
            if reduced != next {
                reduced = next;
                steps += 1;
                continue 'next;
            }
        }

        steps = 0;
        reduced = molecule.to_owned();
        inverse.shuffle(&mut rng);
    }

    steps
}
