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
    grid: Vec<Vec<char>>,
}

impl Input {
    fn count_xmas(&self) -> i128 {
        let mut counted = 0;
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                let c = self.grid[row][col];
                if c == 'X' {
                    counted += self.count_from(row, col, "XMAS");
                }
                if c == 'S' {
                    counted += self.count_from(row, col, "SAMX");
                }
            }
        }
        counted / 2
    }
    fn count_from(&self, row: usize, col: usize, word_to_count: &str) -> i128 {
        // X <-> M <-> A <-> S
        let mut count = 0;
        for dir_idx in 0..9 {
            if self.search_in_direction(row, col, word_to_count, dir_idx) {
                count += 1;
            }
        }
        count
    }
    fn valid_index(&self, row: i128, col: i128) -> bool {
        0 <= row && row < self.grid.len() as i128 && 0 <= col && col < self.grid[0].len() as i128
    }
    fn search_in_direction(
        &self,
        row: usize,
        col: usize,
        word_to_count: &str,
        dir_idx: usize,
    ) -> bool {
        let drow = vec![-1, -1, -1, 0, 0, 0, 1, 1, 1];
        let dcol = vec![-1, 0, 1, -1, 0, 1, -1, 0, 1];
        let mut irow = row as i128;
        let mut icol = col as i128;
        for c in word_to_count.chars() {
            if !self.valid_index(irow, icol) {
                return false;
            }
            if self.grid[irow as usize][icol as usize] != c {
                return false;
            }
            irow += drow[dir_idx];
            icol += dcol[dir_idx];
        }
        true
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    Ok(Input { grid })
}

fn solve_one(input: &Input) -> Result<Answer> {
    Ok(Answer::Num(input.count_xmas()))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day04_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(18));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(2547));
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
