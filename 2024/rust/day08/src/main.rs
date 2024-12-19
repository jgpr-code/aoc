#![feature(test)]
extern crate test;

use anyhow::Result;
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
    antennas: HashMap<char, Vec<(usize, usize)>>,
    rows: usize,
    cols: usize,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;
    for (row, line) in input.trim().lines().enumerate() {
        rows += 1;
        cols = line.len();
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .and_modify(|v| v.push((row, col)))
                    .or_insert(vec![(row, col)]);
            }
        }
    }
    Ok(Input {
        antennas,
        rows,
        cols,
    })
}

fn inside(row: i32, col: i32, rows: usize, cols: usize) -> bool {
    0 <= row && row < rows as i32 && 0 <= col && col < cols as i32
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input {
        antennas,
        rows,
        cols,
    } = input;
    let mut antinodes = HashSet::new();
    for (_c, v) in antennas.iter() {
        let n = v.len();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let from = (v[i].0 as i32, v[i].1 as i32);
                let to = (v[j].0 as i32, v[j].1 as i32);
                let delta = (to.0 - from.0, to.1 - from.1);
                let target = (from.0 + 2 * delta.0, from.1 + 2 * delta.1);
                if inside(target.0, target.1, *rows, *cols) {
                    antinodes.insert((target.0 as usize, target.1 as usize));
                }
            }
        }
    }
    Ok(Answer::Num(antinodes.len() as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input {
        antennas,
        rows,
        cols,
    } = input;
    let mut antinodes = HashSet::new();
    for (_c, v) in antennas.iter() {
        let n = v.len();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let from = (v[i].0 as i32, v[i].1 as i32);
                let to = (v[j].0 as i32, v[j].1 as i32);
                let delta = (to.0 - from.0, to.1 - from.1);
                let mut current_target = to;
                while inside(current_target.0, current_target.1, *rows, *cols) {
                    antinodes.insert((current_target.0 as usize, current_target.1 as usize));
                    current_target = (current_target.0 + delta.0, current_target.1 + delta.1);
                }
            }
        }
    }
    Ok(Answer::Num(antinodes.len() as i128))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day08_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(14));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(293));
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
        assert_eq!(answer, Answer::Num(34));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(934));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
