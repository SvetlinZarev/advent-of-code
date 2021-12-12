use std::collections::HashMap;

const KEY_START: &'static str = "start";
const KEY_END: &'static str = "end";
const NODE_SEPARATOR: char = '-';
const ID_START: usize = 0;

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

pub fn simplify_graph(graph: &[Vec<usize>], limits: &[usize]) -> Vec<Vec<(usize, u64)>> {
    // Assign consecutive IDs to the "small" caves
    let mut id_map = vec![None; graph.len()];
    let mut next_node_idx = 0;
    for node_id in 0..graph.len() {
        if limits[node_id] == 1 {
            id_map[node_id] = Some(next_node_idx);
            next_node_idx += 1;
        }
    }

    let mut simple_graph = Vec::with_capacity(graph.len());
    for (old_id, connections) in graph.iter().enumerate().take(graph.len() - 1) {
        if id_map[old_id].is_some() {
            let mut links: Vec<(usize, u64)> = vec![];

            for &connection in connections.iter() {
                match id_map[connection] {
                    Some(link) => {
                        // The start node can be visited only once!
                        if link != ID_START {
                            match links.binary_search_by(|&(id, _)| id.cmp(&link)) {
                                Ok(idx) => links[idx].1 += 1,
                                Err(idx) => links.insert(idx, (link, 1)),
                            }
                        }
                    }

                    None => {
                        for &conn in graph[connection].iter() {
                            let link = id_map[conn].unwrap();
                            // The start node can be visited only once!
                            if link != ID_START {
                                match links.binary_search_by(|&(id, _)| id.cmp(&link)) {
                                    Ok(idx) => links[idx].1 += 1,
                                    Err(idx) => links.insert(idx, (link, 1)),
                                }
                            }
                        }
                    }
                }
            }

            simple_graph.push(links);
        }
    }

    simple_graph.push(vec![]);
    simple_graph
}
