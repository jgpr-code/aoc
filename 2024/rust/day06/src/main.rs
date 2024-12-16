#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
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

// UP RIGHT DOWN LEFT
const DROW: [i128; 4] = [-1, 0, 1, 0];
const DCOL: [i128; 4] = [0, 1, 0, -1];

fn next_dir(dir: usize) -> usize {
    (dir + 1) % 4
}

struct Input {
    grid: Vec<Vec<char>>,
    guard_start: (usize, usize),
    row_obstacles: HashMap<usize, Vec<usize>>,
    col_obstacles: HashMap<usize, Vec<usize>>,
}

struct Line {
    start: (usize, usize),
    end: (usize, usize),
    dir: usize,
}

impl Line {
    fn intersect(&self, other: &Line) -> Option<(usize, usize)> {
        if next_dir(self.dir) != other.dir {
            return None;
        }
        // spos, sfrom, sto  opos, oto

        // opos between sfrom and sto
        match self.dir {
            0 => {
                // UP
                if o_end_col < s_start_col {
                    return None;
                }
                if o_end_row < s_end_row && o_end_row >= s_start_row {
                    return Some(s_end_row - 1);
                } else {
                    return None;
                }
            }
            1 => {
                // RIGHT
            }
            2 => {
                // DOWN
            }
            3 => {
                // LEFT
            }
            _ => panic!("must never happen"),
        }

        Some((0, 0))
    }
}

struct WalkingGuard {
    field: Vec<Vec<char>>,
    guard: Guard,
    row_obstacles: HashMap<usize, Vec<usize>>,
    col_obstacles: HashMap<usize, Vec<usize>>,
    loop_possibilities: usize,
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
    fn create(
        field: &Vec<Vec<char>>,
        guard_start: (usize, usize),
        row_obstacles: &HashMap<usize, Vec<usize>>,
        col_obstacles: &HashMap<usize, Vec<usize>>,
    ) -> Self {
        Self {
            field: field.clone(),
            guard: Guard {
                pos: (guard_start.0 as i128, guard_start.1 as i128),
                dir: 0,
            },
            row_obstacles: row_obstacles.clone(),
            col_obstacles: col_obstacles.clone(),
            loop_possibilities: 0,
        }
    }
    // false if guard left the field
    fn walk(&mut self) -> bool {
        let drow = vec![-1, 0, 1, 0];
        let dcol = vec![0, 1, 0, -1];
        // mark field on
        let (row, col) = self.guard.pos;
        self.field[row as usize][col as usize] = 'X';
        if self.loop_possible() {
            self.loop_possibilities += 1;
        }
        // determine walk
        let nrow = row + drow[self.guard.dir];
        let ncol = col + dcol[self.guard.dir];
        if !self.inside(nrow, ncol) {
            return false; // guard left
        }
        if self.field[nrow as usize][ncol as usize] == '#' {
            self.guard.turn_right();
            // unmark field
            self.field[row as usize][col as usize] = '.';
        } else {
            self.guard.pos = (nrow, ncol);
        }
        true
    }
    fn loop_possible(&self) -> bool {
        // 0 is UP
        let next_dir = (self.guard.dir + 1) % 4;
        if next_dir % 2 == 0 {
            // check col_obstacles
        } else {
            // check row_obstacles
        }
        false
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
    let mut row_obstacles: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut col_obstacles: HashMap<usize, Vec<usize>> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        let mut line_chars = Vec::new();
        for (col, c) in line.chars().enumerate() {
            if c == '^' {
                guard_pos = Some((row, col));
            }
            if c == '#' {
                row_obstacles
                    .entry(row)
                    .and_modify(|v| v.push(col))
                    .or_insert(vec![col]);
                col_obstacles
                    .entry(col)
                    .and_modify(|v| v.push(row))
                    .or_insert(vec![row]);
            }
            line_chars.push(c);
        }
        grid.push(line_chars);
    }
    for v in row_obstacles.values_mut() {
        v.sort()
    }
    for v in col_obstacles.values_mut() {
        v.sort();
    }
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let guard_start = guard_pos.ok_or(anyhow!("no guard on field"))?;
    Ok(Input {
        grid,
        guard_start,
        row_obstacles,
        col_obstacles,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input {
        grid,
        guard_start,
        row_obstacles,
        col_obstacles,
    } = input;
    let mut walking_guard = WalkingGuard::create(grid, *guard_start, row_obstacles, col_obstacles);
    walking_guard.loop_walk();

    Ok(Answer::Num(walking_guard.count_walked() as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    // idea is:
    // for current line:
    // check if earlier lines with correct direction intersect and then check if one after this intersection is not visited
    // walk the line

    let Input {
        grid,
        guard_start,
        row_obstacles,
        col_obstacles,
    } = input;

    let rows = grid.len();
    let cols = grid[0].len();
    let mut guard_pos = *guard_start;
    let mut guard_dir = 0;
    // let mut existing_lines = Vec::new();
    loop {
        match guard_dir {
            0 => {
                // UP
                if let Some(col_obstacle) = col_obstacles.get(&guard_pos.1) {
                    let first_below = col_obstacle.partition_point(|&row| row < guard_pos.0);
                }
            }
            1 => {
                // RIGHT
                if let Some(row_obstacle) = row_obstacles.get(&guard_pos.0) {
                    let first_right = row_obstacle.partition_point(|&col| col < guard_pos.1);
                }
            }
            2 => {
                // DOWN
                if let Some(col_obstacle) = col_obstacles.get(&guard_pos.1) {
                    let first_below = col_obstacle.partition_point(|&row| row < guard_pos.0);
                }
            }
            3 => {
                // LEFT
                if let Some(row_obstacle) = row_obstacles.get(&guard_pos.0) {
                    let first_right = row_obstacle.partition_point(|&col| col < guard_pos.1);
                }
            }
        }
    }
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
        assert_eq!(answer, Answer::Num(6));
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
