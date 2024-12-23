#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::point::point;
use common::point::Point;
use common::Answer;
use std::collections::VecDeque;
use std::i128;
use std::{collections::HashSet, io};

pub fn main() -> Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    println!("part1: {}", part_one(&stdin, 1024, (71, 71))?);
    println!("part2: {}", part_two(&stdin)?);
    Ok(())
}

pub fn part_one(input: &str, falling: usize, grid_size: (i128, i128)) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input, falling, grid_size)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    falling_bytes: Vec<Point>,
}

fn parse_input(input: &str) -> Result<Input> {
    let falling_bytes = input
        .trim()
        .lines()
        .map(|l| parse_point(l))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Input { falling_bytes })
}

fn parse_point(line: &str) -> Result<Point> {
    let (x, y) = line
        .split_once(",")
        .ok_or(anyhow!("not a valid point {}", line))?;
    let x = i128::from_str_radix(x, 10)?;
    let y = i128::from_str_radix(y, 10)?;
    Ok(Point { x, y })
}

struct Grid {
    obstructions: HashSet<Point>,
    grid_size: Point,
}

impl Grid {
    fn new(grid_size: &Point) -> Self {
        Self {
            obstructions: HashSet::new(),
            grid_size: *grid_size,
        }
    }
    fn get_neighbors(&self, at: &Point) -> Vec<Point> {
        at.get_4_neighbors(&self.grid_size)
            .into_iter()
            .filter(|p| !self.obstructions.contains(p))
            .collect()
    }
    fn add_obstruction(&mut self, at: &Point) {
        self.obstructions.insert(*at);
    }
    fn shortest_path(&self, from: &Point, to: &Point) -> i128 {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((*from, 0));
        visited.insert(*from);
        while let Some((p, cost)) = queue.pop_front() {
            if p == *to {
                return cost;
            }
            for neighbor in self.get_neighbors(&p).into_iter() {
                if !visited.contains(&neighbor) {
                    queue.push_back((neighbor, cost + 1));
                    visited.insert(neighbor);
                }
            }
        }
        return i128::MAX;
    }
}

fn solve_one(input: &Input, falling: usize, grid_size: (i128, i128)) -> Result<Answer> {
    let Input { falling_bytes } = input;
    let mut grid = Grid::new(&point!(grid_size));
    for i in 0..falling {
        let byte = falling_bytes
            .get(i)
            .ok_or(anyhow!("no byte left to fall (at {})", i))?;
        grid.add_obstruction(byte);
    }
    Ok(Answer::Num(grid.shortest_path(
        &point!(0, 0),
        &(point!(grid_size) - &point!(1, 1)),
    )))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day18_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST, 12, (7, 7))?;
        assert_eq!(answer, Answer::Num(22));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT, 1024, (71, 71))?;
        assert_eq!(answer, Answer::Num(308));
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
