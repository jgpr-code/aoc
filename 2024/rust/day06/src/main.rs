#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::Answer;
use std::{collections::HashSet, io};

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

// UP RIGHT DOWN LEFT
const DROW: [i32; 4] = [-1, 0, 1, 0];
const DCOL: [i32; 4] = [0, 1, 0, -1];

#[derive(Clone)]
struct Input {
    grid: Vec<Vec<char>>,
    guard_start: (usize, usize, usize),
    loops_encountered: HashSet<(i32, i32)>,
}

enum EscapeResult {
    Escape(usize),
    Loop,
}

impl Input {
    #[allow(unused)]
    fn print_grid(&self) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                print!("{}", self.grid[row][col]);
            }
            println!();
        }
        println!();
    }
    fn inside(&self, row: i32, col: i32) -> bool {
        0 <= row && row < self.grid.len() as i32 && 0 <= col && col < self.grid[0].len() as i32
    }
    fn turn(dir: usize) -> usize {
        (dir + 1) % 4
    }
    fn escape_from(
        &mut self,
        pos: (usize, usize, usize),
        mut visited: HashSet<(usize, usize, usize)>,
        additional_obstacle: bool,
    ) -> EscapeResult {
        let (mut row, mut col, mut dir) = pos;
        loop {
            if visited.contains(&(row, col, dir)) {
                return EscapeResult::Loop;
            }
            visited.insert((row, col, dir));
            let nrow = row as i32 + DROW[dir];
            let ncol = col as i32 + DCOL[dir];
            if self.inside(nrow, ncol) {
                if self.grid[nrow as usize][ncol as usize] == '#' {
                    dir = Self::turn(dir);
                } else {
                    if !additional_obstacle && !self.loops_encountered.contains(&(nrow, ncol)) {
                        self.grid[nrow as usize][ncol as usize] = '#';
                        match self.escape_from(pos, HashSet::new(), true) {
                            EscapeResult::Loop => {
                                // self.grid[nrow as usize][ncol as usize] = 'O';
                                // self.print_grid();
                                // self.grid[nrow as usize][ncol as usize] = '.';
                                self.loops_encountered.insert((nrow, ncol));
                            }
                            _ => {}
                        }
                        self.grid[nrow as usize][ncol as usize] = '.';
                    }
                    row = nrow as usize;
                    col = ncol as usize;
                }
            } else {
                let unique_pos: HashSet<(usize, usize)> =
                    visited.iter().map(|&(r, c, _)| (r, c)).collect();
                return EscapeResult::Escape(unique_pos.iter().count());
            }
        }
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let mut guard_pos = None;
    let mut grid = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let mut line_chars = Vec::new();
        for (col, c) in line.chars().enumerate() {
            if c == '^' {
                guard_pos = Some((row, col, 0));
            }
            line_chars.push(c);
        }
        grid.push(line_chars);
    }
    let guard_start = guard_pos.ok_or(anyhow!("no guard on field"))?;
    Ok(Input {
        grid,
        guard_start,
        loops_encountered: HashSet::new(),
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let mut input = input.clone();
    let result = input.escape_from(input.guard_start, HashSet::new(), false);
    let answer = match result {
        EscapeResult::Escape(visited) => visited,
        _ => Err(anyhow!("should be able to escape"))?,
    };
    Ok(Answer::Num(answer as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let mut input = input.clone();
    let result = input.escape_from(input.guard_start, HashSet::new(), false);
    let _answer = match result {
        EscapeResult::Escape(visited) => visited,
        _ => Err(anyhow!("should be able to escape"))?,
    };
    Ok(Answer::Num(input.loops_encountered.iter().count() as i128))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day06_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(41));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(5551));
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
        assert_eq!(answer, Answer::Num(6));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(1939));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
