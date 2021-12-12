use std::collections::HashMap;

const KEY_START: &'static str = "start";
const KEY_END: &'static str = "end";
const NODE_SEPARATOR: char = '-';

pub fn parse_input<'l, I: AsRef<str> + 'l>(input: I) -> (Vec<Vec<usize>>, Vec<usize>) {
    let input = input.as_ref();

    let mut connections = HashMap::new();
    let mut ids = HashMap::new();
    let mut last_id = 0;

    input
        .lines()
        .map(|l| l.split_once(NODE_SEPARATOR).unwrap())
        .for_each(|(a, b)| {
            connections
                .entry(a)
                .and_modify(|v: &mut Vec<&str>| v.push(b))
                .or_insert(vec![b]);

            connections
                .entry(b)
                .and_modify(|v| v.push(a))
                .or_insert(vec![a]);

            if a != KEY_START && a != KEY_END {
                ids.entry(a).or_insert_with(|| {
                    last_id += 1;
                    last_id
                });
            }

            if b != KEY_START && b != KEY_END {
                ids.entry(b).or_insert_with(|| {
                    last_id += 1;
                    last_id
                });
            }
        });

    ids.insert(KEY_START, 0);
    ids.insert(KEY_END, last_id + 1);

    let mut graph = vec![vec![]; last_id + 2];
    let mut limits = vec![0; last_id + 2];

    for (&k, v) in connections.iter() {
        let &key_id = ids.get(k).unwrap();
        limits[key_id] = max_visits(k);

        let links = &mut graph[key_id as usize];
        for &node in v.iter() {
            let &node_id = ids.get(node).unwrap();
            links.push(node_id);
        }
        links.sort_unstable();
    }

    (graph, limits)
}

fn max_visits(s: &str) -> usize {
    assert!(!s.is_empty());

    if (b'a'..=b'z').contains(&s.as_bytes()[0]) {
        return 1;
    }

    usize::MAX
}
