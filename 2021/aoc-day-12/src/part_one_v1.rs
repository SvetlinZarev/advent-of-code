pub fn part_one_v1(graph: &[Vec<usize>], limits: &[usize]) -> u64 {
    let mut limits = limits.to_vec();
    count_paths(&graph, &mut limits, 0)
}

fn count_paths(graph: &[Vec<usize>], limits: &mut [usize], current_node: usize) -> u64 {
    if current_node == limits.len() - 1 {
        return 1;
    }

    let links = &graph[current_node];
    limits[current_node] -= 1;

    let mut paths = 0;

    for link in links.iter().copied() {
        if limits[link] == 0 {
            continue;
        }

        paths += count_paths(graph, limits, link);
    }

    limits[current_node] += 1;
    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (graph, limits) = parse_input(input);
        let answer = part_one_v1(&graph, &limits);
        assert_eq!(3887, answer);
    }
}
