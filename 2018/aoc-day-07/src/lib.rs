use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::sync::LazyLock;

const ASCII_LEN: usize = (b'Z' - b'A' + 1) as usize;

static REGEX_INPUT_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^Step (?<FROM>[A-Z]) must be finished before step (?<TO>[A-Z]) can begin\.$"#)
        .unwrap()
});

pub fn parse_input(input: &str) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let mut graph = vec![vec![]; ASCII_LEN];

    for line in input.lines() {
        let Some(captures) = REGEX_INPUT_LINE.captures(line) else {
            return Err(format!("invalid input: {}", line).into());
        };

        let Some(from) = captures.name("FROM") else {
            return Err(format!("invalid input: {}", line).into());
        };

        let Some(to) = captures.name("TO") else {
            return Err(format!("invalid input: {}", line).into());
        };

        let from: usize = (from.as_str().as_bytes()[0] - b'A') as usize;
        let to: usize = (to.as_str().as_bytes()[0] - b'A') as usize;

        graph[from].push(to);
    }

    Ok(graph)
}

pub fn part_one(input: &Vec<Vec<usize>>) -> String {
    let mut indegrees = vec![0; ASCII_LEN];
    for destinations in input {
        for &node in destinations {
            indegrees[node] += 1;
        }
    }

    let mut queue = BinaryHeap::new();
    for node in 0..indegrees.len() {
        if indegrees[node] == 0 && !input[node].is_empty() {
            queue.push(Reverse(node));
        }
    }

    let mut answer = String::new();

    while let Some(Reverse(node)) = queue.pop() {
        answer.push((node as u8 + b'A') as char);

        for &next in input[node].iter() {
            indegrees[next] -= 1;
            if indegrees[next] == 0 {
                queue.push(Reverse(next));
            }
        }
    }

    answer
}

pub fn part_two(input: &Vec<Vec<usize>>) -> u64 {
    const WORKERS: usize = 5;
    const BASE_DURATION: u64 = 60;

    let mut indegrees = vec![0; ASCII_LEN];
    for destinations in input {
        for &node in destinations {
            indegrees[node] += 1;
        }
    }

    let mut queue = BinaryHeap::new();
    for node in 0..indegrees.len() {
        if indegrees[node] == 0 && !input[node].is_empty() {
            queue.push(Reverse((0u64, node)));
        }
    }

    let mut workers = BinaryHeap::new();
    for _ in 0..WORKERS {
        workers.push(Reverse(0u64));
    }

    while let Some(Reverse((no_earlier_than, node))) = queue.pop() {
        let worker_finish_time = workers.pop().unwrap().0;
        let task_start_time = worker_finish_time.max(no_earlier_than);
        let task_finish_time = task_start_time + BASE_DURATION + node as u64 + 1;
        workers.push(Reverse(task_finish_time));

        for &next in input[node].iter() {
            indegrees[next] -= 1;
            if indegrees[next] == 0 {
                queue.push(Reverse((task_finish_time, next)));
            }
        }
    }

    workers
        .into_iter()
        .map(|Reverse(finish_time)| finish_time)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_one(&input);
        assert_eq!("JNOIKSYABEQRUVWXGTZFDMHLPC", answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_two(&input);
        assert_eq!(1099, answer);
    }
}
