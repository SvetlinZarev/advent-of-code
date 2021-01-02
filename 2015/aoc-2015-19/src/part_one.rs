use std::collections::{HashMap, HashSet};

pub fn solve(molecule: &str, replacements: &HashMap<String, Vec<String>>) -> usize {
    let mut variants = HashSet::new();

    let mut start = 0;
    for (idx, ch) in molecule.bytes().enumerate() {
        let mut offset = 1;
        if idx < molecule.len() - 1 {
            if start == idx {
                continue;
            }

            if ch.is_ascii_lowercase() {
                continue;
            }
            offset = 0;
        }

        let end = idx + offset;

        let key = &molecule[start..end];
        if let Some(repl) = replacements.get(key) {
            for rep in repl {
                let mut buffer = String::with_capacity(molecule.len() + rep.len());
                buffer.push_str(&molecule[0..start]);
                buffer.push_str(rep);
                buffer.push_str(&molecule[end..]);

                variants.insert(buffer);
            }
        }

        start = end;
    }

    variants.len()
}
