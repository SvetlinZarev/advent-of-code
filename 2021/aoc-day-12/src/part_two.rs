pub fn part_two(graph: &[Vec<usize>], limits: &[usize]) -> u64 {
    let mut limits = limits.to_vec();
    count_paths(&graph, &mut limits, 1, 0)
}

fn count_paths(
    graph: &[Vec<usize>],
    limits: &mut [usize],
    mut second_visit: usize,
    current_node: usize,
) -> u64 {
    if current_node == limits.len() - 1 {
        return 1;
    }

    let links = &graph[current_node];
    limits[current_node] -= 1;

    let mut paths = 0;
    for link in links.iter().copied() {
        let mut with_second_visit = false;

        if limits[link] == 0 {
            if second_visit <= 0 || link == 0 {
                // Link==0  is the start cave, which can be visited only once!
                continue;
            }

            limits[link] += 1;
            second_visit -= 1;
            with_second_visit = true;
        }

        paths += count_paths(graph, limits, second_visit, link);

        if with_second_visit {
            second_visit += 1;
            limits[link] -= 1;
        }
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
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (graph, limits) = parse_input(input);
        let answer = part_two(&graph, &limits);
        assert_eq!(104834, answer);
    }
}
