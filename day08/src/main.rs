use std::collections::HashSet;
use std::io::{self, Read};
use Instruction::*;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Instruction {
    fn from_str(instr_str: &str) -> Self {
        let parts: Vec<_> = instr_str.split_whitespace().collect();
        let value: isize = parts[1].parse().unwrap();

        match parts[0] {
            "acc" => Self::Acc(value),
            "jmp" => Self::Jmp(value),
            "nop" => Self::Nop(value),
            _ => panic!("Unknown instruction: {}", instr_str),
        }
    }
}

fn flip_jmp_nop(init_program: &[Instruction]) -> (Vec<Instruction>, isize) {
    for i in 0..init_program.len() {
        let instr = &init_program[i];
        let mut program = Vec::new();
        let (completed, acc) = match instr {
            Acc(_) => continue,
            Jmp(value) => {
                program.extend_from_slice(init_program);
                program[i] = Nop(*value);
                run_program(&program, 0, 0)
            }
            Nop(value) => {
                program.extend_from_slice(init_program);
                program[i] = Jmp(*value);
                run_program(&program, 0, 0)
            }
        };
        if completed {
            return (program, acc);
        }
    }
    return (vec![], 0);
}

fn parse_program(input: &str) -> Vec<Instruction> {
    let program = input.lines().map(|l| Instruction::from_str(l)).collect();
    return program;
}

fn run_program(program: &[Instruction], init_pc: isize, init_acc: isize) -> (bool, isize) {
    let mut visited = HashSet::new();
    let size = program.len() as isize;
    let mut pc = init_pc;
    let mut acc = init_acc;

    while !visited.contains(&pc) && pc < size {
        visited.insert(pc);
        match program[pc as usize] {
            Acc(value) => acc += value,
            Jmp(offset) => {
                pc += offset;
                continue;
            }
            Nop(_) => (),
        }
        pc += 1;
    }

    if pc < size {
        return (false, acc);
    } else {
        return (true, acc);
    }
}

fn part1(program: &[Instruction]) {
    let (_, value) = run_program(program, 0, 0);
    println!("Value before loop for part 1: {}", value);
}

fn part2(program: &[Instruction]) {
    let (_, value) = flip_jmp_nop(program);
    println!("Value after terminate for part 2: {}", value);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = parse_program(&input);

    part1(&program);
    part2(&program);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        return input;
    }

    #[test]
    fn test_instruction_from_str_1() {
        let instr = Instruction::from_str("nop +0");
        assert_eq!(instr, Nop(0));
    }

    #[test]
    fn test_instruction_from_str_2() {
        let instr = Instruction::from_str("acc +1");
        assert_eq!(instr, Acc(1));
    }

    #[test]
    fn test_instruction_from_str_3() {
        let instr = Instruction::from_str("jmp -3");
        assert_eq!(instr, Jmp(-3));
    }

    #[test]
    fn test_parse_program() {
        let input = get_input();
        let program = parse_program(input);
        let expected = vec![
            Nop(0),
            Acc(1),
            Jmp(4),
            Acc(3),
            Jmp(-3),
            Acc(-99),
            Acc(1),
            Jmp(-4),
            Acc(6),
        ];
        assert_eq!(program, expected);
    }

    #[test]
    fn test_run_program_1() {
        let input = get_input();
        let program = parse_program(input);
        assert_eq!(run_program(&program, 0, 0), (false, 5));
    }

    #[test]
    fn test_run_program_2() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";
        let program = parse_program(input);
        assert_eq!(run_program(&program, 0, 0), (true, 8));
    }

    #[test]
    fn test_flip_jmp_nop() {
        let input = get_input();
        let program = parse_program(input);
        let expected_prog = vec![
            Nop(0),
            Acc(1),
            Jmp(4),
            Acc(3),
            Jmp(-3),
            Acc(-99),
            Acc(1),
            Nop(-4),
            Acc(6),
        ];
        assert_eq!(flip_jmp_nop(&program), (expected_prog, 8));
    }
}
