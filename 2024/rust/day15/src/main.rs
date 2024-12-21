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

#[derive(Clone)]
struct Input {
    robot: (i32, i32),
    boxes: HashSet<(i32, i32)>,
    obstructions: HashSet<(i32, i32)>,
    instructions: Vec<char>,
    is_enlarged: bool,
}

impl Input {
    fn simulate_instructions(&mut self) -> Result<()> {
        let instruction = self.instructions.clone();
        if self.is_enlarged {
            for instruction in instruction.into_iter() {
                self.simulate_instruction_enlarged(instruction)?;
            }
        } else {
            for instruction in instruction.into_iter() {
                self.simulate_instruction(instruction)?;
            }
        }
        Ok(())
    }
    fn move_box(&self, a_box: (i32, i32), dir: (i32, i32)) -> Option<Vec<(i32, i32)>> {
        let move_to = (a_box.0 + dir.0, a_box.1 + dir.1);
        if self.obstructions.contains(&move_to) {
            return None;
        }
        if self.boxes.contains(&move_to) {
            if let Some(mut moved_boxes) = self.move_box(move_to, dir) {
                moved_boxes.push(a_box);
                return Some(moved_boxes);
            } else {
                return None;
            }
        }
        Some(vec![a_box])
    }
    fn move_box_enlarged(&self, a_box: (i32, i32), dir: (i32, i32)) -> Option<Vec<(i32, i32)>> {
        let move_to = (a_box.0 + dir.0, a_box.1 + dir.1);
        let move_to_right_side = (move_to.0, move_to.1 + 1);
        if self.obstructions.contains(&move_to) || self.obstructions.contains(&move_to_right_side) {
            return None;
        }
        let mut result = vec![a_box];
        let left_box = self.contains_box(move_to);
        if let Some(left_box) = left_box {
            if left_box != a_box {
                if let Some(moved_boxes) = self.move_box_enlarged(left_box, dir) {
                    result.extend(moved_boxes);
                } else {
                    return None;
                }
            }
        }
        if let Some(right_box) = self.contains_box(move_to_right_side) {
            if right_box != a_box && left_box.map_or(true, |left_box| left_box != right_box) {
                if let Some(moved_boxes) = self.move_box_enlarged(right_box, dir) {
                    result.extend(moved_boxes);
                } else {
                    return None;
                }
            }
        }
        return Some(result);
    }
    fn get_dir(instruction: char) -> Result<(i32, i32)> {
        Ok(match instruction {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => return Err(anyhow!("invalid instruction {}", instruction)),
        })
    }
    fn simulate_instruction(&mut self, instruction: char) -> Result<()> {
        let dir = Self::get_dir(instruction)?;
        let move_to = (self.robot.0 + dir.0, self.robot.1 + dir.1);
        if self.obstructions.contains(&move_to) {
            return Ok(());
        }
        if self.boxes.contains(&move_to) {
            if let Some(moved_boxes) = self.move_box(move_to, dir) {
                self.robot = move_to;
                for moved_box in moved_boxes.iter() {
                    self.boxes.remove(&moved_box);
                }
                for moved_box in moved_boxes.iter() {
                    let moved_box = (moved_box.0 + dir.0, moved_box.1 + dir.1);
                    self.boxes.insert(moved_box);
                }
            }
        } else {
            self.robot = move_to;
        }
        Ok(())
    }
    fn contains_box(&self, target: (i32, i32)) -> Option<(i32, i32)> {
        let left_of_target = (target.0, target.1 - 1);
        if self.boxes.contains(&target) {
            return Some(target);
        } else if self.boxes.contains(&left_of_target) {
            return Some(left_of_target);
        }
        None
    }
    fn simulate_instruction_enlarged(&mut self, instruction: char) -> Result<()> {
        let dir = Self::get_dir(instruction)?;
        let move_to = (self.robot.0 + dir.0, self.robot.1 + dir.1);
        if self.obstructions.contains(&move_to) {
            return Ok(());
        }
        if let Some(box_at_target) = self.contains_box(move_to) {
            if let Some(moved_boxes) = self.move_box_enlarged(box_at_target, dir) {
                self.robot = move_to;
                for moved_box in moved_boxes.iter() {
                    self.boxes.remove(&moved_box);
                }
                for moved_box in moved_boxes.iter() {
                    let moved_box = (moved_box.0 + dir.0, moved_box.1 + dir.1);
                    self.boxes.insert(moved_box);
                }
            }
        } else {
            self.robot = move_to;
        }
        Ok(())
    }
    fn gps_coordinate(a_box: &(i32, i32)) -> i128 {
        assert!(a_box.0 > 0 && a_box.1 > 0);
        (a_box.0 * 100 + a_box.1) as i128
    }
    fn gps_sum(&self) -> i128 {
        let mut sum = 0;
        for a_box in self.boxes.iter() {
            sum += Self::gps_coordinate(a_box);
        }
        sum
    }
    fn enlarge(&mut self) {
        // row aka 0 stays the same
        self.robot = (self.robot.0, self.robot.1 * 2);
        let mut new_obstructions = HashSet::new();
        for old_obstruction in self.obstructions.iter() {
            new_obstructions.insert((old_obstruction.0, old_obstruction.1 * 2));
            new_obstructions.insert((old_obstruction.0, old_obstruction.1 * 2 + 1));
        }
        self.obstructions = new_obstructions;
        let mut new_boxes = HashSet::new();
        for old_box in self.boxes.iter() {
            new_boxes.insert((old_box.0, old_box.1 * 2));
        }
        self.boxes = new_boxes;
        self.is_enlarged = true;
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let (field_str, instructions_str) = input
        .trim()
        .split_once("\n\n")
        .expect("splitting input should work");
    let mut boxes = HashSet::new();
    let mut obstructions = HashSet::new();
    let mut robot = None;
    for (row, line) in field_str.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let (row, col) = (row as i32, col as i32);
            if c == '@' {
                robot = Some((row, col));
            }
            if c == 'O' {
                boxes.insert((row, col));
            }
            if c == '#' {
                obstructions.insert((row, col));
            }
        }
    }
    let instructions = instructions_str.lines().flat_map(|l| l.chars()).collect();
    Ok(Input {
        robot: robot.ok_or(anyhow!("robot must be present"))?,
        boxes,
        obstructions,
        instructions,
        is_enlarged: false,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let mut input = input.clone();
    input.simulate_instructions()?;
    Ok(Answer::Num(input.gps_sum()))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let mut input = input.clone();
    input.enlarge();
    input.simulate_instructions()?;
    Ok(Answer::Num(input.gps_sum()))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day15_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(10092));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(1371036));
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
        assert_eq!(answer, Answer::Num(9021));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(1392847));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
