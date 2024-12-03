#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
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

struct Input {
    haystack: String,
}

fn parse_input(input: &str) -> Result<Input> {
    Ok(Input {
        haystack: String::from(input),
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { haystack } = input;
    let mul_re = regx!(r"mul\((\d{1,3}),(\d{1,3})\)");
    let mut sum = 0;
    for (_, [a, b]) in mul_re.captures_iter(haystack).map(|c| c.extract()) {
        sum += i128::from_str_radix(a, 10)? * i128::from_str_radix(b, 10)?;
    }
    Ok(Answer::Num(sum))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { haystack } = input;
    let mul_or_do_re = regx!(r"mul\((\d{1,3}),(\d{1,3})\)|(do|don't)\(\)");
    let mut do_mul = true;
    let mut sum = 0;
    for caps in mul_or_do_re.captures_iter(haystack) {
        if let Some(a) = caps.get(1) {
            if do_mul {
                let b = caps
                    .get(2)
                    .ok_or(anyhow!("second number missing from mul"))?;
                let astr: &str = a.into();
                let bstr: &str = b.into();
                // println!("{}, {}", astr, bstr);
                sum += i128::from_str_radix(astr, 10)? * i128::from_str_radix(bstr, 10)?;
            }
        } else {
            let what_do = caps
                .get(3)
                .map_or(Err(anyhow!("match should be do or don't")), |c| {
                    Ok(c.as_str())
                })?;
            // println!("{}", what_do);
            if what_do == "do" {
                do_mul = true;
            } else if what_do == "don't" {
                do_mul = false;
            } else {
                return Err(anyhow!("neither do nor don't"));
            }
        }
    }
    Ok(Answer::Num(sum))
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
    static TEST2: LazyLock<String> = local_file!("test2");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(161));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(174336360));
        Ok(())
    }
    #[bench]
    fn part_one(b: &mut Bencher) {
        b.iter(|| part_one_impl())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST2)?;
        assert_eq!(answer, Answer::Num(48));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(88802350));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        b.iter(|| part_two_impl())
    }
}
