use aoc_shared_2019::intcode::{Computer, Fault, Outcome};
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

                        let phase = [s0, s1, s2, s3, s4];
                        let mut output = 0;
                        for amp in 0..5 {
                            let mut input = once(phase[amp]).chain(once(output));
                            let mut vm = Computer::new(mem.to_vec());
                            match vm.run(&mut input, &mut output) {
                                Outcome::Halt => best = best.max(output),
                                Outcome::Fault(fault) => panic!("Unexpected error: {:?}", fault),
                            }
                        }
                    }
                }
            }
        }
    }

    best
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

    let mut vms = vec![Computer::new(mem.to_vec()); CHAIN_LEN];
    let mut iobuf = vec![VecDeque::new(); CHAIN_LEN];

    iobuf[0].push_back(s0);
    iobuf[0].push_back(0);
    iobuf[1].push_back(s1);
    iobuf[2].push_back(s2);
    iobuf[3].push_back(s3);
    iobuf[4].push_back(s4);

    'all: loop {
        for vm_idx in 0..vms.len() {
            let (input, output) = if vm_idx == CHAIN_LEN - 1 {
                let (a, b) = iobuf.split_at_mut(vm_idx);
                (&mut b[0], &mut a[0])
            } else {
                let (a, b) = iobuf.split_at_mut(vm_idx + 1);
                (a.last_mut().unwrap(), &mut b[0])
            };

            loop {
                match vms[vm_idx].run(input, output) {
                    Outcome::Halt => {
                        if vm_idx == CHAIN_LEN - 1 {
                            break 'all;
                        }

                        break;
                    }
                    Outcome::Fault(fault) => match fault {
                        Fault::Error(fault) => panic!("Unexpected error: {:?}", fault),
                        Fault::InsufficientInput => break,
                    },
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
