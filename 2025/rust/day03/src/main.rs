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
        let (max_voltage, _) = (0..n).fold((0i128, 0usize), |(acc, start), nth_battery| {
            // n_batteries: just this would mean all batteries
            // -n+1: because let's say we have n_batteries == 3 and n == 2, then the first battery can't be at index 2,
            //       because there wouldn't be any space left for the second battery
            // +use_idx: because we can search farther for each subsequent battery
            let end = n_batteries - n + nth_battery + 1;
            let (index, value) = Self::first_max_with_index(batteries, start, end);
            (acc * 10 + value as i128, index + 1)
        });
        Ok(max_voltage)
    }

    fn first_max_with_index(batteries: &[u8], start: usize, end: usize) -> (usize, u8) {
        batteries[start..end]
            .iter()
            .enumerate()
            .map(|(idx, &v)| (idx + start, v))
            .max_by(|(_, last_max), (_, value)| {
                last_max.cmp(value).then(std::cmp::Ordering::Greater)
            })
            .unwrap()
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
    fn test_max_by_behavior() {
        let vec = vec![5; 3];
        let max = vec
            .iter()
            .enumerate()
            // .max() returns last 5, but we want first 5
            .max_by(|(_, v1), (_, v2)| v1.cmp(v2).then(std::cmp::Ordering::Greater))
            .unwrap();
        assert_eq!(max, (0usize, &5i32));
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
