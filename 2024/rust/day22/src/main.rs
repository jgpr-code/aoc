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

struct Input {
    secret_nums: Vec<i128>,
}

fn parse_input(input: &str) -> Result<Input> {
    // example to collect Vec<Result<T, E>> to Result<Vec<T>, E>
    let secret_nums: Vec<i128> = input
        .trim()
        .lines()
        .map(|l| i128::from_str_radix(l, 10))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Input { secret_nums })
}
fn mix_prune(num: &mut i128, mix: i128) {
    const PRUNE: i128 = 16777216;
    *num ^= mix;
    *num %= PRUNE;
}
fn next_secret(secret: i128) -> i128 {
    let mut next_secret = secret;
    let mul64 = next_secret << 6;
    mix_prune(&mut next_secret, mul64);
    let div32 = next_secret >> 5;
    mix_prune(&mut next_secret, div32);
    let mul2024 = next_secret << 11;
    mix_prune(&mut next_secret, mul2024);
    next_secret
}

fn nth_secret(n: usize, secret: &i128) -> i128 {
    let mut nth_secret = *secret;
    for _i in 0..n {
        nth_secret = next_secret(nth_secret);
    }
    nth_secret
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { secret_nums } = input;
    let answer = secret_nums.iter().map(|s| nth_secret(2000, s)).sum();
    Ok(Answer::Num(answer))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day22_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_next_secret() {
        let mut secret = 123;
        let mut actual = Vec::new();
        for _i in 0..10 {
            secret = next_secret(secret);
            actual.push(secret);
        }
        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(37327623));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(16299144133));
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
