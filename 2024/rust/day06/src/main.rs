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

struct Input {
    grid: Vec<Vec<char>>,
    guard_start: (usize, usize),
}

struct WalkingGuard {
    field: Vec<Vec<char>>,
    guard: Guard,
}

struct Guard {
    pos: (i128, i128),
    dir: usize,
}

impl Guard {
    fn turn_right(&mut self) {
        self.dir = (self.dir + 1) % 4;
    }
}

impl WalkingGuard {
    fn create(field: &Vec<Vec<char>>, guard_start: (usize, usize)) -> Self {
        Self {
            field: field.clone(),
            guard: Guard {
                pos: (guard_start.0 as i128, guard_start.1 as i128),
                dir: 0,
            },
        }
    }
    // false if guard left the field
    fn walk(&mut self) -> bool {
        let drow = vec![-1, 0, 1, 0];
        let dcol = vec![0, 1, 0, -1];
        // mark field on
        let (row, col) = self.guard.pos;
        self.field[row as usize][col as usize] = 'X';
        // determine walk
        let nrow = row + drow[self.guard.dir];
        let ncol = col + dcol[self.guard.dir];
        if !self.inside(nrow, ncol) {
            return false; // guard left
        }
        if self.field[nrow as usize][ncol as usize] == '#' {
            self.guard.turn_right();
        } else {
            self.guard.pos = (nrow, ncol);
        }
        true
    }
    fn loop_walk(&mut self) {
        while self.walk() {}
    }
    fn inside(&self, row: i128, col: i128) -> bool {
        0 <= row && row < self.field.len() as i128 && 0 <= col && col < self.field[0].len() as i128
    }
    fn count_walked(&self) -> usize {
        self.field
            .iter()
            .map(|v| v.iter().filter(|&c| *c == 'X').count())
            .sum()
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let mut guard_pos = None;
    let mut grid = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let mut line_chars = Vec::new();
        for (col, c) in line.chars().enumerate() {
            if c == '^' {
                guard_pos = Some((row, col));
            }
            line_chars.push(c);
        }
        grid.push(line_chars);
    }
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let guard_start = guard_pos.ok_or(anyhow!("no guard on field"))?;
    Ok(Input { grid, guard_start })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { grid, guard_start } = input;
    let mut walking_guard = WalkingGuard::create(grid, *guard_start);
    walking_guard.loop_walk();

    Ok(Answer::Num(walking_guard.count_walked() as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
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
