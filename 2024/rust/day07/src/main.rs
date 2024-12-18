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

struct Equation {
    target: i128,
    nums: Vec<i128>,
}

impl Equation {
    fn solveable(&self) -> bool {
        if self.nums.len() < 1 {
            return false;
        }
        let next = self.nums[0];
        return self.solve(next, Self::skip_one_cloned(&self.nums));
    }
    fn skip_one_cloned(nums: &Vec<i128>) -> Vec<i128> {
        nums.iter().skip(1).cloned().collect()
    }
    fn solve(&self, accu: i128, nums: Vec<i128>) -> bool {
        if nums.is_empty() {
            return accu == self.target;
        }
        let next = nums[0];
        let solve_mul = self.solve(accu * next, Self::skip_one_cloned(&nums));
        let solve_add = self.solve(accu + next, Self::skip_one_cloned(&nums));
        solve_mul || solve_add
    }
}

struct Input {
    equations: Vec<Equation>,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut equations = Vec::new();
    for line in input.trim().lines() {
        let (target, rest) = line.split_once(":").ok_or(anyhow!(": expected"))?;
        let target = i128::from_str_radix(target.trim(), 10)?;
        let nums = rest
            .trim()
            .split(" ")
            .map(|n| i128::from_str_radix(n, 10))
            .collect::<Result<Vec<_>, _>>()?;
        equations.push(Equation { target, nums });
    }
    Ok(Input { equations })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { equations } = input;
    let calibration = equations
        .iter()
        .filter(|e| e.solveable())
        .map(|e| e.target)
        .sum();
    Ok(Answer::Num(calibration))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day07_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(3749));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(6392012777720));
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
