use super::common::*;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref TEST: String = read_from_file("src/day01/test.txt");
    pub static ref INPUT: String = read_from_file("src/day01/input.txt");
}

struct Elves {
    elf_vec: Vec<Elf>,
}

struct Elf {
    calories: Vec<i128>,
}

impl Elf {
    fn sum_calories(&self) -> i128 {
        self.calories.iter().sum()
    }
}

fn parse_input(buffer: &str) -> Result<Elves> {
    let mut elf_vec: Vec<Elf> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\n|\r\n){2}").unwrap();
    }
    let all_elf_calories = RE.split(buffer).collect::<Vec<_>>();

    for elf_calories in all_elf_calories {
        let calories = elf_calories
            .lines()
            .map(|l| i128::from_str_radix(l, 10))
            .collect::<Result<Vec<_>, _>>()?;
        elf_vec.push(Elf { calories });
    }

    Ok(Elves { elf_vec })
}

pub fn part_one(input: &str) -> Result<Answer> {
    let elves = parse_input(input)?;
    let total_calories = elves
        .elf_vec
        .iter()
        .map(|e| e.sum_calories())
        .collect::<Vec<_>>();

    if let Some(&maximum) = total_calories.iter().max() {
        Ok(Answer::Num(maximum))
    } else {
        Err(anyhow!("no elements for maximum!"))
    }
}

pub fn part_two(input: &str) -> Result<Answer> {
    let elves = parse_input(input)?;
    let mut total_calories: Vec<i128> = elves.elf_vec.iter().map(|e| e.sum_calories()).collect();
    total_calories.sort();
    let top3: i128 = total_calories.iter().rev().take(3).sum();
    Ok(Answer::Num(top3))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::read_from_file;

    // lazy_static! {
    //     static ref TEST: String = read_from_file("test.txt");
    //     static ref INPUT: String = read_from_file("input.txt");
    // }

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(24000));
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, Answer::Num(66306));
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(45000));
        Ok(())
    }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, Answer::Num(195292));
        Ok(())
    }
}
