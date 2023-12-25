use aoc_shared::hashing::FxHashMap;

type HashMap<K, V> = FxHashMap<K, V>;

pub mod v1;
pub mod v2;
pub mod v3;

pub fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<(usize, usize)>) {
    let mut labels = HashMap::default();
    let mut graph = vec![vec![]; 0];
    let mut connections = vec![];

    for line in input.lines() {
        let (src, rest) = line.split_once(':').unwrap();
        let mut src_id = labels.len();
        src_id = *labels.entry(src).or_insert(src_id);

        if graph.len() <= src_id {
            graph.push(vec![]);
        }

        for dst in rest.trim().split_ascii_whitespace() {
            let mut dst_id = labels.len();
            dst_id = *labels.entry(dst).or_insert(dst_id);

            if graph.len() <= dst_id {
                graph.push(vec![]);
            }

            graph[src_id].push(dst_id);
            graph[dst_id].push(src_id);
            connections.push((src_id.min(dst_id), src_id.max(dst_id)));
        }
    }

    (graph, connections)
}

fn count_reachable(graph: &[Vec<usize>], start: usize) -> usize {
    let mut visited = vec![false; graph.len()];
    count_nodes(graph, &mut visited, start)
}

fn count_nodes(graph: &[Vec<usize>], visited: &mut [bool], node: usize) -> usize {
    visited[node] = true;

    let mut count = 1;

    for next in graph[node].iter().copied() {
        if !visited[next] {
            count += count_nodes(graph, visited, next);
        }
    }

    count
}
