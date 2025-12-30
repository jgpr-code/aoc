//! This is a crate to solve Advent of Code 2025 day03

#![warn(missing_docs)]
#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::Answer;
use std::io;

/// Main function which produces the outputs of both parts for given input from [`std::io::stdin()`].
///
/// Calls [`part_one`] and [`part_two`] to obtain the respective answers.
pub fn main() -> Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    println!("part1: {}", part_one(&stdin)?);
    println!("part2: {}", part_two(&stdin)?);
    Ok(())
}

/// Computes the Answer for part one
pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

/// Computes the Answer for part two
pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

/// Thin wrapper to represent a collection of batteries.
///
/// Each battery is a single digit between 0-9 representing its
/// storage capabilities.
///
/// Note: A battery bank always has at least two batteries!
pub struct BatteryBank(Vec<u8>);

impl TryFrom<&str> for BatteryBank {
    type Error = anyhow::Error;

    fn try_from(bank_line: &str) -> std::result::Result<Self, Self::Error> {
        if bank_line.len() < 2 {
            return Err(anyhow!("BatteryBank must have at least 2 batteries"));
        }
        bank_line
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .ok_or(anyhow!("Invalid digit: {}", c))
                    .map(|d| d as u8)
            })
            .collect::<Result<Vec<u8>>>()
            .map(BatteryBank)
    }
}

impl BatteryBank {
    // NOTE to myself: doctests don't work with binary crates (`cargo test --doc` expects library crate)

    /// Computes the maximum joltage using two batteries from the bank.
    ///
    /// Joltages are obtained by concatenating the battery digits
    /// and are always read from left to right.
    ///
    /// # Examples
    ///
    /// ```
    /// let bank = BatteryBank::try_from("969594").unwrap();
    /// assert_eq!(bank.max_joltage(), 96);
    /// ```
    fn max_joltage(&self) -> i128 {
        let batteries = &self.0;
        let mut front: Option<(usize, u8)> = None;
        for i in 0..batteries.len() - 1 {
            let voltage = batteries[i];
            if front.is_none_or(|(_, v)| v < voltage) {
                front = Some((i, voltage));
            }
        }
        let (front_idx, front_value) = front.expect("front should always be Some( )");
        let mut back: Option<(usize, u8)> = None;
        for i in front_idx + 1..batteries.len() {
            let voltage = batteries[i];
            if back.is_none_or(|(_, v)| v < voltage) {
                back = Some((i, voltage))
            }
        }
        let (_, back_value) = back.expect("back should always be Some( )");
        i128::from_str_radix(&format!("{front_value}{back_value}"), 10)
            .expect("from_str_radix should always succeed here")
    }
}

struct Input {
    banks: Vec<BatteryBank>,
}

fn parse_input(input: &str) -> Result<Input> {
    let banks = input
        .lines()
        .map(|line| BatteryBank::try_from(line))
        .collect::<Result<_>>()?;
    Ok(Input { banks })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { banks } = input;
    Ok(Answer::Num(
        banks.iter().map(|bank| bank.max_joltage()).sum(),
    ))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day03_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    mod battery_bank {
        mod max_joltage {
            use crate::BatteryBank;
            #[test]
            fn prioritize_high_digit() {
                let bank = BatteryBank::try_from("8291").unwrap();
                assert_eq!(bank.max_joltage(), 91);
            }
        }
    }

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(357));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(17087));
        Ok(())
    }
    #[bench]
    fn part_one(b: &mut Bencher) {
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
        b.iter(|| part_two_impl())
    }
}
