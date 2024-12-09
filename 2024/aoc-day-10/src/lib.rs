use aoc_shared::grid::DIR4;

pub fn part_one(input: &str) -> u32 {
    let input = input.as_bytes();
    let width = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let height = input.len() / width;

    let mut answer = 0;
    let mut mark = 0;

    let mut stack = vec![];
    let mut seen = vec![0u16; input.len()];

    for r in 0..height {
        for c in 0..width - 1 {
            if input[r * width + c] == b'0' {
                mark += 1;

                stack.push((r as isize, c as isize));
                while let Some((r, c)) = stack.pop() {
                    let old_idx = r as usize * width + c as usize;

                    for (dr, dc) in DIR4 {
                        let y = r + dr;
                        let x = c + dc;

                        if !(0..height as isize).contains(&y) {
                            continue;
                        }

                        if !(0..width as isize - 1).contains(&x) {
                            continue;
                        }

                        let new_idx = y as usize * width + x as usize;
                        if input[new_idx] as i32 - input[old_idx] as i32 != 1 {
                            continue;
                        }

                        if seen[new_idx] == mark {
                            continue;
                        }
                        seen[new_idx] = mark;

                        if input[new_idx] == b'9' {
                            answer += 1;
                        } else {
                            stack.push((y, x));
                        }
                    }
                }
            }
        }
    }

    answer
}

pub fn part_two_v1(input: &str) -> u32 {
    let input = input.as_bytes();
    let width = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let height = input.len() / width;

    let mut answer = 0;
    let mut stack = vec![];

    for r in 0..height {
        for c in 0..width - 1 {
            if input[r * width + c] == b'0' {
                stack.push((r as isize, c as isize));

                while let Some((r, c)) = stack.pop() {
                    let old_idx = r as usize * width + c as usize;

                    for (dr, dc) in DIR4 {
                        let y = r + dr;
                        let x = c + dc;

                        if !(0..height as isize).contains(&y) {
                            continue;
                        }

                        if !(0..width as isize - 1).contains(&x) {
                            continue;
                        }

                        let new_idx = y as usize * width + x as usize;
                        if input[new_idx] as i32 - input[old_idx] as i32 != 1 {
                            continue;
                        }

                        if input[new_idx] == b'9' {
                            answer += 1;
                        } else {
                            stack.push((y, x));
                        }
                    }
                }
            }
        }
    }

    answer
}

pub fn part_two_v2(input: &str) -> u32 {
    let input = input.as_bytes();
    let width = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let height = input.len() / width;

    let mut answer = 0;
    let mut memo = vec![-1i8; input.len()];

    for r in 0..height {
        for c in 0..width - 1 {
            if input[r * width + c] == b'0' {
                answer += dfs(
                    input,
                    &mut memo,
                    height as isize,
                    width as isize,
                    r as isize,
                    c as isize,
                ) as u32;
            }
        }
    }

    answer
}

fn dfs(input: &[u8], memo: &mut [i8], height: isize, width: isize, r: isize, c: isize) -> i8 {
    let idx = (r * width + c) as usize;
    if input[idx] == b'9' {
        return 1;
    }

    if memo[idx] >= 0 {
        return memo[idx];
    }

    let mut count = 0;
    if r - 1 >= 0 && input[((r - 1) * width + c) as usize] as i32 - input[idx] as i32 == 1 {
        count += dfs(input, memo, height, width, r - 1, c);
    }
    if c - 1 >= 0 && input[(r * width + c - 1) as usize] as i32 - input[idx] as i32 == 1 {
        count += dfs(input, memo, height, width, r, c - 1);
    }
    if c + 1 < width - 1 && input[(r * width + c + 1) as usize] as i32 - input[idx] as i32 == 1 {
        count += dfs(input, memo, height, width, r, c + 1);
    }
    if r + 1 < height && input[((r + 1) * width + c) as usize] as i32 - input[idx] as i32 == 1 {
        count += dfs(input, memo, height, width, r + 1, c);
    }

    memo[idx] = count;
    count
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(512, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_v1(&input);
        assert_eq!(1045, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two_v2(&input);
        assert_eq!(1045, answer);
    }
}
