use super::common::*;
use anyhow::Result;
use regex::Regex;
use std::num::ParseIntError;
use std::sync::OnceLock;

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

#[derive(Debug)]
struct Input {
    lines: Vec<String>,
    numbers: Vec<Vec<Number>>,
}

#[derive(Debug)]
struct Number {
    val: i128,
    start: usize,
    end: usize,
}

impl Input {
    fn sum_adjacent(&self) -> i128 {
        let grid = self
            .lines
            .iter()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<_>>>();
        let dx = vec![-1, 0, 1, -1, 1, -1, 0, 1];
        let dy = vec![-1, -1, -1, 0, 0, 1, 1, 1];

        let mut sum = 0;
        for (y, numbers) in self.numbers.iter().enumerate() {
            for num in numbers {
                'number_loop: for x in num.start..num.end {
                    for i in 0..8 {
                        let cx = x as i128 + dx[i];
                        let cy = y as i128 + dy[i];
                        if cx < 0
                            || cy < 0
                            || cx >= grid[0].len() as i128
                            || cy >= grid.len() as i128
                        {
                            continue;
                        }
                        let c = grid[cy as usize][cx as usize];
                        if c != '.' && !c.is_ascii_digit() {
                            println!("{:?}", num.val);
                            sum += num.val;
                            break 'number_loop;
                        }
                    }
                }
            }
        }
        sum
    }
}

fn parse_input(input: &str) -> Result<Input> {
    // let grid = input
    //     .lines()
    //     .map(|l| l.chars().collect::<Vec<_>>())
    //     .collect::<Vec<_>>();
    let lines: Vec<String> = input.lines().map(|s| String::from(s)).collect();

    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let number_regex = RE.get_or_init(|| regex::Regex::new(r"\d+").unwrap());

    let mut numbers = Vec::new(); // TODO with_capacity?
    for line in lines.iter() {
        let numbers_in_line: Vec<Number> = number_regex
            .find_iter(line)
            .map(|m| Number {
                val: m
                    .as_str()
                    .parse::<i128>()
                    .expect("parse i128 was not possible with found regex"),
                start: m.start(),
                end: m.end(),
            })
            .collect();
        numbers.push(numbers_in_line);
    }

    Ok(Input { lines, numbers })
}

fn solve_one(input: &Input) -> Result<Answer> {
    println!("{:?}", input);
    Ok(Answer::Num(input.sum_adjacent()))
}

fn solve_two(input: &Input) -> Result<Answer> {
    Ok(Answer::Num(input.sum_adjacent()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> =
        LazyLock::new(|| test_utils::read_from_file("src/day03/test.txt"));
    static INPUT: LazyLock<String> =
        LazyLock::new(|| test_utils::read_from_file("src/day03/input.txt"));

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(4361));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        b.iter(|| part_one())
    }
    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        b.iter(|| part_two())
    }
}
