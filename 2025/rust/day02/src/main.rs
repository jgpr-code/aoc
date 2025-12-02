#![feature(test)]
extern crate test;

use anyhow::anyhow;
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

struct Input {
    ranges: Vec<(i128, i128)>,
}

fn parse_input(input: &str) -> Result<Input> {
    let input = sanitize_newlines(input);
    let ranges: Vec<(i128, i128)> = input
        .split(',')
        .map(|range| parse_range(range))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Input { ranges })
}

fn sanitize_newlines(input: &str) -> String {
    input.replace(|c| c == '\r' || c == '\n', "")
}

fn parse_range(range: &str) -> Result<(i128, i128)> {
    range
        .split_once('-')
        .ok_or(anyhow!("range without: -"))
        .and_then(|(from, to)| {
            Ok((
                i128::from_str_radix(from, 10)?,
                i128::from_str_radix(to, 10)?,
            ))
        })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { ranges } = input;
    Ok(Answer::Num(
        ranges.iter().map(|range| sum_invalid_ids(*range)).sum(),
    ))
}

fn sum_invalid_ids(range: (i128, i128)) -> i128 {
    (range.0..=range.1)
        .filter(|id| is_repeated_twice(*id))
        .sum()
}

fn is_repeated_twice(number: i128) -> bool {
    let digit_count = number.ilog10() + 1;
    if digit_count % 2 == 1 {
        return false;
    }
    let divisor = 10i128.pow(digit_count / 2);
    let first_part = number / divisor;
    let second_part = number % divisor;
    return first_part == second_part;
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
        assert_eq!(answer, Answer::Num(1227775554));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(44854383294));
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
