use crate::{Input, A, B, C};
use std::collections::VecDeque;

pub fn part_one(input: &Input) -> String {
    let (xa, xb) = extract_coeff(&input.rom);

    let mut out = String::new();
    let mut reg = input.reg;

    while reg[A] != 0 {
        let v = step_native(xa, xb, &mut reg);
        if !out.is_empty() {
            out.push(',');
        }

        out.push((v + b'0') as char);
    }

    out
}

pub fn part_two(input: &Input) -> u64 {
    let (xa, xb) = extract_coeff(&input.rom);

    let mut reg = input.reg;
    let mut next_a = VecDeque::with_capacity(16);
    next_a.push_back(0);

    for &expected in input.rom.iter().rev() {
        for _ in 0..next_a.len() {
            let value = next_a.pop_front().unwrap();

            for n in 0..8u64 {
                let candidate = (value << 3) | n;

                reg[A] = candidate;
                let result = step_native(xa, xb, &mut reg);
                if expected == result {
                    next_a.push_back(candidate);
                }
            }
        }
    }

    // SANITY CHECK
    if cfg!(debug_assertions) {
        let mut reg = input.reg;
        reg[A] = next_a.front().copied().unwrap();

        let out = part_one(&Input {
            rom: input.rom.to_vec(),
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

// Assume that all inputs have the same structure/program,
// and it's different only by the two XOR B parameters
//
// If this assumption is incorrect, then the whole V2 is incorrect
fn extract_coeff(rom: &[u8]) -> (u64, u64) {
    let mut coeff = rom
        .chunks_exact(2)
        .filter(|ch| ch[0] == 1)
        .map(|ch| ch[1] as u64);

    let a = coeff.next().unwrap();
    let b = coeff.next().unwrap();
    (a, b)
}

#[inline(always)]
fn step_native(xor_a: u64, xor_b: u64, reg: &mut [u64; 3]) -> u8 {
    reg[B] = reg[A] % 8;
    reg[B] = reg[B] ^ xor_a;
    reg[C] = reg[A] / 2u64.pow(reg[B] as u32);
    reg[B] = reg[B] ^ xor_b;
    reg[B] = reg[B] ^ reg[C];
    reg[A] = reg[A] / 8;

    (reg[B] % 8) as u8
}
