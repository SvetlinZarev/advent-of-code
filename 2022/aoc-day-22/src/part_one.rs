use crate::{find_start_column, Instruction};

pub fn part_one(map: &[Vec<u8>], instructions: &[Instruction]) -> usize {
    let start = find_start_column(map);

    // The facing is: R(0). D(1), L(2), U(3)
    let (mut r, mut c, mut f) = (0, start, 0);

    'next: for instr in instructions.iter().copied() {
        match instr {
            Instruction::RotR => f = (f + 1) % 4,
            Instruction::RotL => f = if f == 0 { 3 } else { f - 1 },
            Instruction::Move(steps) => {
                let steps = steps.get();

                match f {
                    //move right
                    0 => {
                        'step: for _ in 0..steps {
                            // should we wrap around
                            if c + 1 >= map[r].len() {
                                for cx in 0..map[r].len() {
                                    // skip padding
                                    if map[r][cx] == b' ' {
                                        continue;
                                    }

                                    // cannot wrap because of a wall
                                    if map[r][cx] == b'#' {
                                        continue 'next;
                                    }

                                    c = cx;
                                    continue 'step;
                                }

                                unreachable!()
                            }

                            // cannot move because of a wall
                            if map[r][c + 1] == b'#' {
                                continue 'next;
                            }

                            c += 1;
                        }
                    }

                    // move down
                    1 => {
                        'step: for _ in 0..steps {
                            // should we wrap around
                            if r + 1 >= map.len() || c >= map[r + 1].len() || map[r + 1][c] == b' '
                            {
                                for rx in 0..map.len() {
                                    // skip padding
                                    if c >= map[rx].len() || map[rx][c] == b' ' {
                                        continue;
                                    }

                                    // cannot wrap because of a wall
                                    if map[rx][c] == b'#' {
                                        continue 'next;
                                    }

                                    r = rx;
                                    continue 'step;
                                }
                                unreachable!()
                            }

                            // cannot move because of a wall
                            if map[r + 1][c] == b'#' {
                                continue 'next;
                            }

                            r += 1;
                        }
                    }

                    // move left
                    2 => {
                        'step: for _ in 0..steps {
                            // should we wrap around
                            if c == 0 || map[r][c - 1] == b' ' {
                                for cx in (0..map[r].len()).rev() {
                                    if map[r][cx] == b' ' {
                                        continue;
                                    }

                                    // cannot wrap because of a wall
                                    if map[r][cx] == b'#' {
                                        continue 'next;
                                    }

                                    c = cx;
                                    continue 'step;
                                }

                                unreachable!()
                            }

                            // cannot move because of a wall
                            if map[r][c - 1] == b'#' {
                                continue 'next;
                            }

                            c -= 1;
                        }
                    }

                    // move up
                    3 => {
                        'step: for _ in 0..steps {
                            // should we wrap around
                            if r == 0 || c >= map[r - 1].len() || map[r - 1][c] == b' ' {
                                for rx in (0..map.len()).rev() {
                                    // skip padding
                                    if c >= map[rx].len() || map[rx][c] == b' ' {
                                        continue;
                                    }

                                    // cannot wrap because of a wall
                                    if map[rx][c] == b'#' {
                                        continue 'next;
                                    }

                                    r = rx;
                                    continue 'step;
                                }
                                unreachable!()
                            }

                            // cannot move because of a wall
                            if map[r - 1][c] == b'#' {
                                continue 'next;
                            }

                            r -= 1;
                        }
                    }

                    _ => unreachable!(),
                }
            }
        }
    }

    1000 * (r + 1) + 4 * (c + 1) + f as usize
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::parse_input;
    use crate::part_one::part_one;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (map, instr) = parse_input(input);

        let answer = part_one(&map, &instr);
        assert_eq!(57350, answer);
    }
}
