#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::{regx, Answer};
use std::io;

pub fn main() -> Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    println!("part1: {}", part_one(&stdin, (101, 103))?);
    println!("part2: {}", part_two(&stdin, (101, 103))?);
    Ok(())
}

pub fn part_one(input: &str, on_field: (i32, i32)) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input, on_field)
}

pub fn part_two(input: &str, on_field: (i32, i32)) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input, on_field)
}

#[derive(Clone, Copy)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn modulo(a: i32, modul: i32) -> i32 {
        let mut a = a;
        while a < 0 {
            a += modul;
        }
        while a >= modul {
            a -= modul;
        }
        a
    }
    fn move_me(&mut self, times: i32, on_field: (i32, i32)) {
        self.position = (
            Self::modulo(self.position.0 + times * self.velocity.0, on_field.0),
            Self::modulo(self.position.1 + times * self.velocity.1, on_field.1),
        );
    }

    pub fn safety_factor(robots: &Vec<Robot>, on_field: (i32, i32)) -> usize {
        let middle = (on_field.0 / 2, on_field.1 / 2);
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        for robot in robots {
            let mut x_low = None;
            if robot.position.0 < middle.0 {
                x_low = Some(true);
            } else if robot.position.0 > middle.0 {
                x_low = Some(false);
            }
            let mut y_low = None;
            if robot.position.1 < middle.1 {
                y_low = Some(true);
            } else if robot.position.1 > middle.1 {
                y_low = Some(false);
            }
            match (x_low, y_low) {
                (None, _) => {}
                (_, None) => {}
                (Some(true), Some(true)) => {
                    q1 += 1;
                }
                (Some(false), Some(true)) => {
                    q2 += 1;
                }
                (Some(true), Some(false)) => {
                    q3 += 1;
                }
                (Some(false), Some(false)) => {
                    q4 += 1;
                }
            }
        }
        println!("{} {} {} {}", q1, q2, q3, q4);
        q1 * q2 * q3 * q4
    }
}

impl TryFrom<&str> for Robot {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let robot_regx = regx!(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)");
        let caps = robot_regx
            .captures(value)
            .ok_or(anyhow!("robot regex must match"))?;
        let position = (
            i32::from_str_radix(&caps[1], 10)?,
            i32::from_str_radix(&caps[2], 10)?,
        );
        let velocity = (
            i32::from_str_radix(&caps[3], 10)?,
            i32::from_str_radix(&caps[4], 10)?,
        );
        Ok(Robot { position, velocity })
    }
}

struct Input {
    robots: Vec<Robot>,
}

fn parse_input(input: &str) -> Result<Input> {
    let robots = input
        .trim()
        .lines()
        .map(|l| Robot::try_from(l))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Input { robots })
}

fn solve_one(input: &Input, on_field: (i32, i32)) -> Result<Answer> {
    let Input { robots } = input;
    let mut the_robots = robots.clone();
    the_robots.iter_mut().for_each(|r| r.move_me(100, on_field));
    Ok(Answer::Num(
        Robot::safety_factor(&the_robots, on_field) as i128
    ))
}

fn solve_two(input: &Input, on_field: (i32, i32)) -> Result<Answer> {
    let _unused = input;
    let _unused = on_field;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day14_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST, (11, 7))?;
        assert_eq!(answer, Answer::Num(12));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT, (101, 103))?;
        assert_eq!(answer, Answer::Num(218619324));
        Ok(())
    }
    #[bench]
    fn part_one(b: &mut Bencher) {
        part_one_impl().expect("Error");
        b.iter(|| part_one_impl())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST, (11, 7))?;
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT, (101, 103))?;
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
