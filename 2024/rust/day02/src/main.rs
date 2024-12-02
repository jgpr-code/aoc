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

impl From<&str> for Report {
    fn from(line: &str) -> Self {
        let mut last_n: Option<i128> = None;
        let mut levels = Vec::new();
        let mut deltas = Vec::new();
        for nstr in line.split(" ") {
            let n = i128::from_str_radix(nstr, 10).unwrap(); // ugly unwrap!
            if let Some(ln) = last_n {
                deltas.push(n - ln);
            }
            last_n = Some(n);
            levels.push(n);
        }
        Self { levels, deltas }
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        print!("{:?}", self.levels);
        let verdict = self.deltas.iter().all(|&n| {
            let n_abs = i128::abs(n);
            1 <= n_abs && n_abs <= 3
        }) && (self.deltas.iter().all(|&n| i128::signum(n) == 1)
            || self.deltas.iter().all(|&n| i128::signum(n) == -1));
        println!(" -> {}", verdict);
        verdict
    }
}

struct Input {
    reports: Vec<Report>,
}

fn parse_input(input: &str) -> Result<Input> {
    let reports: Vec<_> = input.lines().map(|l| Report::from(l)).collect();
    Ok(Input { reports })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { reports } = input;
    Ok(Answer::Num(
        reports.iter().filter(|r| r.is_safe()).count().try_into()?,
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
