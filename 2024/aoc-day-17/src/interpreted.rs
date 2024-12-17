use crate::{Input, A, ADV, B, BDV, BST, BXC, BXL, C, CDV, JNZ, OUT};
use std::collections::VecDeque;

pub fn part_one(input: &Input) -> String {
    let mut out = String::new();

    let mut reg = input.reg;
    let mut pc = 0usize;

    while let Some(v) = step(&input.rom, &mut reg, &mut pc) {
        if !out.is_empty() {
            out.push(',');
        }

        out.push((v + b'0') as char);
    }

    out
}

pub fn part_two(input: &Input) -> u64 {
    // My program is:
    // do {
    //     b = a % 8;         // Take the last 3 bits of register A
    //     b = b ^ 5;         // [0x101]
    //     c = a / 2.pow(b);  //
    //     b = b ^ 6;         // [0x110]
    //     b = b ^ c;         //
    //     print b            //
    //     a = a / 8;         //I.e. `a >> 3` - remove the last 3 bits
    // } while(a != 0)
    //
    // As the input is 16 digits long, this means that the A we are looking
    // for must be `16 * 3 = 48` bits long; If it's shorter -> then we'll not
    // print 16 characters. If it's longer -> we'll print more characters
    //
    // We can make the following observations about the program:
    // * Register A is always 0 at the end of the program execution.
    // * The program always works with chunks of 3 bits and always shifts
    //   register A by 3 (i.e. divides A by 8);
    // * Smaller values of register A correspond to LSB bytes in the program ROM
    //
    // Thus we can bruteforce the value of A, by trying candidate values of A
    // which are multiples of 8 combined with all possible 3 LSB bits (i.e 0..=7)
    //
    // For instance if we start with the initial value of 0 (i.e. the end
    // condition of the program) we need to try all possible (only 8) 3 LSB bits
    // in order to find the last number of our program.
    //
    // We need to keep a list of all candidate values of A which successfully
    // produced the last digit. The for each of those candidates we repeat the
    // process: multiply the candidate by 8 and for that value try all possible
    // 3 LSB bits. We remember for later verification all the candidates that
    // successfully produced the second (going in reverse) digit of our programs.
    //
    // And we repeat that process until we';ve produced all digits of our program.
    //
    // At the end, our queue should contain at 0th position the value of
    // register A that produces the value of our program
    let mut next_a = VecDeque::with_capacity(16);
    next_a.push_back(0);

    let rom = &input.rom;
    let mut reg = input.reg;

    for &expected in rom.iter().rev() {
        for _ in 0..next_a.len() {
            let value = next_a.pop_front().unwrap();

            for n in 0..8u64 {
                let candidate = (value << 3) | n;

                reg[A] = candidate;
                if let Some(result) = step(rom, &mut reg, &mut 0) {
                    if expected == result {
                        next_a.push_back(candidate);
                    }
                }
            }
        }
    }

    // SANITY CHECK
    if cfg!(debug_assertions) {
        let mut reg = input.reg;
        reg[A] = next_a.front().copied().unwrap();

        let out = part_one(&Input {
            rom: rom.to_vec(),
            reg,
        });

        assert_eq!(
            input.rom,
            out.split(',')
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        );
    }

    next_a.pop_front().unwrap()
}

fn step(rom: &[u8], reg: &mut [u64; 3], pc: &mut usize) -> Option<u8> {
    while *pc < rom.len() {
        match rom[*pc] {
            ADV => {
                let x = operand(reg, rom, pc);
                reg[A] = reg[A] / 2u64.pow(x as u32);
                *pc += 2;
            }

            BXL => {
                reg[B] = reg[B] ^ rom[*pc + 1] as u64;
                *pc += 2;
            }

            BST => {
                let x = operand(reg, rom, pc);
                reg[B] = x % 8;
                *pc += 2;
            }

            JNZ => {
                if reg[A] == 0 {
                    *pc += 2;
                } else {
                    *pc = rom[*pc + 1] as usize;
                }
            }

            BXC => {
                reg[B] = reg[B] ^ reg[C];
                *pc += 2;
            }

            OUT => {
                let value = operand(reg, rom, pc) % 8;
                *pc += 2;

                return Some(value as u8);
            }

            BDV => {
                let x = operand(reg, rom, pc);
                reg[B] = reg[A] / 2u64.pow(x as u32);
                *pc += 2;
            }

            CDV => {
                let x = operand(reg, rom, pc);
                reg[C] = reg[A] / 2u64.pow(x as u32);
                *pc += 2;
            }

            _ => unreachable!(),
        }
    }

    None
}

fn operand(reg: &[u64; 3], rom: &[u8], pc: &usize) -> u64 {
    let operand_ptr = *pc + 1;
    let value = rom[operand_ptr];

    match value {
        0..4 => value as u64,
        4..7 => reg[(value - 4) as usize],
        _ => unreachable!(),
    }
}
