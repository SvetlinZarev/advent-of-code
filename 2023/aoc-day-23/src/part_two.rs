use std::collections::VecDeque;

use aoc_shared::grid::DIR4;
use aoc_shared::hashing::FxHashMap;

const START_RC: (usize, usize) = (0, 1);

type HashMap<K, V> = FxHashMap<K, V>;

pub fn part_two(input: &str) -> usize {
    let grid = input.as_bytes();
    let cols = grid.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = grid.len() / cols;

    let graph = compress_graph(grid, cols, rows, (0, 1));
    let graph = relabel_graph(&graph, START_RC, (rows - 1, cols - 3));
    assert!(graph.len() < 64);

    let exit_nodes = graph[1].iter().fold(0, |acc, &(id, _)| acc | (1 << id));

    // because we have less than 64 nodes in the graph, we can
    // use a bitset to track the visited nodes. Initially,
    // the bitset contains only the starting node
    longest_path(&graph, exit_nodes, 0 | 1 << 0, 0).unwrap()
}

fn compress_graph(
    grid: &[u8],
    cols: usize,
    rows: usize,
    start: (usize, usize),
) -> HashMap<(usize, usize), Vec<((usize, usize), usize)>> {
    let mut graph = HashMap::default();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((r, c)) = queue.pop_front() {
        // do not re-process already explored crossroads in order to avoid duplicates
        if graph.contains_key(&(r, c)) {
            continue;
        }

        for (dr, dc) in DIR4 {
            if let Some((nr, nc, cost)) = walk(grid, rows, cols, r, c, dr, dc) {
                // Check if that starting node has already been explored
                // If so - do not add it to the queue
                if !graph.contains_key(&(nr, nc)) {
                    queue.push_back((nr, nc));
                }

                graph.entry((r, c)).or_insert(vec![]).push(((nr, nc), cost));
            }
        }
    }

    graph
}

fn walk(
    grid: &[u8],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    dr: isize,
    dc: isize,
) -> Option<(usize, usize, usize)> {
    let (mut r, mut c) = (r, c);
    let (mut pr, mut pc) = (dr, dc);

    let initial_dir = [(dr, dc)];
    let mut dirs = initial_dir.as_slice().into_iter();

    let mut len = 0;

    loop {
        let mut neighbors = 0;
        let (mut dir_r, mut dir_c) = (pr, pc);

        for (dr, dc) in dirs.copied() {
            // do not go back the same path we come from
            if (dr, dc) == (-pr, -pc) {
                continue;
            }

            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 {
                continue;
            }

            let nr = nr as usize;
            let nc = nc as usize;
            if nr >= rows || nc >= cols - 1 {
                continue;
            }

            if grid[nr * cols + nc] == b'#' {
                continue;
            }

            neighbors += 1;
            (dir_r, dir_c) = (dr, dc);
        }

        // This is a "crossroads" node, so we have to walk each
        // individual outgoing path separately
        if neighbors > 1 {
            return Some((r, c, len));
        }

        // This path leads to a dead-end, so we can ignore it
        // unless this is our target node
        if neighbors == 0 {
            if len > 0 {
                return Some((r, c, len));
            }

            // dead end -> ignore it
            return None;
        }

        pr = dir_r;
        pc = dir_c;

        r = (r as isize + pr) as usize;
        c = (c as isize + pc) as usize;

        len += 1;
        dirs = DIR4.as_slice().into_iter();
    }
}

fn relabel_graph(
    graph: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<Vec<(usize, usize)>> {
    let mut labels = HashMap::default();
    labels.reserve(graph.len() + 32);

    labels.insert(start, 0);
    labels.insert(end, 1);

    let mut nodes = vec![vec![]; graph.len()];

    for (&node, children) in graph {
        let mut node_id = labels.len();
        node_id = *labels.entry(node).or_insert(node_id);

        for (child, cost) in children.iter().copied() {
            let mut child_id = labels.len();
            child_id = *labels.entry(child).or_insert(child_id);

            nodes[node_id].push((child_id, cost));
        }
    }

    nodes
}

fn longest_path(
    graph: &[Vec<(usize, usize)>],
    exit_nodes: usize,
    seen: usize,
    node: usize,
) -> Option<usize> {
    // We fixed the DST node to be always 1 when we relabeled the graph
    const DST_NODE: usize = 1;

    // Base case
    if node == DST_NODE {
        return Some(0);
    }

    if seen & exit_nodes == exit_nodes {
        // Small optimisation:
        // All nodes that are directly connected to the DST node
        // have been visited, yet, the current node is not the
        // DST node. Therefore, this path will not yield a result,
        // because we can never reach the DST node, because we
        // cannot visit nodes twice. Thus stop searching.
        return None;
    }

    let mut len = None;

    for &(next, l) in graph[node].iter() {
        if seen & 1 << next == 0 {
            // Note that we are marking the CURRENT node as visited, not the
            // NEXT node. This is because of the short-circuiting logic
            // checking for blocked path to the DST node above
            if let Some(path) = longest_path(graph, exit_nodes, seen | (1 << node), next) {
                let dist = l + path;
                len = Some(dist).max(len);
            }
        }
    }

    len
}
