use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "a" => Register::A,
            "b" => Register::B,
            "c" => Register::C,
            "d" => Register::D,
            _ => return Err(format!("invalid register: {}", s)),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Accessor {
    Value(i64),
    Register(Register),
}

impl FromStr for Accessor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse() {
            Ok(numeric) => Accessor::Value(numeric),
            Err(_) => Accessor::Register(s.parse()?),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpCode {
    Cpy(Accessor, Accessor),
    Jnz(Accessor, Accessor),
    Inc(Register),
    Dec(Register),
    Tgl(Accessor),
}

impl FromStr for OpCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, operands) = s
            .split_once(' ')
            .ok_or_else(|| format!("invalid instruction: {}", s))?;

        let opcode = match instruction {
            "inc" => OpCode::Inc(operands.parse()?),
            "dec" => OpCode::Dec(operands.parse()?),
            "tgl" => OpCode::Tgl(operands.parse()?),
            "jnz" => {
                let (x, y) = operands
                    .split_once(' ')
                    .ok_or_else(|| format!("invalid instruction: {}", s))?;

                OpCode::Jnz(x.parse()?, y.parse()?)
            }
            "cpy" => {
                let (x, y) = operands
                    .split_once(' ')
                    .ok_or_else(|| format!("invalid instruction: {}", s))?;

                OpCode::Cpy(x.parse()?, y.parse()?)
            }

            _ => return Err(format!("invalid instruction: {}", s)),
        };

        Ok(opcode)
    }
}

pub fn part_one(input: &[OpCode]) -> i64 {
    solve(input, [7, 0, 0, 0])
}

pub fn part_two(input: &[OpCode]) -> i64 {
    solve(input, [12, 0, 0, 0])
}

fn solve(instructions: &[OpCode], registers: [i64; 4]) -> i64 {
    let mut instructions = instructions.to_vec();
    let mut registers = registers; // [a,b,c,d]
    let mut ip = 0;

    while ip < instructions.len() {
        match instructions[ip] {
            OpCode::Cpy(src, dst) => match dst {
                Accessor::Value(_) => { /* skip invalid instruction */ }
                Accessor::Register(x) => {
                    registers[x as usize] = match src {
                        Accessor::Value(value) => value,
                        Accessor::Register(x) => registers[x as usize],
                    }
                }
            },
            OpCode::Jnz(value, dist) => {
                let value = match value {
                    Accessor::Value(value) => value,
                    Accessor::Register(x) => registers[x as usize],
                };

                let dist = match dist {
                    Accessor::Value(value) => value,
                    Accessor::Register(x) => registers[x as usize],
                } as isize;

                if value != 0 {
                    ip = (ip as isize + dist - 1)
                        .try_into()
                        .expect("invalid instruction pointer");
                }
            }
            OpCode::Inc(x) => registers[x as usize] += 1,
            OpCode::Dec(x) => registers[x as usize] -= 1,
            OpCode::Tgl(dist) => {
                let dist = match dist {
                    Accessor::Value(value) => value,
                    Accessor::Register(x) => registers[x as usize],
                } as isize;
                let idx = ip.checked_add_signed(dist).unwrap();

                if idx < instructions.len() {
                    instructions[idx] = match instructions[idx] {
                        OpCode::Cpy(x, y) => OpCode::Jnz(x, y),
                        OpCode::Jnz(x, y) => OpCode::Cpy(x, y),
                        OpCode::Inc(x) => OpCode::Dec(x),
                        OpCode::Dec(x) => OpCode::Inc(x),
                        OpCode::Tgl(x) => match x {
                            Accessor::Value(_) => {
                                panic!("cannot issuer an INC <value> instruction")
                            }
                            Accessor::Register(r) => OpCode::Inc(r),
                        },
                    };
                }
            }
        }

        ip += 1;
    }

    registers[Register::A as usize]
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(11739, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");

        let answer = part_two(&input);
        assert_eq!(479008299, answer);
    }
}
