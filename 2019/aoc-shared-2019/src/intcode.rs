use std::collections::VecDeque;

pub trait Input {
    fn read(&mut self) -> Option<isize>;
}

pub trait Output {
    fn write(&mut self, value: isize);
}

impl Input for std::iter::Once<isize> {
    fn read(&mut self) -> Option<isize> {
        self.next()
    }
}

impl<A: Iterator<Item = isize>, B: Iterator<Item = isize>> Input for std::iter::Chain<A, B> {
    fn read(&mut self) -> Option<isize> {
        self.next()
    }
}

impl Input for std::iter::Empty<isize> {
    fn read(&mut self) -> Option<isize> {
        self.next()
    }
}

impl Input for VecDeque<isize> {
    fn read(&mut self) -> Option<isize> {
        self.pop_front()
    }
}

impl Output for VecDeque<isize> {
    fn write(&mut self, value: isize) {
        self.push_back(value);
    }
}

impl Output for Vec<isize> {
    fn write(&mut self, value: isize) {
        self.push(value);
    }
}

impl Output for isize {
    fn write(&mut self, value: isize) {
        *self = value;
    }
}

#[derive(Debug, Clone)]
pub struct Computer {
    mem: Vec<isize>,
    ip: usize,
    rb: isize,
}

impl Computer {
    pub fn new(mem: Vec<isize>) -> Self {
        Self { mem, ip: 0, rb: 0 }
    }

    pub fn run(&mut self, input: &mut dyn Input, output: &mut dyn Output) -> Outcome {
        loop {
            let instruction = Instruction::decode(self.mem[self.ip]);
            if let Some(outcome) = instruction.execute(self, input, output) {
                return outcome;
            }
        }
    }

    pub fn mem(&mut self, addr: usize) -> &mut isize {
        &mut self.mem[addr]
    }

    fn arg(&mut self, mode: Mode, arg: usize) -> &mut isize {
        match mode {
            Mode::Position => {
                let idx = self.ip + arg;
                if self.mem.len() <= idx {
                    self.mem.resize(idx + 1, 0);
                }

                let pos = self.mem[idx] as usize;
                if self.mem.len() <= pos {
                    self.mem.resize(pos + 1, 0);
                }

                &mut self.mem[pos]
            }
            Mode::Immediate => {
                let idx = self.ip + arg;
                if self.mem.len() <= idx {
                    self.mem.resize(idx + 1, 0);
                }

                &mut self.mem[idx]
            }

            Mode::Relative => {
                let idx = self.ip + arg;
                if self.mem.len() <= idx {
                    self.mem.resize(idx + 1, 0);
                }

                let pos = (self.rb + self.mem[idx]) as usize;
                if self.mem.len() <= pos {
                    self.mem.resize(pos + 1, 0);
                }

                &mut self.mem[pos]
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Outcome {
    Halt,
    Fault(Fault),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Fault {
    Error(String),
    InsufficientInput,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<isize> for Mode {
    type Error = String;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => return Err(format!("Invalid mode: {:?}", value)),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Inp(Mode),
    Out(Mode),
    JiT(Mode, Mode),
    JiF(Mode, Mode),
    LT(Mode, Mode, Mode),
    EQ(Mode, Mode, Mode),
    RB(Mode),
    Hlt,
}

impl Instruction {
    pub fn decode(n: isize) -> Self {
        assert!(n >= 0);

        let instr = n % 100;
        let modes = n / 100;

        match instr {
            1 => {
                let (a, b, c) = decode_3(modes);
                Instruction::Add(a, b, c)
            }

            2 => {
                let (a, b, c) = decode_3(modes);
                Instruction::Mul(a, b, c)
            }

            3 => {
                let mode = decode_1(modes);
                Instruction::Inp(mode)
            }

            4 => {
                let mode = decode_1(modes);
                Instruction::Out(mode)
            }

            5 => {
                let (a, b) = decode_2(modes);
                Instruction::JiT(a, b)
            }

            6 => {
                let (a, b) = decode_2(modes);
                Instruction::JiF(a, b)
            }

            7 => {
                let (a, b, c) = decode_3(modes);
                Instruction::LT(a, b, c)
            }

            8 => {
                let (a, b, c) = decode_3(modes);
                Instruction::EQ(a, b, c)
            }

            9 => {
                let mode = decode_1(modes);
                Instruction::RB(mode)
            }

            99 => Instruction::Hlt,
            _ => panic!("Cannot decode int-code opcode: {:?}", n),
        }
    }

    pub fn execute(
        self,
        vm: &mut Computer,
        input: &mut dyn Input,
        output: &mut dyn Output,
    ) -> Option<Outcome> {
        match self {
            Instruction::Add(m1, m2, md) => {
                exec_3(vm, m1, m2, md, |dst, v1, v2| *dst = v1 + v2);
            }

            Instruction::Mul(m1, m2, md) => {
                exec_3(vm, m1, m2, md, |dst, v1, v2| *dst = v1 * v2);
            }

            Instruction::Inp(md) => match input.read() {
                None => return Some(Outcome::Fault(Fault::InsufficientInput)),
                Some(value) => *vm.arg(md, 1) = value,
            },

            Instruction::Out(md) => {
                output.write(*vm.arg(md, 1));
            }

            Instruction::JiT(m1, m2) => {
                if 0 != *vm.arg(m1, 1) {
                    return exec_jump(vm, m2, 2);
                }
            }

            Instruction::JiF(m1, m2) => {
                if 0 == *vm.arg(m1, 1) {
                    return exec_jump(vm, m2, 2);
                }
            }

            Instruction::LT(m1, m2, md) => {
                exec_3(vm, m1, m2, md, |dst, v1, v2| *dst = (v1 < v2) as isize);
            }

            Instruction::EQ(m1, m2, md) => {
                exec_3(vm, m1, m2, md, |dst, v1, v2| *dst = (v1 == v2) as isize);
            }

            Instruction::RB(m) => {
                vm.rb += *vm.arg(m, 1);
            }

            Instruction::Hlt => return Some(Outcome::Halt),
        }

        vm.ip += self.increment();
        None
    }

    pub fn increment(self) -> usize {
        match self {
            Instruction::Add(_, _, _) => 4,
            Instruction::Mul(_, _, _) => 4,
            Instruction::Inp(_) => 2,
            Instruction::Out(_) => 2,
            Instruction::JiT(_, _) => 3,
            Instruction::JiF(_, _) => 3,
            Instruction::LT(_, _, _) => 4,
            Instruction::EQ(_, _, _) => 4,
            Instruction::RB(_) => 2,
            Instruction::Hlt => 0,
        }
    }
}

fn exec_3<F: Fn(&mut isize, isize, isize)>(
    vm: &mut Computer,
    m1: Mode,
    m2: Mode,
    md: Mode,
    action: F,
) {
    let v1 = *vm.arg(m1, 1);
    let v2 = *vm.arg(m2, 2);
    let dst = vm.arg(md, 3);
    action(dst, v1, v2);
}

fn exec_jump(vm: &mut Computer, mode: Mode, arg: usize) -> Option<Outcome> {
    return match (*vm.arg(mode, arg)).try_into() {
        Ok(addr) => {
            vm.ip = addr;
            None
        }

        Err(_) => {
            let msg = format!(
                "Cannot jump to negative address. IP={}; RB={}; ARG={}, MODE={:?}",
                vm.ip, vm.rb, arg, mode
            );

            Some(Outcome::Fault(Fault::Error(msg)))
        }
    };
}

fn decode_3(modes: isize) -> (Mode, Mode, Mode) {
    assert!(modes >= 0);

    let mut code = modes;

    let mode_1 = Mode::try_from(code % 10).unwrap();
    code /= 10;

    let mode_2 = Mode::try_from(code % 10).unwrap();
    code /= 10;

    let mode_3 = Mode::try_from(code % 10).unwrap();

    (mode_1, mode_2, mode_3)
}

fn decode_2(modes: isize) -> (Mode, Mode) {
    assert!(modes >= 0);

    let mut code = modes;

    let mode_1 = Mode::try_from(code % 10).unwrap();
    code /= 10;

    let mode_2 = Mode::try_from(code % 10).unwrap();

    (mode_1, mode_2)
}

fn decode_1(modes: isize) -> Mode {
    assert!(modes >= 0);

    Mode::try_from(modes % 10).unwrap()
}
