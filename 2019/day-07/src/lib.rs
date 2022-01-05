use aoc_shared_2019::intcode::{Evaluation, Input, Instruction};
use std::collections::VecDeque;
use std::iter::once;

pub fn part_one(mem: &[isize]) -> isize {
    let mut best = 0;

    for s0 in 0..5 {
        for s1 in 0..5 {
            if s1 == s0 {
                continue;
            }
            for s2 in 0..5 {
                if s2 == s0 || s2 == s1 {
                    continue;
                }
                for s3 in 0..5 {
                    if s3 == s0 || s3 == s1 || s3 == s2 {
                        continue;
                    }
                    for s4 in 0..5 {
                        if s4 == s0 || s4 == s1 || s4 == s2 || s4 == s3 {
                            continue;
                        }

                        let a0 = amplifier(mem.to_vec(), once(s0).chain(once(0)));
                        let a1 = amplifier(mem.to_vec(), once(s1).chain(once(a0)));
                        let a2 = amplifier(mem.to_vec(), once(s2).chain(once(a1)));
                        let a3 = amplifier(mem.to_vec(), once(s3).chain(once(a2)));
                        let a4 = amplifier(mem.to_vec(), once(s4).chain(once(a3)));
                        best = best.max(a4);
                    }
                }
            }
        }
    }

    best
}

fn amplifier(mut mem: Vec<isize>, mut input: impl Input) -> isize {
    let mut state = Evaluation::Continue(0);
    let mut output = vec![];

    loop {
        match state {
            Evaluation::Continue(ip) => {
                let instruction = Instruction::decode(mem[ip]);
                state = instruction.eval(&mut mem, &mut input, &mut output, ip);
            }
            Evaluation::Halt => break,
            Evaluation::Fault(fault) => panic!("Failed to execute intcode: {:?}", fault),
            Evaluation::InsufficientInput(_) => panic!("The program requires input"),
        }
    }

    assert_eq!(output.len(), 1);
    output[0]
}

pub fn part_two(mem: &[isize]) -> isize {
    let mut best = 0;

    for s0 in 0..5 {
        for s1 in 0..5 {
            if s1 == s0 {
                continue;
            }
            for s2 in 0..5 {
                if s2 == s0 || s2 == s1 {
                    continue;
                }
                for s3 in 0..5 {
                    if s3 == s0 || s3 == s1 || s3 == s2 {
                        continue;
                    }
                    for s4 in 0..5 {
                        if s4 == s0 || s4 == s1 || s4 == s2 || s4 == s3 {
                            continue;
                        }

                        let value = amplifier_chain(mem, s0 + 5, s1 + 5, s2 + 5, s3 + 5, s4 + 5);
                        best = best.max(value);
                    }
                }
            }
        }
    }

    best
}

fn amplifier_chain(mem: &[isize], s0: isize, s1: isize, s2: isize, s3: isize, s4: isize) -> isize {
    const CHAIN_LEN: usize = 5;

    let mut amps = vec![mem.to_vec(); CHAIN_LEN];
    let mut state = vec![Evaluation::Continue(0); CHAIN_LEN];
    let mut iobuf = vec![VecDeque::new(); CHAIN_LEN];

    iobuf[0].push_back(s0);
    iobuf[0].push_back(0);
    iobuf[1].push_back(s1);
    iobuf[2].push_back(s2);
    iobuf[3].push_back(s3);
    iobuf[4].push_back(s4);

    'all: loop {
        for amp in 0..amps.len() {
            let mem = &mut amps[amp];
            let mut amp_state = state[amp].clone();

            let (input, output) = if amp == CHAIN_LEN - 1 {
                let (a, b) = iobuf.split_at_mut(amp);
                (&mut b[0], &mut a[0])
            } else {
                let (a, b) = iobuf.split_at_mut(amp + 1);
                (a.last_mut().unwrap(), &mut b[0])
            };

            loop {
                match amp_state {
                    Evaluation::Halt => {
                        if mem.is_empty() {
                            panic!("Cannot continue execution on a halted amplifier: {}", amp);
                        }

                        mem.truncate(0);
                        if amp == CHAIN_LEN - 1 {
                            break 'all;
                        }

                        break;
                    }
                    Evaluation::Fault(fault) => {
                        panic!("[AMP {}] Failed to execute program: {:?}", amp, fault);
                    }
                    Evaluation::Continue(ip) => {
                        let instruction = Instruction::decode(mem[ip]);
                        amp_state = instruction.eval(mem, input, output, ip);
                    }
                    Evaluation::InsufficientInput(ip) => {
                        state[amp] = Evaluation::Continue(ip);
                        break;
                    }
                }
            }
        }
    }

    iobuf[0].pop_front().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared_2019::input::load_csv_input_from_file;

    #[test]
    fn test_part_one() {
        let mem = load_csv_input_from_file("inputs/input.txt");
        let answer = part_one(&mem);
        assert_eq!(212460, answer);
    }

    #[test]
    fn test_part_two() {
        let mem = load_csv_input_from_file("inputs/input.txt");
        let answer = part_two(&mem);
        assert_eq!(21844737, answer);
    }
}
