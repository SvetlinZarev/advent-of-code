const ASCII_LEN: usize = (b'z' - b'a' + 1) as usize;

pub fn part_one(input: &str) -> usize {
    sliding_window(input, 4)
}

pub fn part_two(input: &str) -> usize {
    sliding_window(input, 14)
}

pub fn sliding_window(input: &str, window: usize) -> usize {
    let input = input.as_bytes();
    if input.len() < window {
        panic!("Input is too short: {}", input.len());
    }

    let mut seen = [0u32; ASCII_LEN];
    let mut uniq = 0;

    // Seed the algorithm with the first `window` bytes
    for idx in 0..window {
        let ch = (input[idx] - b'a') as usize;

        seen[ch] += 1;
        if seen[ch] == 1 {
            uniq += 1;
        }
    }

    // Fast path: check if the first `window` bytes are the solution
    if uniq == window {
        return window;
    }

    input
        .windows(window + 1)
        .enumerate()
        .find(|&(_idx, w)| {
            let ch = (w[0] - b'a') as usize;
            seen[ch] -= 1;
            if seen[ch] == 0 {
                uniq -= 1;
            }

            let ch = (w[window] - b'a') as usize;
            seen[ch] += 1;
            if seen[ch] == 1 {
                uniq += 1;
            }

            uniq == window
        })
        // convert the window index to character index
        .map(|(pos, w)| pos + w.len())
        .expect("no answer")
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(1702, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(3559, answer);
    }
}
