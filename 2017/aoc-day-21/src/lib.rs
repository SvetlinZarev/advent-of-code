use std::collections::HashMap;

const INITIAL_PATTERN: &[u8] = b".#...####";
const INITIAL_WIDTH: usize = 3;

pub fn part_one(input: &str) -> usize {
    solve(input, 5)
}

pub fn part_two(input: &str) -> usize {
    solve(input, 18)
}

fn solve(input: &str, iterations: usize) -> usize {
    let (mut pattern_idx_map, patterns) = load_input(input);

    let mut width = INITIAL_WIDTH;
    let mut buf1 = INITIAL_PATTERN.to_vec();
    let mut buf2 = vec![];
    let mut buf0 = vec![];

    for _ in 0..iterations {
        let kernel = kernel_width(width);
        let next_kernel = kernel + 1;
        let next_width = expand_grid(&mut buf2, width);

        buf0.clear();
        buf0.resize(kernel * kernel, 0);

        let steps = width / kernel;

        for r in 0..steps {
            for c in 0..steps {
                read_pattern(&buf1, &mut buf0, r * kernel, c * kernel, width, kernel);
                let idx = find_pattern(&mut pattern_idx_map, &buf0, kernel);
                write_pattern(
                    &patterns[idx],
                    &mut buf2,
                    r * next_kernel,
                    c * next_kernel,
                    next_width,
                    next_kernel,
                );
            }
        }

        std::mem::swap(&mut buf1, &mut buf2);
        width = next_width;
    }

    buf1.iter().filter(|&&x| x == b'#').count()
}

fn load_input(input: &str) -> (HashMap<Vec<u8>, usize>, Vec<Vec<u8>>) {
    let mut key_idx_map = HashMap::new();
    let mut patterns = vec![];

    for line in input.lines() {
        let (key, value) = line.split_once("=>").unwrap();
        let key = key.trim().replace('/', "").into_bytes();
        let value = value.trim().replace('/', "").into_bytes();

        key_idx_map.insert(key, patterns.len());
        patterns.push(value)
    }

    (key_idx_map, patterns)
}

fn kernel_width(grid_width: usize) -> usize {
    if grid_width % 2 == 0 {
        2
    } else {
        3
    }
}

fn expand_grid(buf: &mut Vec<u8>, width: usize) -> usize {
    let kernel = kernel_width(width);
    let squares = width / kernel;
    let next_width = squares * (kernel + 1);

    buf.clear();
    buf.resize(next_width * next_width, 0);
    next_width
}

fn read_pattern(src: &[u8], dst: &mut [u8], r: usize, c: usize, width: usize, kernel: usize) {
    let mut idx = 0;

    for y in r..r + kernel {
        for x in c..c + kernel {
            dst[idx] = src[y * width + x];
            idx += 1;
        }
    }
}

fn write_pattern(src: &[u8], dst: &mut [u8], r: usize, c: usize, width: usize, kernel: usize) {
    let mut idx = 0;

    for y in r..r + kernel {
        for x in c..c + kernel {
            dst[y * width + x] = src[idx];
            idx += 1;
        }
    }
}

fn find_pattern(
    pattern_idx_map: &mut HashMap<Vec<u8>, usize>,
    pattern: &Vec<u8>,
    kernel: usize,
) -> usize {
    if let Some(pos) = pattern_idx_map.get(pattern).copied() {
        return pos;
    }

    // cache the key-values for faster future searches
    let mut key = pattern.to_vec();
    let mut keys = vec![];

    for _ in 0..2 {
        for _ in 0..4 {
            if let Some(pos) = pattern_idx_map.get(&key).copied() {
                for key in keys {
                    pattern_idx_map.insert(key, pos);
                }

                return pos;
            }

            keys.push(key.clone());
            rot(&mut key, kernel);
        }

        flip(&mut key, kernel);
    }

    unreachable!("a pattern must exist")
}

fn rot(buf: &mut [u8], kernel: usize) {
    for i in 0..kernel {
        for j in i..kernel {
            swap(buf, kernel, i, j, j, i);
        }
    }

    for i in 0..kernel {
        for j in 0..kernel / 2 {
            swap(buf, kernel, j, i, kernel - j - 1, i);
        }
    }
}

fn flip(buf: &mut [u8], kernel: usize) {
    buf.chunks_exact_mut(kernel)
        .for_each(|chunk| chunk.reverse());
}

fn swap(buf: &mut [u8], kernel: usize, r0: usize, c0: usize, r1: usize, c1: usize) {
    let x = r0 * kernel + c0;
    let y = r1 * kernel + c1;
    buf.swap(x, y);
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let answer = part_one(&input);

        assert_eq!(184, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two(&input);

        assert_eq!(2_810_258, answer);
    }
}
