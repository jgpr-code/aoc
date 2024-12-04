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
    fn count_x_mas(&self) -> i128 {
        let mut count = 0;
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                if self.is_x_mas(row, col) {
                    count += 1;
                }
            }
        }
        count
    }
    fn is_x_mas(&self, row: usize, col: usize) -> bool {
        // M.S  M.M  S.M  S.S
        // .A.  .A.  .A.  .A.
        // M.S  S.S  S.M  M.M
        let x_mas = vec!["M.S.A.M.S", "M.M.A.S.S", "S.M.A.S.M", "S.S.A.M.M"];
        let x_mas: Vec<Vec<char>> = x_mas
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        for xmas in x_mas.iter() {
            if self.check_xmas(row, col, xmas) {
                return true;
            }
        }
        false
    }
    fn check_xmas(&self, row: usize, col: usize, xmas: &[char]) -> bool {
        for xmas_row in 0..3 {
            for xmas_col in 0..3 {
                let check_row = (row + xmas_row) as i128;
                let check_col = (col + xmas_col) as i128;
                if !self.valid_index(check_row, check_col) {
                    return false;
                }
                let xmas_idx = (xmas_row * 3 + xmas_col) as usize;
                if xmas[xmas_idx] == '.' {
                    continue;
                }
                if xmas[xmas_idx] != self.grid[check_row as usize][check_col as usize] {
                    return false;
                }
            }
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
    Ok(Answer::Num(input.count_x_mas()))
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
        assert_eq!(answer, Answer::Num(9));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(1939));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        b.iter(|| part_two_impl())
    }
}
