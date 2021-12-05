pub fn part_one_v1(input: &[u8]) -> u32 {
    let mut stage_a = [0u32; 9];
    let mut stage_b = [0u32; 9];

    input.iter().for_each(|&i| stage_a[i as usize] += 1);

    for _ in 0..80 {
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


pub fn part_two_v1(input: &[u8]) -> u64 {
    let mut stage_a = [0u64; 9];
    let mut stage_b = [0u64; 9];

    input.iter().for_each(|&i| stage_a[i as usize] += 1);

    for _ in 0..256 {
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

pub fn part_one_v2(input: &[u8]) -> u32 {
    let mut state = [0u32; 9];

    input.iter().for_each(|&i| state[i as usize] += 1);

    for idx in 0..80 {
        state[(idx + 7) % 9] += state[idx % 9];
    }

    state.iter().sum()
}


pub fn part_two_v2(input: &[u8]) -> u64 {
    let mut state = [0u64; 9];

    input.iter().for_each(|&i| state[i as usize] += 1);

    for idx in 0..256 {
        state[(idx + 7) % 9] += state[idx % 9];
    }

    state.iter().sum()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_csv;
    use super::*;

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
}