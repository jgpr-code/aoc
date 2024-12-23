#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::{regex::Regex, Answer};
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

struct Input {
    vocabular: Vec<String>,
    sentences: Vec<String>,
}

fn parse_input(input: &str) -> Result<Input> {
    let (vocabular, sentences) = input
        .trim()
        .split_once("\n\n")
        .ok_or(anyhow!("there must be two sections"))?;
    let vocabular = vocabular
        .replace(" ", "")
        .split(",")
        .map(|s| String::from(s))
        .collect();
    let sentences = sentences.lines().map(|l| String::from(l)).collect();

    Ok(Input {
        vocabular,
        sentences,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input {
        vocabular,
        sentences,
    } = input;
    let mut vocabular_regex = String::new();
    vocabular_regex.push_str(r"^(");
    vocabular_regex.push_str(vocabular.join("|").as_str());
    vocabular_regex.push_str(")+$");
    let re = Regex::new(&vocabular_regex)?;
    let answer = sentences.iter().filter(|s| re.is_match(s)).count();
    Ok(Answer::Num(answer as i128))
}

struct Matcher {
    vocabular: Vec<String>,
    counted: HashMap<String, usize>,
}
impl Matcher {
    fn with_vocabular(vocabular: &[String]) -> Self {
        Self {
            vocabular: vocabular.iter().cloned().collect(),
            counted: HashMap::new(),
        }
    }
    fn count_possibilities(&mut self, rest: String) -> usize {
        if rest.is_empty() {
            return 1;
        }
        if let Some(&count) = self.counted.get(&rest) {
            return count;
        }
        let mut possibilities = 0;
        let vocabular_len = self.vocabular.len();
        for word_idx in 0..vocabular_len {
            let word = &self.vocabular[word_idx];
            let len = word.len();
            if len > rest.len() {
                continue;
            }
            if word[..] == rest[0..len] {
                possibilities += self.count_possibilities(String::from(&rest[len..]));
            }
        }
        self.counted.insert(rest, possibilities);
        return possibilities;
    }
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input {
        vocabular,
        sentences,
    } = input;
    let mut matcher = Matcher::with_vocabular(vocabular);
    let mut total = 0;
    for sentence in sentences {
        total += matcher.count_possibilities(sentence.clone());
    }
    Ok(Answer::Num(total as i128))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day19_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(6));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(358));
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
        assert_eq!(answer, Answer::Num(16));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(600639829400603));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
