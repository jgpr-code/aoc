#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::Answer;
use std::io;

pub fn main() -> Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    println!("part1: {}", part_one(&stdin)?);
    println!("part2: {}", part_two(&stdin)?);
    Ok(())
}

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    register_a: i128,
    register_b: i128,
    register_c: i128,
    program: Vec<u8>,
}

struct Computer {
    register_a: i128,
    register_b: i128,
    register_c: i128,
}
impl From<&Input> for Computer {
    fn from(input: &Input) -> Self {
        Self {
            register_a: input.register_a,
            register_b: input.register_b,
            register_c: input.register_c,
        }
    }
}
impl Computer {
    fn run_program(&mut self, program: &[u8]) -> Result<String> {
        let mut output = Vec::new();
        let mut instruction_pointer = 0;
        while instruction_pointer < program.len() - 1 {
            // -1 because opcode but no operand
            let opcode = program[instruction_pointer];
            let operand = program[instruction_pointer + 1];
            let mut jumped = false;
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => {
                    if let Some(new_instruction_pointer) = self.jnz(operand) {
                        instruction_pointer = new_instruction_pointer;
                        jumped = true;
                    }
                }
                4 => self.bxc(operand),
                5 => output.push(self.out(operand)),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => return Err(anyhow!("invalid opcode")),
            }
            if !jumped {
                instruction_pointer += 2;
            }
        }
        Ok(output.join(","))
    }
    fn combo(&self, operand: u8) -> i128 {
        match operand {
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            x => x as i128,
        }
    }
    fn adv(&mut self, operand: u8) {
        let numerator = self.register_a;
        let denominator = 1i128 << self.combo(operand);
        self.register_a = numerator / denominator;
    }
    fn bxl(&mut self, operand: u8) {
        self.register_b = self.register_b ^ operand as i128;
    }
    fn bst(&mut self, operand: u8) {
        self.register_b = self.combo(operand) % 8;
    }
    fn jnz(&mut self, operand: u8) -> Option<usize> {
        if self.register_a == 0 {
            return None;
        }
        Some(operand as usize)
    }
    fn bxc(&mut self, _operand: u8) {
        self.register_b = self.register_b ^ self.register_c;
    }
    fn out(&mut self, operand: u8) -> String {
        (self.combo(operand) % 8).to_string()
    }
    fn bdv(&mut self, operand: u8) {
        let numerator = self.register_a;
        let denominator = 1i128 << self.combo(operand);
        self.register_b = numerator / denominator;
    }
    fn cdv(&mut self, operand: u8) {
        let numerator = self.register_a;
        let denominator = 1i128 << self.combo(operand);
        self.register_c = numerator / denominator;
    }
}

fn parse_register(register: &str) -> Result<i128> {
    Ok(i128::from_str_radix(
        register
            .split_once(": ")
            .ok_or(anyhow!("register must be present"))?
            .1,
        10,
    )?)
}

fn parse_input(input: &str) -> Result<Input> {
    let (registers, program) = input
        .trim()
        .split_once("\n\n")
        .ok_or(anyhow!("there should be an empty line"))?;
    let registers: Vec<&str> = registers.lines().collect();
    let register_a = parse_register(registers[0])?;
    let register_b = parse_register(registers[1])?;
    let register_c = parse_register(registers[2])?;
    let program = program
        .split_once(": ")
        .ok_or(anyhow!("program must be present"))?
        .1
        .split(",")
        .map(|n| u8::from_str_radix(n, 10))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Input {
        register_a,
        register_b,
        register_c,
        program,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let mut computer = Computer::from(input);
    let program_output = computer.run_program(&input.program)?;
    Ok(Answer::Str(program_output))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day17_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Str(String::from("4,6,3,5,6,3,5,2,1,0")));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Str(String::from("4,1,5,3,1,5,3,5,7")));
        Ok(())
    }
    #[bench]
    fn part_one(b: &mut Bencher) {
        part_one_impl().expect("Error");
        b.iter(|| part_one_impl())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
