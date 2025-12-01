#![feature(test)]
extern crate test;

use anyhow::anyhow;
use anyhow::Result;
use common::regx;
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

enum Rotation {
    Left(u128),
    Right(u128),
}

impl TryFrom<&str> for Rotation {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let re = regx!(r"([LR])(\d+)");
        let caps = re.captures(value).ok_or(anyhow!("regex did not match"))?;
        // unwraps and unreachable are fine because otherwise the regex wouldn't match
        let rotation_type = caps.get(1).unwrap().as_str();
        let amount = u128::from_str_radix(caps.get(2).unwrap().as_str(), 10)?;
        Ok(match rotation_type {
            "L" => Rotation::Left(amount),
            "R" => Rotation::Right(amount),
            _ => unreachable!(),
        })
    }
}

struct Input {
    rotations: Vec<Rotation>,
}

fn parse_input(input: &str) -> Result<Input> {
    // example to collect Vec<Result<T, E>> to Result<Vec<T>, E>
    let rotations: Vec<Rotation> = input
        .lines()
        .map(|l| Rotation::try_from(l))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Input { rotations })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let dial_size = 100;
    let Input { rotations } = input;
    let mut zero_counter = 0;
    let mut dial_position = 50;
    for rotation in rotations.iter() {
        dial_position = match rotation {
            Rotation::Left(amount) => {
                let amount = amount % dial_size;
                let mut new_dial_position = dial_position - amount as i128;
                while new_dial_position < 0 {
                    new_dial_position += dial_size as i128;
                }
                new_dial_position
            }
            Rotation::Right(amount) => {
                let amount = amount % dial_size;
                (dial_position + amount as i128) % 100
            }
        };
        if dial_position == 0 {
            zero_counter += 1;
        }
    }
    Ok(Answer::Num(zero_counter))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day01_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(3));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(984));
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
