#![feature(test)]
extern crate test;

use anyhow::Result;
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

struct Report {
    levels: Vec<i128>,
    deltas: Vec<i128>,
}

impl TryFrom<&str> for Report {
    type Error = core::num::ParseIntError;

    fn try_from(line: &str) -> std::result::Result<Self, Self::Error> {
        let mut last_n: Option<i128> = None;
        let mut levels = Vec::new();
        let mut deltas = Vec::new();
        for nstr in line.split(" ") {
            let n = i128::from_str_radix(nstr, 10)?;
            if let Some(ln) = last_n {
                deltas.push(n - ln);
            }
            last_n = Some(n);
            levels.push(n);
        }
        Ok(Self { levels, deltas })
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        // print!("{:?}", self.levels);
        let verdict = self.deltas.iter().all(|&n| {
            let n_abs = i128::abs(n);
            1 <= n_abs && n_abs <= 3
        }) && (self.deltas.iter().all(|&n| i128::signum(n) == 1)
            || self.deltas.iter().all(|&n| i128::signum(n) == -1));
        // println!(" -> {}", verdict);
        verdict
    }
    fn is_dampened_safe(&self) -> bool {
        for skip_idx in 0..self.levels.len() {
            if self.check_skipping(skip_idx) {
                // println!("{:?} skip:{}", self.levels, skip_idx);
                return true;
            }
        }
        false
    }
    fn check_skipping(&self, skip_idx: usize) -> bool {
        let mut signum = 0;
        let mut last_n: Option<i128> = None;
        for (i, &n) in self.levels.iter().enumerate() {
            if i == skip_idx {
                continue;
            }
            if let Some(ln) = last_n {
                let delta = n - ln;
                let abs_delta = i128::abs(delta);
                if !(1 <= abs_delta && abs_delta <= 3) {
                    return false;
                }
                let delta_signum = i128::signum(delta);
                if delta_signum == 0 {
                    return false;
                }
                if signum == 0 {
                    signum = delta_signum;
                } else if signum != delta_signum {
                    return false;
                }
            }
            last_n = Some(n);
        }
        true
    }
}

struct Input {
    reports: Vec<Report>,
}

fn parse_input(input: &str) -> Result<Input> {
    let reports: Vec<_> = input
        .lines()
        .map(|l| Report::try_from(l))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Input { reports })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { reports } = input;
    Ok(Answer::Num(
        reports.iter().filter(|r| r.is_safe()).count().try_into()?,
    ))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { reports } = input;
    let mut count = 0;
    for report in reports {
        if report.is_safe() {
            count += 1;
            continue;
        }
        if report.is_dampened_safe() {
            count += 1;
        }
    }

    Ok(Answer::Num(count))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day02_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(2));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(299));
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
        assert_eq!(answer, Answer::Num(4));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(364));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
