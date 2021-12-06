pub fn part_one_v1(input: &[u8]) -> u64 {
    solve_v1(input, 80)
}

pub fn part_two_v1(input: &[u8]) -> u64 {
    solve_v1(input, 256)
}

pub fn part_one_v2(input: &[u8]) -> u64 {
    solve_v2(input, 80)
}

pub fn part_two_v2(input: &[u8]) -> u64 {
    solve_v2(input, 256)
}

pub fn part_one_v3(input: &[u8]) -> u64 {
    solve_v3(input, 80)
}

pub fn part_two_v3(input: &[u8]) -> u64 {
    solve_v3(input, 256)
}

fn solve_v1(input: &[u8], rounds: usize) -> u64 {
    let mut stage_a = [0u64; 9];
    let mut stage_b = [0u64; 9];

    input.iter().for_each(|&i| stage_a[i as usize] += 1);

    for _ in 0..rounds {
        stage_b[0] = stage_a[1];
        stage_b[1] = stage_a[2];
        stage_b[2] = stage_a[3];
        stage_b[3] = stage_a[4];
        stage_b[4] = stage_a[5];
        stage_b[5] = stage_a[6];
        stage_b[6] = stage_a[7] + stage_a[0];
        stage_b[7] = stage_a[8];
        stage_b[8] = stage_a[0];

        std::mem::swap(&mut stage_a, &mut stage_b);
    }

    stage_a.iter().sum()
}

fn solve_v2(input: &[u8], rounds: usize) -> u64 {
    let mut state = [0u64; 9];

    input.iter().for_each(|&i| state[i as usize] += 1);

    for idx in 0..rounds {
        state[(idx + 7) % 9] += state[idx % 9];
    }

    state.iter().sum()
}

fn solve_v3(input: &[u8], rounds: usize) -> u64 {
    let mut fishes = [0u64; 9];

    input.iter().for_each(|&i| fishes[i as usize] += 1);

    for _ in 0..rounds {
        let gen0 = fishes[0];
        fishes[0] = fishes[1];
        fishes[1] = fishes[2];
        fishes[2] = fishes[3];
        fishes[3] = fishes[4];
        fishes[4] = fishes[5];
        fishes[5] = fishes[6];
        fishes[6] = fishes[7] + gen0;
        fishes[7] = fishes[8];
        fishes[8] = gen0;
    }

    fishes.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_csv;

    #[test]
    fn test_part_one_v1() {
        let input = parse_csv(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one_v1(&input);
        assert_eq!(351188, answer);
    }

    #[test]
    fn test_part_one_v2() {
        let input = parse_csv(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one_v2(&input);
        assert_eq!(351188, answer);
    }
    #[test]
    fn test_part_one_v3() {
        let input = parse_csv(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one_v3(&input);
        assert_eq!(351188, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = parse_csv(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two_v1(&input);
        assert_eq!(1595779846729, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = parse_csv(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two_v2(&input);
        assert_eq!(1595779846729, answer);
    }

    #[test]
    fn test_part_two_v3() {
        let input = parse_csv(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two_v3(&input);
        assert_eq!(1595779846729, answer);
    }
}
