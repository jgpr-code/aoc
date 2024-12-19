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
    sides: i128,
}

impl PlantZone {
    fn new(plant_type: char, area: i128, perimeter: i128, sides: i128) -> Self {
        Self {
            plant_type,
            area,
            perimeter,
            sides,
        }
    }
    fn price(&self) -> i128 {
        println!("{} * {} = {}", self.area, self.perimeter, self.plant_type);
        self.area * self.perimeter
    }
    fn new_price(&self) -> i128 {
        self.area * self.sides
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
        let mut sides = 0;
        let mut local_visited = HashSet::new();
        let mut sides_counted: HashMap<(usize, usize), Vec<bool>> = HashMap::new(); // (row, col) -> [up, right, down, left] counted booleans
        let mut queue = VecDeque::new();
        queue.push_back((row, col));
        local_visited.insert((row as usize, col as usize));
        while let Some((row, col)) = queue.pop_front() {
            area += 1;
            perimeter += 4;
            let mut transitions = vec![true, true, true, true];
            for dir in 0..4 {
                let nrow = row + DROW[dir];
                let ncol = col + DCOL[dir];
                if self.inside(nrow, ncol) {
                    if self.field[nrow as usize][ncol as usize] == plant_type {
                        perimeter -= 1;
                        transitions[dir] = false;
                        if !local_visited.contains(&(nrow as usize, ncol as usize)) {
                            queue.push_back((nrow, ncol));
                            local_visited.insert((nrow as usize, ncol as usize));
                        }
                    }
                }
            }
            for dir in 0..4 {
                if transitions[dir] {
                    let n1 = (dir + 1) % 4;
                    let n1_counted = self.side_counted(dir, n1, row, col, &sides_counted);
                    let n2 = (dir + 3) % 4;
                    let n2_counted = self.side_counted(dir, n2, row, col, &sides_counted);
                    if !n1_counted && !n2_counted {
                        sides += 1;
                    }
                    let e = sides_counted
                        .entry((row as usize, col as usize))
                        .or_insert(vec![false, false, false, false]);
                    e[dir] = true;
                }
            }
        }
        (
            PlantZone::new(plant_type, area, perimeter, sides),
            local_visited,
        )
    }
    fn side_counted(
        &self,
        dir: usize,
        ndir: usize,
        row: i32,
        col: i32,
        sides_counted: &HashMap<(usize, usize), Vec<bool>>,
    ) -> bool {
        let nrow = row + DROW[ndir];
        let ncol = col + DCOL[ndir];
        let mut counted = false;
        if self.inside(nrow, ncol) {
            if let Some(v) = sides_counted.get(&(nrow as usize, ncol as usize)) {
                counted = v[dir];
            }
        }
        counted
    }
}

fn get_plant_zones(garden: &Garden) -> Vec<PlantZone> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut plant_zones = Vec::new();
    for row in 0..garden.rows {
        for col in 0..garden.cols {
            if !visited.contains(&(row, col)) {
                let (zone, local_visited) = garden.fence_plants(row, col);
                local_visited.iter().for_each(|&x| {
                    visited.insert(x);
                });
                plant_zones.push(zone);
            }
        }
    }
    plant_zones
}

fn solve_one(input: &Garden) -> Result<Answer> {
    let plant_zones = get_plant_zones(input);
    Ok(Answer::Num(plant_zones.iter().map(|z| z.price()).sum()))
}

fn solve_two(input: &Garden) -> Result<Answer> {
    let plant_zones = get_plant_zones(input);
    Ok(Answer::Num(plant_zones.iter().map(|z| z.new_price()).sum()))
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
        assert_eq!(answer, Answer::Num(1206));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(841934));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
