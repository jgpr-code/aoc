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

struct Key([u8; 5]);
struct Lock([u8; 5]);

enum Parsed {
    Key(Key),
    Lock(Lock),
    Garbage(String),
}

fn parse_block(block: &str) -> Result<Parsed> {
    let lines: Vec<&str> = block.lines().collect();
    if lines.len() != 7 {
        return Ok(Parsed::Garbage(String::from("not enough lines")));
    }
    return match count_hashes(&lines) {
        Err(err) => Ok(Parsed::Garbage(err.to_string())),
        Ok(counts) => {
            if lines[0] == "#####" {
                Ok(Parsed::Lock(Lock(counts)))
            } else if lines[6] == "#####" {
                Ok(Parsed::Key(Key(counts)))
            } else {
                Ok(Parsed::Garbage(String::from("neither key nor lock")))
            }
        }
    };
}

fn count_hashes(lines: &[&str]) -> Result<[u8; 5]> {
    let mut counts = [0, 0, 0, 0, 0];
    for line in lines.iter() {
        for (i, c) in line.chars().enumerate() {
            if i > 4 {
                return Err(anyhow!("line is too long"));
            }
            if c == '#' {
                counts[i] += 1;
            }
        }
    }
    Ok(counts)
}

impl Lock {
    fn key_fits(&self, key: &Key) -> bool {
        for i in 0..5 {
            if key.0[i] + self.0[i] > 7 {
                return false;
            }
        }
        true
    }
}

struct Input {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for block in input.trim().split("\n\n") {
        match parse_block(block)? {
            Parsed::Garbage(err) => Err(anyhow!(err))?,
            Parsed::Key(key) => keys.push(key),
            Parsed::Lock(lock) => locks.push(lock),
        }
    }
    Ok(Input { keys, locks })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { keys, locks } = input;
    let answer: usize = locks
        .iter()
        .map(|lock| keys.iter().filter(|key| lock.key_fits(key)).count())
        .sum();
    Ok(Answer::Num(answer as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day25_tests {
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
        assert_eq!(answer, Answer::Num(2854));
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
