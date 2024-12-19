#![feature(test)]
extern crate test;

use anyhow::Result;
use common::Answer;
use std::{collections::HashMap, io};

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
    stones: Vec<i128>,
}

fn parse_input(input: &str) -> Result<Input> {
    let stones = input
        .trim()
        .split_whitespace()
        .map(|s| i128::from_str_radix(s, 10))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Input { stones })
}

fn even_digits(num: i128) -> bool {
    num.to_string().len() % 2 == 0
}
fn split_even_digits(num: i128) -> Result<(i128, i128)> {
    let num_str = num.to_string();
    let half = num_str.len() / 2;
    let a = i128::from_str_radix(&num_str[0..half], 10)?;
    let b = i128::from_str_radix(&num_str[half..], 10)?;
    Ok((a, b))
}

fn blink(stones: &[i128]) -> Result<Vec<i128>> {
    let mut result = Vec::new();
    for &stone in stones {
        if stone == 0 {
            result.push(1);
        } else if even_digits(stone) {
            let (a, b) = split_even_digits(stone)?;
            result.push(a);
            result.push(b);
        } else {
            result.push(stone * 2024);
        }
    }
    Ok(result)
}

struct FastBlinking {
    blink_map: HashMap<(i128, i128), i128>,
}

impl FastBlinking {
    fn new() -> Self {
        Self {
            blink_map: HashMap::new(),
        }
    }
    fn blink(&mut self, stone: i128, blinks: i128) -> Result<i128> {
        if blinks == 0 {
            return Ok(1);
        }
        if let Some(&answer) = self.blink_map.get(&(stone, blinks)) {
            return Ok(answer);
        }
        let answer = if stone == 0 {
            self.blink(1, blinks - 1)?
        } else if even_digits(stone) {
            let (a, b) = split_even_digits(stone)?;
            self.blink(a, blinks - 1)? + self.blink(b, blinks - 1)?
        } else {
            self.blink(stone * 2024, blinks - 1)?
        };
        self.blink_map.insert((stone, blinks), answer);
        return Ok(answer);
    }
}

fn solve_one(input: &Input) -> Result<Answer> {
    let test = i128::from_str_radix("0010", 10)?;
    assert_eq!(test, 10);
    let Input { stones } = input;
    let mut current_stones = stones.clone();
    let mut blinks_left = 25;
    while blinks_left > 0 {
        current_stones = blink(&current_stones)?;
        blinks_left -= 1;
    }
    Ok(Answer::Num(current_stones.len() as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { stones } = input;
    let blinks_left = 75;
    let mut fast_blinking = FastBlinking::new();
    let mut sum = 0;
    for stone in stones {
        sum += fast_blinking.blink(*stone, blinks_left)?;
    }
    Ok(Answer::Num(sum))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day11_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(55312));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(203228));
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
        assert_eq!(answer, Answer::Num(65601038650482));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(240884656550923));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
