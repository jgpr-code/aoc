#![feature(test)]
extern crate test;

use anyhow::Context;
use anyhow::Result;
use common::Answer;
use std::collections::HashMap;
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
    left_nums: Vec<i128>,
    right_nums: Vec<i128>,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut left_nums = Vec::new();
    let mut right_nums = Vec::new();
    for line in input.lines() {
        let (left, right) = parse_line(line)?;
        left_nums.push(left);
        right_nums.push(right);
    }
    Ok(Input {
        left_nums,
        right_nums,
    })
}

fn parse_line(line: &str) -> Result<(i128, i128)> {
    let mut split = line.split("   ");
    let left = i128::from_str_radix(split.next().context("left missing")?, 10)?;
    let right = i128::from_str_radix(split.next().context("right missing")?, 10)?;
    Ok((left, right))
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input {
        left_nums,
        right_nums,
    } = input;

    let mut lnums = left_nums.clone();
    let mut rnums = right_nums.clone();

    lnums.sort();
    rnums.sort();

    let mut sum_distances = 0;
    for (&l, &r) in lnums.iter().zip(rnums.iter()) {
        sum_distances += i128::abs(l - r);
    }

    Ok(Answer::Num(sum_distances))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input {
        left_nums,
        right_nums,
    } = input;
    let mut counts = HashMap::new();
    for right in right_nums.iter() {
        counts
            .entry(right)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut similarity_score = 0;
    for left in left_nums.iter() {
        let count = counts.get(left).unwrap_or(&0);
        similarity_score += left * count;
    }
    Ok(Answer::Num(similarity_score))
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(11));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(2164381));
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
        assert_eq!(answer, Answer::Num(31));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(20719933));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
