use aoc_shared_2019::intcode::{Computer, Outcome};

pub fn part_one(program: &[isize]) -> isize {
    let mut vm = Computer::new(program.to_vec());
    let mut input = std::iter::once(1);
    let mut output = 0;

    if let Outcome::Fault(f) = vm.run(&mut input, &mut output) {
        panic!("{:?}", f);
    }

    output
}

pub fn part_two(program: &[isize]) -> isize {
    let mut vm = Computer::new(program.to_vec());
    let mut input = std::iter::once(2);
    let mut output = 0;

    if let Outcome::Fault(f) = vm.run(&mut input, &mut output) {
        panic!("{:?}", f);
    }

    output
}
