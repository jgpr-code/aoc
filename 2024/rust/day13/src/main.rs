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

struct ClawMachine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    price: (usize, usize),
}

impl ClawMachine {
    fn to_signed(p: (usize, usize)) -> (i128, i128) {
        (p.0 as i128, p.1 as i128)
    }
    fn get_the_price(&self) -> Option<usize> {
        // min 3 * na + nb
        // s.t.
        // ax * na + bx * nb = px
        // ay * na + by * nb = py
        // 0 <= na <= 100
        // 0 <= nb <= 100

        // na = (px - bx * nb) / ax
        // ay * ((px - bx * nb) / ax) + by * nb = py
        // (ay*px - ay*bx*nb) / ax + (by * nb * ax) / ax = py
        // ay*px - ay*bx*nb + by*nb*ax = py*ax
        // nb = (ax*py - ay*px) / (ax*by - ay*bx)

        // example: a = (4, 4), b = (2, 2), p = (4, 4)
        // nb = (4 * 4 - 4 * 4) / (4*2 - 4*2) => nb = 0
        // na = (4 - 2 * 0) / 4 => 1

        // nb = (px - ax * na) / bx
        // ay * na + by * ((px - ax * na) / bx) = py
        // ay*na*bx + by*px - by*ax*na = py*bx
        // na = (bx*py - by*px) / (bx*ay - by*ax)

        let (ax, ay) = Self::to_signed(self.button_a);
        let (bx, by) = Self::to_signed(self.button_b);
        let (px, py) = Self::to_signed(self.price);

        let numerator = bx * py - by * px;
        let denominator = bx * ay - by * ax;
        let na;
        let nb;
        if denominator == 0 {
            if numerator == 0 {
                na = 0;
            } else {
                return None;
            }
        } else {
            if numerator % denominator == 0 {
                na = numerator / denominator;
            } else {
                return None;
            }
        }
        let numerator = px - ax * na;
        if numerator % bx == 0 {
            nb = numerator / bx;
        } else {
            return None;
        }

        let cost = 3 * na + nb;
        if cost < 0 {
            return None;
        }

        Some(cost as usize)
    }
}

struct Input {
    all_machines: Vec<ClawMachine>,
}

fn coordinates_from_str(s: &str) -> Result<(usize, usize)> {
    let button_regx = regx!(r"(?:Button [AB]|Prize): X[+=](\d+), Y[+=](\d+)");
    let button_caps = button_regx
        .captures(s)
        .ok_or(anyhow!("button section is invalid: {}", s))?;
    let x = usize::from_str_radix(&button_caps[1], 10)?;
    let y = usize::from_str_radix(&button_caps[2], 10)?;
    Ok((x, y))
}

fn parse_input(input: &str) -> Result<Input> {
    let mut all_machines = Vec::new();
    for section in input.trim().split("\n\n").filter(|s| !s.is_empty()) {
        let section_lines: Vec<&str> = section.lines().collect();
        let button_a = coordinates_from_str(&section_lines[0])?;
        let button_b = coordinates_from_str(&section_lines[1])?;
        let price = coordinates_from_str(&section_lines[2])?;
        all_machines.push(ClawMachine {
            button_a,
            button_b,
            price,
        });
    }
    Ok(Input { all_machines })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { all_machines } = input;
    let mut total_cost = 0;
    for machine in all_machines {
        if let Some(cost) = machine.get_the_price() {
            total_cost += cost;
        }
    }
    Ok(Answer::Num(total_cost as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day13_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(480));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(27105));
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
