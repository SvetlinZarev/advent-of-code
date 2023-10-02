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
    Jnz(Accessor, isize),
    Inc(Register),
    Dec(Register),
    Out(Accessor),
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
            "jnz" => {
                let (x, y) = operands
                    .split_once(' ')
                    .ok_or_else(|| format!("invalid instruction: {}", s))?;

                OpCode::Jnz(
                    x.parse()?,
                    y.parse()
                        .map_err(|_e| format!("invalid instruction: {}", s))?,
                )
            }
            "cpy" => {
                let (x, y) = operands
                    .split_once(' ')
                    .ok_or_else(|| format!("invalid instruction: {}", s))?;

                OpCode::Cpy(x.parse()?, y.parse()?)
            }
            "out" => OpCode::Out(operands.parse()?),
            _ => return Err(format!("invalid instruction: {}", s)),
        };

        Ok(opcode)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Expectation {
    Zero,
    One,
}

pub fn part_one(input: &[OpCode]) -> i64 {
    for init_value in 0.. {
        if solve(input, [init_value, 0, 0, 0], 1024) {
            return init_value;
        }
    }

    unreachable!()
}


fn solve(instructions: &[OpCode], registers: [i64; 4], watch_cycles: u32) -> bool {
    let mut registers = registers; // [a,b,c,d]
    let mut ip = 0;

    let mut expectation = Expectation::Zero;
    let mut watch_cycles = watch_cycles;

    while ip < instructions.len() {
        match instructions[ip] {
            OpCode::Cpy(src, dst) => match dst {
                Accessor::Value(_) => panic!("cannot copy value into a 'value'"),
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

                if value != 0 {
                    ip = (ip as isize + dist - 1)
                        .try_into()
                        .expect("invalid instruction pointer");
                }
            }
            OpCode::Inc(x) => registers[x as usize] += 1,
            OpCode::Dec(x) => registers[x as usize] -= 1,
            OpCode::Out(value) => {
                let value = match value {
                    Accessor::Value(value) => value,
                    Accessor::Register(x) => registers[x as usize]
                };

                match expectation {
                    Expectation::Zero if value == 0 => expectation = Expectation::One,
                    Expectation::One if value == 1 => expectation = Expectation::Zero,
                    _ => return false,
                }

                watch_cycles -= 1;
                if watch_cycles == 0 {
                    return true;
                }
            }
        }

        ip += 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use crate::part_one;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(196, answer);
    }
}
