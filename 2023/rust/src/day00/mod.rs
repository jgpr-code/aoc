use super::common::*;
use anyhow::Result;
use std::num::ParseIntError;

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    nums: Vec<i128>,
}

fn parse_input(input: &str) -> Result<Input> {
    let nums = input
        .lines()
        .map(|l| l.parse::<i128>())
        .collect::<Result<Vec<_>, ParseIntError>>()?;
    Ok(Input { nums })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { nums } = input;
    let sum = nums.iter().sum::<i128>();
    Ok(Answer::Num(sum))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { nums } = input;
    let squared_sum = nums.iter().map(|n| n * n).sum::<i128>();
    Ok(Answer::Num(squared_sum))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    // in theory std::sync::LazyLock would be best, but its not in stable yet
    lazy_static! {
        static ref TEST: String = read_from_file("src/day00/test.txt");
        static ref INPUT: String = read_from_file("src/day00/input.txt");
    }

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(6));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(15));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(14));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(55));
        Ok(())
    }
}
