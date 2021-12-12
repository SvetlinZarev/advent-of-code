use crate::parsing::simplify_graph;

pub fn part_two_v2(graph: &[Vec<usize>], limits: &[usize]) -> u64 {
    let graph = simplify_graph(graph, limits);
    let mut visited = vec![false; graph.len()];
    count_paths(&graph, &mut visited, 0, true)
}

fn count_paths(
    graph: &[Vec<(usize, u64)>],
    visited: &mut [bool],
    current_node: usize,
    second_visit: bool,
) -> u64 {
    if current_node == graph.len() - 1 {
        return 1;
    }

    let links = &graph[current_node];
    visited[current_node] = true;

    let mut paths = 0;
    for (link, cnt) in links.iter().copied() {
        let mut with_second_visit = false;

        if visited[link] {
            if !second_visit || link == 0 {
                // Link==0  is the start cave, which can be visited only once!
                continue;
            }

            visited[link] = false;
            with_second_visit = true;
        }

        paths += cnt * count_paths(graph, visited, link, second_visit ^ with_second_visit);

        if with_second_visit {
            visited[link] = true;
        }
    }

    visited[current_node] = false;
    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (graph, limits) = parse_input(input);
        let answer = part_two_v2(&graph, &limits);
        assert_eq!(104834, answer);
    }
}
