#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::Answer;
use std::{
    collections::{HashSet, VecDeque},
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

struct Garden {
    field: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

fn parse_input(input: &str) -> Result<Garden> {
    let field: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
    let rows = field.len();
    let cols = field
        .get(0)
        .ok_or(anyhow!("at least one line is expected"))?
        .len();
    Ok(Garden { field, rows, cols })
}

struct PlantZone {
    plant_type: char,
    area: i128,
    perimeter: i128,
}

impl PlantZone {
    fn new(plant_type: char, area: i128, perimeter: i128) -> Self {
        Self {
            plant_type,
            area,
            perimeter,
        }
    }
    fn price(&self) -> i128 {
        println!("{} * {} = {}", self.area, self.perimeter, self.plant_type);
        self.area * self.perimeter
    }
}

const DROW: [i32; 4] = [-1, 0, 1, 0];
const DCOL: [i32; 4] = [0, 1, 0, -1];
impl Garden {
    fn inside(&self, row: i32, col: i32) -> bool {
        0 <= row && row < self.rows as i32 && 0 <= col && col < self.cols as i32
    }
    fn fence_plants(&self, row: usize, col: usize) -> (PlantZone, HashSet<(usize, usize)>) {
        let plant_type = self.field[row][col];
        let row = row as i32;
        let col = col as i32;
        let mut area = 0;
        let mut perimeter = 0;
        let mut local_visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((row, col));
        local_visited.insert((row as usize, col as usize));
        while let Some((row, col)) = queue.pop_front() {
            area += 1;
            perimeter += 4;
            for dir in 0..4 {
                let nrow = row + DROW[dir];
                let ncol = col + DCOL[dir];
                if self.inside(nrow, ncol) {
                    if self.field[nrow as usize][ncol as usize] == plant_type {
                        perimeter -= 1;
                        if !local_visited.contains(&(nrow as usize, ncol as usize)) {
                            queue.push_back((nrow, ncol));
                            local_visited.insert((nrow as usize, ncol as usize));
                        }
                    }
                }
            }
        }
        (PlantZone::new(plant_type, area, perimeter), local_visited)
    }
}

fn solve_one(input: &Garden) -> Result<Answer> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut plant_zones = Vec::new();
    // visited.union(other)
    for row in 0..input.rows {
        for col in 0..input.cols {
            if !visited.contains(&(row, col)) {
                let (zone, local_visited) = input.fence_plants(row, col);
                local_visited.iter().for_each(|&x| {
                    visited.insert(x);
                });
                plant_zones.push(zone);
            }
        }
    }
    Ok(Answer::Num(plant_zones.iter().map(|z| z.price()).sum()))
}

fn solve_two(input: &Garden) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day12_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(1930));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(1477924));
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
