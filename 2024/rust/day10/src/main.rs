#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::Answer;
use std::{
    collections::{HashMap, HashSet, VecDeque},
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

const DROW: [i128; 4] = [-1, 0, 1, 0];
const DCOL: [i128; 4] = [0, 1, 0, -1];

struct Input {
    hiking_area: Vec<Vec<i128>>,
    hiking_starts: Vec<(i128, i128)>,
    hiking_ends: Vec<(i128, i128)>,
}

impl Input {
    fn count_hikes(&self) -> i128 {
        let mut sum = 0;
        for (start_row, start_col) in self.hiking_starts.iter() {
            sum += self.bfs(*start_row, *start_col);
        }
        sum
    }
    fn bfs(&self, row: i128, col: i128) -> i128 {
        let mut encountered = 0;
        let mut visited: HashSet<(i128, i128)> = HashSet::new();
        let mut queue: VecDeque<(i128, i128)> = VecDeque::new();
        queue.push_back((row, col));
        visited.insert((row, col));
        while let Some((row, col)) = queue.pop_front() {
            // println!(
            //     "{}{}: {}",
            //     row, col, self.hiking_area[row as usize][col as usize]
            // );
            if self.hiking_area[row as usize][col as usize] == 9 {
                encountered += 1;
            } else {
                for neigh in self.get_neighs(row, col) {
                    if !visited.contains(&neigh) {
                        queue.push_back(neigh);
                        visited.insert(neigh);
                    }
                }
            }
        }
        encountered
    }
    fn get_neighs(&self, row: i128, col: i128) -> Vec<(i128, i128)> {
        let mut neighs = Vec::new();
        for dir in 0..4 {
            if self.hike_possible(row, col, dir) {
                neighs.push((row + DROW[dir], col + DCOL[dir]));
            }
        }
        neighs
    }
    fn hike_possible(&self, row: i128, col: i128, dir: usize) -> bool {
        let nrow = row + DROW[dir];
        let ncol = col + DCOL[dir];
        if !self.inside(nrow, ncol) {
            return false;
        }
        if self.hiking_area[row as usize][col as usize]
            != self.hiking_area[nrow as usize][ncol as usize] - 1
        {
            return false;
        }
        true
    }
    fn inside(&self, row: i128, col: i128) -> bool {
        0 <= row
            && row < self.hiking_area.len() as i128
            && 0 <= col
            && col < self.hiking_area[0].len() as i128
    }
    fn rate_hiking_trails(&self) -> i128 {
        let mut sum = 0;
        for (start_row, start_col) in self.hiking_starts.iter() {
            for (end_row, end_col) in self.hiking_ends.iter() {
                let mut memo = HashMap::new();
                sum += self.dfs(*start_row, *start_col, *end_row, *end_col, &mut memo);
            }
        }
        sum
    }
    fn dfs(
        &self,
        row: i128,
        col: i128,
        end_row: i128,
        end_col: i128,
        memo: &mut HashMap<(i128, i128), i128>,
    ) -> i128 {
        if let Some(value) = memo.get(&(row, col)) {
            return *value;
        }
        if row == end_row && col == end_col {
            return 1;
        }
        let mut possibilities = 0;
        for (nrow, ncol) in self.get_neighs(row, col) {
            possibilities += self.dfs(nrow, ncol, end_row, end_col, memo);
        }
        memo.insert((row, col), possibilities);
        possibilities
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let mut hiking_area: Vec<Vec<i128>> = Vec::new();
    let mut hiking_starts = Vec::new();
    let mut hiking_ends = Vec::new();
    for (row, line) in input.trim().lines().enumerate() {
        let mut nums = Vec::new();
        for (col, c) in line.trim().chars().enumerate() {
            if c == '0' {
                hiking_starts.push((row as i128, col as i128));
            }
            if c == '9' {
                hiking_ends.push((row as i128, col as i128));
            }
            let num = if c == '.' {
                Ok(100)
            } else {
                c.to_digit(10)
                    .ok_or(anyhow!("only digits are expected in the input"))
            }?;
            nums.push(num as i128);
        }
        hiking_area.push(nums);
    }

    Ok(Input {
        hiking_area,
        hiking_starts,
        hiking_ends,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    Ok(Answer::Num(input.count_hikes()))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(input.rate_hiking_trails()))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day10_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(36));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(629));
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
        assert_eq!(answer, Answer::Num(81));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(1242));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
