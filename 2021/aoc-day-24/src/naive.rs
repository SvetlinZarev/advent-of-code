use std::str::FromStr;

pub type Int = i64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operand {
    Reg(u8),
    Val(i8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instr {
    Inp(u8),
    Add(u8, Operand),
    Mul(u8, Operand),
    Div(u8, Operand),
    Mod(u8, Operand),
    Eql(u8, Operand),
}

impl FromStr for Instr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (inst, operands) = s
            .split_once(' ')
            .ok_or_else(|| format!("Cannot split instruction: {:?}", s))?;

        let op = match inst {
            "inp" => Instr::Inp(parse_register(operands)?),
            _ => {
                let (l, r) = operands
                    .split_once(' ')
                    .ok_or_else(|| format!("Cannot split operands: {:?}", s))?;
                let l = parse_register(l)?;
                let r = parse_operand(r)?;

                match inst {
                    "add" => Instr::Add(l, r),
                    "mul" => Instr::Mul(l, r),
                    "div" => Instr::Div(l, r),
                    "mod" => Instr::Mod(l, r),
                    "eql" => Instr::Eql(l, r),
                    _ => return Err(format!("unsupported instruction: {:?}", s)),
                }
            }
        };

        Ok(op)
    }
}

fn parse_operand(s: &str) -> Result<Operand, String> {
    match s {
        "w" => Ok(Operand::Reg(0)),
        "x" => Ok(Operand::Reg(1)),
        "y" => Ok(Operand::Reg(2)),
        "z" => Ok(Operand::Reg(3)),
        _ => Ok(Operand::Val(
            s.parse().map_err(|_| format!("invalid operand: {}", s))?,
        )),
    }
}

fn parse_register(s: &str) -> Result<u8, String> {
    match s {
        "w" => Ok(0),
        "x" => Ok(1),
        "y" => Ok(2),
        "z" => Ok(3),
        _ => Err(format!("invalid register: {:?}", s)),
    }
}

pub fn monad(instr: &[Instr], input: &[u8]) -> bool {
    let mut reg = [0; 4];
    let mut inp = input;

    for op in instr.iter().copied() {
        match op {
            Instr::Inp(idx) => {
                reg[idx as usize] = inp[0] as Int;
                inp = &inp[1..];
            }
            Instr::Add(i, j) => match j {
                Operand::Reg(r) => {
                    reg[i as usize] += reg[r as usize];
                }
                Operand::Val(v) => {
                    reg[i as usize] += v as Int;
                }
            },
            Instr::Mul(i, j) => match j {
                Operand::Reg(r) => {
                    reg[i as usize] *= reg[r as usize];
                }
                Operand::Val(v) => {
                    reg[i as usize] *= v as Int;
                }
            },
            Instr::Div(i, j) => match j {
                Operand::Reg(r) => {
                    reg[i as usize] /= reg[r as usize];
                }
                Operand::Val(v) => {
                    reg[i as usize] /= v as Int;
                }
            },
            Instr::Mod(i, j) => match j {
                Operand::Reg(r) => {
                    reg[i as usize] %= reg[r as usize];
                }
                Operand::Val(v) => {
                    reg[i as usize] %= v as Int;
                }
            },
            Instr::Eql(i, j) => match j {
                Operand::Reg(r) => {
                    reg[i as usize] = (reg[i as usize] == reg[r as usize]) as Int;
                }
                Operand::Val(v) => {
                    reg[i as usize] = (reg[i as usize] == v as Int) as Int;
                }
            },
        }
    }

    reg[3] == 0
}
