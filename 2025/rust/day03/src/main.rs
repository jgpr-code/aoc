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

    /// Computes the maximum joltage using `n` batteries from the bank.
    ///
    /// Joltages are obtained by concatenating the battery digits
    /// and are always read from left to right.
    ///
    /// # Examples
    ///
    /// ```
    /// let bank = BatteryBank::try_from("969594").unwrap();
    /// assert_eq!(bank.max_joltage_n(3).ok(), Some(999));
    /// ```
    fn max_joltage_n(&self, n: usize) -> Result<i128> {
        let batteries = &self.0;
        let n_batteries = batteries.len();
        if n > n_batteries {
            return Err(anyhow!(
                "requested {n} batteries, but only {n_batteries} present"
            ));
        }
        let mut used_batteries: Vec<Option<(usize, u8)>> = vec![None; n];
        for use_idx in 0..used_batteries.len() {
            let start = if use_idx == 0 {
                0
            } else {
                let last_battery_idx = used_batteries[use_idx - 1].unwrap().0;
                last_battery_idx + 1
            };
            for next_idx in start..batteries.len() - n + use_idx + 1 {
                let voltage = batteries[next_idx];
                if used_batteries[use_idx].is_none_or(|(_, v)| v < voltage) {
                    used_batteries[use_idx] = Some((next_idx, voltage));
                }
            }
        }
        Ok(used_batteries.iter().fold(0i128, |acc, b| {
            let (_, voltage) = b.unwrap();
            acc * 10 + voltage as i128
        }))
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
    Ok(Answer::Num(solve(banks, 2)?))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { banks } = input;
    Ok(Answer::Num(solve(banks, 12)?))
}

fn solve(banks: &[BatteryBank], n: usize) -> Result<i128> {
    banks
        .iter()
        .map(|bank| bank.max_joltage_n(n))
        .sum::<Result<i128>>()
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
        mod max_joltage_n {
            use crate::BatteryBank;
            #[test]
            fn prioritizes_high_digit() {
                let bank = BatteryBank::try_from("8291").unwrap();
                assert_eq!(bank.max_joltage_n(2).ok(), Some(91));
            }
            #[test]
            #[should_panic]
            fn more_batteries_than_present_requested() {
                let bank = BatteryBank::try_from("8291").unwrap();
                bank.max_joltage_n(42).unwrap();
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
        assert_eq!(answer, Answer::Num(3121910778619));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(169019504359949));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        b.iter(|| part_two_impl())
    }
}
