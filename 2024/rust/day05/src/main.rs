#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::Answer;
use std::{
    collections::{HashMap, HashSet},
    io,
};

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
    page_rules: HashMap<i128, HashSet<i128>>,
    page_updates: Vec<Vec<i128>>,
}

impl Input {
    fn is_valid_update(&self, update: &[i128]) -> Result<bool> {
        // for current page check if after_hashset intersects occured_hashset
        let mut occured_pages = HashSet::new();
        println!("CHECKING {:?}", update);
        for value in update {
            let empty: HashSet<i128> = HashSet::new();
            let after_set = self.page_rules.get(value).unwrap_or_else(|| &empty);
            println!(
                "afterset {:?}, occured_pages {:?}, value {}",
                after_set, occured_pages, value
            );
            let intersection = occured_pages.intersection(after_set);
            if intersection.count() != 0 {
                return Ok(false);
            }
            occured_pages.insert(*value);
        }
        Ok(true)
    }
    fn get_update_value(update: &[i128]) -> i128 {
        update[update.len() / 2]
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let (rules, updates) = input
        .split_once("\n\n")
        .ok_or(anyhow!("just one blank line expected"))?;
    let mut page_rules: HashMap<i128, HashSet<i128>> = HashMap::new();
    for line in rules.lines() {
        let (a, b) = line.split_once("|").ok_or(anyhow!("rule requires |"))?;
        let before = i128::from_str_radix(a, 10)?;
        let after = i128::from_str_radix(b, 10)?;
        page_rules
            .entry(before)
            .and_modify(|s| {
                s.insert(after);
            })
            .or_insert(HashSet::from([after]));
    }
    let page_updates = updates
        .lines()
        .map(|l| {
            l.split(",")
                .map(|s| i128::from_str_radix(s, 10))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()?;
    Ok(Input {
        page_rules,
        page_updates,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    // iterate through updates
    // keep occured pages in HashSet
    // check if page

    let Input {
        page_rules: _,
        page_updates,
    } = input;

    let mut sum = 0;
    for update in page_updates {
        if input.is_valid_update(update)? {
            println!("{:?}", update);
            sum += Input::get_update_value(update);
        }
    }

    Ok(Answer::Num(sum))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day05_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(143));
        Ok(())
    }
    #[test]
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(6041));
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
    #[test]
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
