use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Instruction {
    Mask(Masks),
    Mem(u64, u64),
}

#[derive(Debug, PartialEq)]
enum InstructionV2 {
    Mask(MaskV2),
    Mem(u64, u64),
}

#[derive(Debug, PartialEq)]
struct MaskV2 {
    floating: Vec<usize>,
    mask1: u64,
}

type Masks = (u64, u64);

const NUM_BITS: usize = 36;

fn apply_masks(value: u64, masks: Masks) -> u64 {
    let masked0 = value & !masks.0;
    let masked = masked0 | masks.1;
    return masked;
}

fn apply_v2_mask(address: u64, mask: &MaskV2) -> HashSet<u64> {
    let bitmasked = address | mask.mask1;
    let mut addrs = HashSet::new();

    for i in &mask.floating {
        if addrs.is_empty() {
            addrs.insert(bitmasked & !(1 << i));
            addrs.insert(bitmasked | (1 << i));
        } else {
            for addr in addrs.to_owned() {
                addrs.insert(addr & !(1 << i));
                addrs.insert(addr | (1 << i));
            }
        }
    }
    return addrs;
}

fn parse_mask(mask_str: &str) -> Masks {
    let mut mask0 = 0;
    let mut mask1 = 0;

    for (i, c) in mask_str.chars().enumerate() {
        match c {
            '0' => mask0 |= 1 << (NUM_BITS - 1 - i),
            '1' => mask1 |= 1 << (NUM_BITS - 1 - i),
            'X' => (),
            _ => panic!("Unexpected mask char: {}", c),
        }
    }

    return (mask0, mask1);
}

fn parse_v2_mask(mask_str: &str) -> MaskV2 {
    let mut floating = Vec::new();
    let mut mask1 = 0;

    for (i, c) in mask_str.chars().enumerate() {
        match c {
            '0' => (),
            '1' => mask1 |= 1 << (NUM_BITS - 1 - i),
            'X' => floating.push(NUM_BITS - 1 - i),
            _ => panic!("Unexpected mask char: {}", c),
        }
    }
    return MaskV2{
        floating: floating,
        mask1: mask1,
    };
}

fn parse_program(input: &str) -> Vec<Instruction> {
    lazy_static! {
        static ref MASK_RE: Regex = Regex::new(r"^mask = ([X01]+)$").unwrap();
        static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    }

    let program = input.lines().map(|l| {
        match MASK_RE.captures(l) {
            Some(caps) => {
                let masks = parse_mask(&caps[1]);
                return Instruction::Mask(masks);
            },
            None => (),
        };
        match MEM_RE.captures(l) {
            Some(caps) => {
                let addr = caps[1].parse().unwrap();
                let val = caps[2].parse().unwrap();
                return Instruction::Mem(addr, val);
            },
            None => (),
        }
        panic!("Unexpected instruction: {}", l);
    }).collect();
    return program;
}

fn parse_v2_program(input: &str) -> Vec<InstructionV2> {
    lazy_static! {
        static ref MASK_RE: Regex = Regex::new(r"^mask = ([X01]+)$").unwrap();
        static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    }

    let program = input.lines().map(|l| {
        match MASK_RE.captures(l) {
            Some(caps) => {
                let masks = parse_v2_mask(&caps[1]);
                return InstructionV2::Mask(masks);
            },
            None => (),
        };
        match MEM_RE.captures(l) {
            Some(caps) => {
                let addr = caps[1].parse().unwrap();
                let val = caps[2].parse().unwrap();
                return InstructionV2::Mem(addr, val);
            },
            None => (),
        }
        panic!("Unexpected instruction: {}", l);
    }).collect();
    return program;
}

fn run_program(program: &[Instruction]) -> HashMap<u64, u64> {
    let mut memory = HashMap::new();
    let mut masks = (0, 0);

    for &instr in program {
        match instr {
            Instruction::Mask(ms) => {
                masks = ms;
            },
            Instruction::Mem(addr, val) => {
                memory.insert(addr, apply_masks(val, masks));
            },
        }
    }
    return memory;
}

fn run_v2_program(program: &[InstructionV2]) -> HashMap<u64, u64> {
    let mut memory = HashMap::new();
    let mut mask = &MaskV2{floating: Vec::new(), mask1: 0};

    for instr in program {
        match instr {
            InstructionV2::Mask(m) => mask = m,
            InstructionV2::Mem(addr, val) => {
                let addrs = apply_v2_mask(*addr, mask);
                for a in addrs {
                    memory.insert(a, *val);
                }
            }
        }
    }
    return memory;
}

fn part1(program: &[Instruction]) {
    let memory = run_program(program);
    let sum: u64 = memory.values().sum();
    println!("Sum for part 1: {}", sum);
}

fn part2(program: &[InstructionV2]) {
    let memory = run_v2_program(program);
    let sum: u64 = memory.values().sum();
    println!("Sum for part 2: {}", sum);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = parse_program(&input);
    let program_v2 = parse_v2_program(&input);

    part1(&program);
    part2(&program_v2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_masks() {
        let mask0 = 0b000000000000000000000000000000000010;
        let mask1 = 0b000000000000000000000000000001000000;
        let masks = (mask0, mask1);
        assert_eq!(apply_masks(11, masks), 73);
        assert_eq!(apply_masks(101, masks), 101);
        assert_eq!(apply_masks(0, masks), 64);
    }

    #[test]
    fn test_parse_masks() {
        let mask_str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let mask0 = 0b000000000000000000000000000000000010;
        let mask1 = 0b000000000000000000000000000001000000;
        let expected = (mask0, mask1);
        let actual = parse_mask(mask_str);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_run_program() {
        let mask0 = 0b000000000000000000000000000000000010;
        let mask1 = 0b000000000000000000000000000001000000;
        let masks = (mask0, mask1);
        let program = vec![
            Instruction::Mask(masks),
            Instruction::Mem(8, 11),
            Instruction::Mem(7, 101),
            Instruction::Mem(8, 0),
        ];
        let mut expected: HashMap<u64, u64> = HashMap::new();
        expected.insert(7, 101);
        expected.insert(8, 64);
        let actual = run_program(&program);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_program() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let mask0 = 0b000000000000000000000000000000000010;
        let mask1 = 0b000000000000000000000000000001000000;
        let masks = (mask0, mask1);
        let expected = vec![
            Instruction::Mask(masks),
            Instruction::Mem(8, 11),
            Instruction::Mem(7, 101),
            Instruction::Mem(8, 0),
        ];
        let actual = parse_program(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_v2_mask() {
        let mask_str = "000000000000000000000000000000X1001X";
        let mask1 = 0b000000000000000000000000000000010010;
        let floating = vec![5, 0];
        let expected = MaskV2{mask1, floating};
        let actual = parse_v2_mask(mask_str);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_apply_v2_mask() {
        let mask1 = 0b000000000000000000000000000000010010;
        let floating = vec![5, 0];
        let mask = MaskV2{mask1, floating};
        let expected: HashSet<u64> = [26, 27, 58, 59].iter().cloned().collect();
        let actual = apply_v2_mask(42, &mask);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_v2_program() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let mask1_1 = 0b000000000000000000000000000000010010;
        let floating_1 = vec![5, 0];
        let mask_1 = MaskV2{mask1: mask1_1, floating: floating_1};
        let mask1_2 = 0;
        let floating_2 = vec![3, 1, 0];
        let mask_2 = MaskV2{mask1: mask1_2, floating: floating_2};
        let expected = vec![
            InstructionV2::Mask(mask_1),
            InstructionV2::Mem(42, 100),
            InstructionV2::Mask(mask_2),
            InstructionV2::Mem(26, 1),
        ];
        let actual = parse_v2_program(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_run_v2_program() {
        let mask1_1 = 0b000000000000000000000000000000010010;
        let floating_1 = vec![5, 0];
        let mask_1 = MaskV2{mask1: mask1_1, floating: floating_1};
        let mask1_2 = 0;
        let floating_2 = vec![3, 1, 0];
        let mask_2 = MaskV2{mask1: mask1_2, floating: floating_2};
        let program = vec![
            InstructionV2::Mask(mask_1),
            InstructionV2::Mem(42, 100),
            InstructionV2::Mask(mask_2),
            InstructionV2::Mem(26, 1),
        ];
        let mut expected: HashMap<u64, u64> = HashMap::new();
        expected.insert(58, 100);
        expected.insert(59, 100);
        expected.insert(16, 1);
        expected.insert(17, 1);
        expected.insert(18, 1);
        expected.insert(19, 1);
        expected.insert(24, 1);
        expected.insert(25, 1);
        expected.insert(26, 1);
        expected.insert(27, 1);
        let actual = run_v2_program(&program);
        assert_eq!(actual, expected);
    }
}
