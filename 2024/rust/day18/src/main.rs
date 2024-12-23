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
    println!("part2: {}", part_two(&stdin, point!(71, 71))?);
    Ok(())
}

pub fn part_one(input: &str, falling: usize, grid_size: (i128, i128)) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input, falling, grid_size)
}

pub fn part_two(input: &str, grid_size: Point) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input, grid_size)
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
    fn shortest_path(&self, from: &Point, to: &Point) -> Option<i128> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((*from, 0));
        visited.insert(*from);
        while let Some((p, cost)) = queue.pop_front() {
            if p == *to {
                return Some(cost);
            }
            for neighbor in self.get_neighbors(&p).into_iter() {
                if !visited.contains(&neighbor) {
                    queue.push_back((neighbor, cost + 1));
                    visited.insert(neighbor);
                }
            }
        }
        None
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
    let from = point!(0, 0);
    let to = point!(grid_size) - point!(1, 1);
    let shortest_path = grid
        .shortest_path(&from, &to)
        .ok_or(anyhow!("there still must be a path"))?;
    Ok(Answer::Num(shortest_path))
}

fn solve_two(input: &Input, grid_size: Point) -> Result<Answer> {
    let Input { falling_bytes } = input;
    let from = point!(0, 0);
    let to = grid_size - point!(1, 1);
    let mut grid = Grid::new(&grid_size);
    let mut fall_idx = 0;
    while let Some(byte) = falling_bytes.get(fall_idx) {
        grid.add_obstruction(byte);
        if grid.shortest_path(&from, &to).is_none() {
            break;
        }
        fall_idx += 1;
    }
    let last_fallen = falling_bytes
        .get(fall_idx)
        .ok_or(anyhow!("path was never blocked"))?;
    Ok(Answer::Str(format!("{},{}", last_fallen.x, last_fallen.y)))
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
        let answer = super::part_two(&TEST, point!(7, 7))?;
        assert_eq!(answer, Answer::from("6,1"));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT, point!(71, 71))?;
        assert_eq!(answer, Answer::from("46,28"));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
