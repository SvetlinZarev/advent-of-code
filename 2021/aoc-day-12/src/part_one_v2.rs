pub fn part_one_v2(graph: &[Vec<(usize, u64)>]) -> u64 {
    let mut visited = vec![false; graph.len()];
    count_paths(&graph, &mut visited, 0)
}

fn count_paths(graph: &[Vec<(usize, u64)>], visited: &mut [bool], current_node: usize) -> u64 {
    if current_node == graph.len() - 1 {
        return 1;
    }

    let links = &graph[current_node];
    visited[current_node] = true;

    let mut paths = 0;

    for (link, cnt) in links.iter().copied() {
        if visited[link] {
            continue;
        }

        paths += cnt * count_paths(graph, visited, link);
    }

    visited[current_node] = false;
    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_input, simplify_graph};
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (graph, limits) = parse_input(input);
        let graph = simplify_graph(&graph, &limits);
        let answer = part_one_v2(&graph);
        assert_eq!(3887, answer);
    }
}
