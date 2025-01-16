#![feature(variant_count)]
#![feature(test)]

extern crate test;

use core::panic;
use std::{
    fmt::Display,
    io::{Error, ErrorKind, Read},
    ops::{Index, IndexMut},
};

use aoc_utils::parse;
use itertools::{EitherOrBoth, Itertools};
advent_of_code::solution!(17);

#[derive(Clone, Copy, Debug)]
struct Register(usize);

const REGISTER_COUNT: usize = 3;

type Value = u64;
type Out = u8;

#[derive(Clone, Copy, Debug)]
struct Operand(u8);

#[derive(Clone, Copy, Debug)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<u8> for Operand {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..7 => Ok(Operand(value)),
            _ => Err(Error::other("Invalid operand")),
        }
    }
}

impl TryFrom<u8> for Opcode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Adv),
            1 => Ok(Opcode::Bxl),
            2 => Ok(Opcode::Bst),
            3 => Ok(Opcode::Jnz),
            4 => Ok(Opcode::Bxc),
            5 => Ok(Opcode::Out),
            6 => Ok(Opcode::Bdv),
            7 => Ok(Opcode::Cdv),
            opcode => Err(Error::other("Invalid opcode")),
        }
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|x| format!("{:>8x}", x)).join(" ")
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\t{:>6} {}",
            format!("{:?}", self.opcode),
            self.operand.0
        )
    }
}

impl TryFrom<&[u8; 2]> for Instruction {
    type Error = std::io::Error;

    fn try_from([opcode, operand]: &[u8; 2]) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(*opcode)?;
        let operand = Operand::try_from(*operand)?;
        Ok(Instruction { opcode, operand })
    }
}

#[derive(Clone, Debug)]
struct Registers([Value; REGISTER_COUNT]);

impl Registers {
    pub fn new(init: &[Value; REGISTER_COUNT]) -> Self {
        Registers(init.to_owned())
    }
}

#[derive(Clone, Debug)]
struct Cpu {
    memory: Vec<Instruction>,
    registers: Registers,
    program_counter: usize,
}

impl Cpu {
    pub fn output(&mut self) -> String {
        format!("{}", self.flatten().format(","))
    }
}

impl Cpu {
    pub fn read(input: &str) -> std::io::Result<Self> {
        let mut nums = parse::nums_positive::<Value>(input);
        let mut registers: [Value; REGISTER_COUNT] = nums
            .get(0..REGISTER_COUNT)
            .ok_or(Error::other("not enough numbers to init registers"))?
            .try_into()
            .unwrap();
        let registers = Registers::new(&registers);
        let memory = nums
            .get(REGISTER_COUNT..)
            .unwrap()
            .iter()
            .map(|&x| x as u8)
            .chunks(2)
            .into_iter()
            .map(|x| x.collect_array::<2>())
            .map(|x| x.and_then(|y| Instruction::try_from(&y).ok()))
            .collect::<Option<Vec<Instruction>>>()
            .ok_or(Error::other("instructions could not be parsed"))?;
        Ok(Cpu {
            memory,
            registers,
            program_counter: 0,
        })
    }

    fn a(&mut self) -> &mut Value {
        &mut self.registers.0[0]
    }
    fn b(&mut self) -> &mut Value {
        &mut self.registers.0[1]
    }
    fn c(&mut self) -> &mut Value {
        &mut self.registers.0[2]
    }

    fn combo(&self, operand: Operand) -> Value {
        match operand.0 {
            0..=3 => operand.0 as Value,
            4..=6 => self.registers.0[operand.0 as usize - 4],
            _ => panic!("a combo operand in this range should never appear in a program"),
        }
    }
    fn literal(&self, operand: Operand) -> Value {
        operand.0 as Value
    }

    fn adv(&mut self, operand: Operand) -> Value {
        *self.a() >> self.combo(operand) as Value
    }

    pub fn reset(&mut self) {
        self.program_counter = 0;
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let memory = self.memory.iter().join("\n");
        write!(
            f,
            "CPU\nA: {}\nB: {}\nC: {}\n{memory}",
            self.registers.0[0], self.registers.0[1], self.registers.0[2],
        )
    }
}

impl Iterator for Cpu {
    type Item = Option<Out>;

    fn next(&mut self) -> Option<Self::Item> {
        let i @ &Instruction { opcode, operand } = self.memory.get(self.program_counter)?;
        // println!("{}\t{}", *i, self.registers);
        let a = self.a();
        let mut out = None;
        // dbg!(opcode, self.program_counter);
        self.program_counter += 1;
        match opcode {
            Opcode::Adv => *self.a() = self.adv(operand),
            Opcode::Bxl => {
                *self.b() = *self.b() ^ self.literal(operand);
            }
            Opcode::Bst => *self.b() = truncate(self.combo(operand)),
            Opcode::Jnz => {
                if *self.a() != 0 {
                    self.program_counter = self.literal(operand) as usize
                }
            }
            Opcode::Bxc => *self.b() = *self.b() ^ *self.c(),
            Opcode::Out => out = Some(truncate(self.combo(operand)) as u8),
            Opcode::Bdv => *self.b() = self.adv(operand),
            Opcode::Cdv => *self.c() = self.adv(operand),
        }
        Some(out)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let cpu = Cpu::read(input).unwrap();
    println!("{}\n----------------------------", &cpu);
    println!("initial method: ----------------------------------");
    let outs: Vec<_> = cpu
        .clone()
        // .inspect(|x| {
        //     dbg!(x);
        // })
        .flatten()
        .collect();
    println!("im_fkin_stupid: ----------------------------------");
    // let outs = im_fkin_stupid(cpu);
    Some(outs.iter().join(","))
}

fn cmp(cpu: &mut Cpu, program: &[u8], a: Value) -> bool {
    cpu.reset();
    cpu.registers.0[0] = a;
    cpu.flatten()
        .zip_longest(program.iter().copied())
        // .inspect(|x| {
        //     dbg!(x);
        // })
        .all(zip_eq)
}

pub fn part_two(input: &str) -> Option<u64> {
    // return manual_driver();
    let mut cpu = Cpu::read(input).unwrap();
    let hash = get_hash(&cpu.memory);
    let program = parse::nums_positive::<i64>(input);
    let goal: Vec<u8> = program.iter().get(3..).map(|&x| x as u8).rev().collect();
    solve(&hash, &goal, 0)
    // const START: i64 = 0;
    // dbg!(&program);
    // // (0i64..i64::MAX)
    // //     .into_par_iter()
    // //     .inspect(|x| {
    // //         if (x & (1 << 25) - 1 == 0) {
    // //             println!("0x{x:<20x} {x}")
    // //         }
    // //     })
    // //     .find_any(|&a| cmp(&mut cpu.clone(), program, a))
    // (START..)
    //     .inspect(|x| {
    //         if (x & (1 << 25) - 1 == 0) {
    //             println!("0x{x:<20x} {x}")
    //         }
    //     })
    //     .find(|&a| cmp(&mut cpu.clone(), &program[..], a))
}

fn combo(reg: &[Value], operand: u8) -> Value {
    match operand {
        0..=3 => operand as Value,
        4..=6 => reg[(operand - 4) as usize],
        _ => panic!("Invalid operand"),
    }
}

fn literal(operand: u8) -> Value {
    assert!((0..8).contains(&operand));
    operand as Value
}

fn adv(reg: &[Value], operand: u8) -> Value {
    reg[0] >> combo(reg, operand)
}
fn truncate(value: Value) -> Value {
    value & 0b111
}

fn im_fkin_stupid(cpu: Cpu) -> Vec<Value> {
    let mut out = vec![];
    // let (mut a, mut b, mut c) = (cpu.registers.0[0], cpu.registers.0[1], cpu.registers.0[2]);
    let mut reg = cpu.registers.0;
    let mut pc = 0;
    while let Some(
        i @ &Instruction {
            opcode,
            operand: Operand(operand),
        },
    ) = cpu.memory.get(pc)
    {
        pc += 1;
        match opcode {
            Opcode::Adv => reg[0] = adv(&reg, operand),
            Opcode::Bxl => reg[1] = reg[1] ^ literal(operand),
            Opcode::Bst => reg[1] = truncate(combo(&reg, operand)),
            Opcode::Jnz => {
                if reg[0] != 0 {
                    pc = literal(operand) as usize
                }
            }
            Opcode::Bxc => reg[1] ^= reg[2],
            Opcode::Out => out.push(truncate(combo(&reg, operand))),
            Opcode::Bdv => reg[1] = adv(&reg, operand),
            Opcode::Cdv => reg[2] = adv(&reg, operand),
        }
    }
    out
}
fn zip_eq<T: Eq>(x: EitherOrBoth<T>) -> bool {
    x.is_both()
        && match x {
            EitherOrBoth::Both(a, b) => a == b,
            _ => false,
        }
}

// fn part_two_manual(input: &str) -> Option<u64> {
//     let program = parse::nums_positive::<i64>(input);
//     let program: Vec<u8> = program.iter().get(3..).map(|&x| x as u8).collect();
//     dbg!(&program);
//     let mut cpu = Cpu::read(input).unwrap();

//     let rev_prog = program.iter().rev().copied().collect_vec();

//     manual_solve(0, &rev_prog)
// }

// fn manual_driver() -> Option<u64> {
//     let mut program = [2, 4, 1, 1, 7, 5, 4, 0, 0, 3, 1, 6, 5, 5, 3, 0];
//     program.reverse();
//     manual_solve(0, &program)
// }

/// Will not work generically, only allows for solving quines that have special expected structure
fn get_hash(memory: &[Instruction]) -> impl Fn(u64) -> u64 {
    let memory = memory.to_owned();
    move |a| {
        let mut reg = [a, 0, 0];
        let mut out = None;
        for i @ &Instruction {
            opcode,
            operand: Operand(operand),
        } in memory[..memory.len() - 1].iter()
        {
            match opcode {
                Opcode::Adv => reg[0] = adv(&reg, operand),
                Opcode::Bxl => reg[1] = reg[1] ^ literal(operand),
                Opcode::Bst => reg[1] = truncate(combo(&reg, operand)),
                Opcode::Jnz => {
                    panic!("Jump should have been excluded");
                }
                Opcode::Bxc => reg[1] ^= reg[2],
                Opcode::Out => out = Some(truncate(combo(&reg, operand))),
                Opcode::Bdv => reg[1] = adv(&reg, operand),
                Opcode::Cdv => reg[2] = adv(&reg, operand),
            }
        }
        out.expect("Out should always have been called in a cycle")
    }
}

fn solve(hash: &impl Fn(u64) -> u64, goal: &[u8], a: u64) -> Option<u64> {
    if let Some(o) = goal.get(0).copied() {
        (0..8)
            .map(|i| a << 3 | i)
            .filter(|&x| o as u64 == hash(x))
            .flat_map(|x| solve(hash, &goal[1..], x))
            .min()
    } else {
        return Some(a);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }
    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some("1,6,3,6,5,6,5,1,7".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("0,3,5,4,3,0".to_owned()));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(247839653009594));
    }

    // #[test]
    // fn test_part_two_manual() {
    //     let result = part_two_manual(&advent_of_code::template::read_file("inputs", DAY));
    //     assert_eq!(result, Some(247839653009594));
    // }

    #[test]
    fn test_random() {
        //
        let mut cpu = Cpu::read(&advent_of_code::template::read_file("inputs", DAY)).unwrap();
        cpu.registers.0[0] = 247839653009594;
        println!("{}", cpu.output());
    }

    #[test]
    fn test_zip() {
        let a = [0, 1, 2, 3, 4, 5];
        let b = [1, 2, 3, 4, 5, 6];
        assert!(!a
            .iter()
            .zip_longest(b.iter())
            .inspect(|x| {
                dbg!(x);
            })
            .all(zip_eq));
        assert!(a
            .iter()
            .map(|x| x + 1)
            .zip_longest(b.iter().copied())
            .inspect(|x| {
                dbg!(x);
            })
            .all(zip_eq));
    }
    #[bench]
    fn bench_part_two_mine(b: &mut Bencher) {
        let input = advent_of_code::template::read_file("inputs", DAY);
        b.iter(|| part_two(&input));
    }
    #[bench]
    fn bench_part_two_hard(b: &mut Bencher) {
        let input = advent_of_code::template::read_file_part("inputs", DAY, "hard");
        b.iter(|| part_two(&input));
    }
}
