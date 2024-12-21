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
    field: Vec<Vec<char>>,
    // rows: usize,
    // cols: usize,
    instructions: Vec<char>,
    boxes: HashSet<(usize, usize)>,
    robot: (usize, usize),
}

impl Input {
    // input is constructed s.t. this is not necessary (i.e. surrounded by border '#')
    // fn inside(&self, row: i32, col: i32) -> bool {
    //     0 <= row && row < self.rows as i32 && 0 <= col && col < self.cols as i32
    // }
    fn simulate_instructions(&mut self) -> Result<()> {
        let instruction = self.instructions.clone();
        for instruction in instruction.into_iter() {
            self.simulate_instruction(instruction)?;
        }
        Ok(())
    }
    fn simulate_instruction(&mut self, instruction: char) -> Result<()> {
        let (drow, dcol) = match instruction {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => return Err(anyhow!("invalid instruction {}", instruction)),
        };
        let nrow = self.robot.0 as i32 + drow;
        let ncol = self.robot.1 as i32 + dcol;
        let target = self.field[nrow as usize][ncol as usize];
        match target {
            '#' => {}
            '.' => {
                self.field[nrow as usize][ncol as usize] = '@';
                self.field[self.robot.0][self.robot.1] = '.';
                self.robot = (nrow as usize, ncol as usize);
            }
            'O' => {
                let mut mrow = nrow;
                let mut mcol = ncol;
                while self.field[mrow as usize][mcol as usize] == 'O' {
                    mrow += drow;
                    mcol += dcol;
                }
                match self.field[mrow as usize][mcol as usize] {
                    '#' => {}
                    '.' => {
                        self.field[mrow as usize][mcol as usize] = 'O';
                        self.field[nrow as usize][ncol as usize] = '@';
                        self.field[self.robot.0][self.robot.1] = '.';
                        self.robot = (nrow as usize, ncol as usize);
                        self.boxes.remove(&(nrow as usize, ncol as usize));
                        self.boxes.insert((mrow as usize, mcol as usize));
                    }
                    x => return Err(anyhow!("movement to weird field {}", x)),
                }
            }
            _ => return Err(anyhow!("invalid field entry {}", target)),
        }
        Ok(())
    }
    fn gps_coordinate(a_box: &(usize, usize)) -> usize {
        a_box.0 * 100 + a_box.1
    }
    fn gps_sum(&self) -> usize {
        let mut sum = 0;
        for a_box in self.boxes.iter() {
            sum += Self::gps_coordinate(a_box);
        }
        sum
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let (field_str, instructions_str) = input
        .trim()
        .split_once("\n\n")
        .expect("splitting input should work");
    let mut field = Vec::new();
    let mut boxes = HashSet::new();
    let mut robot = None;
    for (row, line) in field_str.lines().enumerate() {
        let mut line_chars = Vec::new();
        for (col, c) in line.chars().enumerate() {
            line_chars.push(c);
            if c == '@' {
                robot = Some((row, col));
            }
            if c == 'O' {
                boxes.insert((row, col));
            }
        }
        field.push(line_chars);
    }
    // let rows = field.len();
    // let cols = field.get(0).expect("at least one line is expected").len();
    let instructions = instructions_str.lines().flat_map(|l| l.chars()).collect();
    Ok(Input {
        field,
        // rows,
        // cols,
        instructions,
        boxes,
        robot: robot.ok_or(anyhow!("robot must be present"))?,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let mut input = input.clone();
    input.simulate_instructions()?;
    Ok(Answer::Num(input.gps_sum() as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
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
