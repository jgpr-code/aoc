use super::common::*;
use anyhow::Result;

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
    let nums = input.lines().map(|l| l.parse::<i128>().unwrap()).collect();
    Ok(Input { nums })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { nums } = input;
    let sum = nums.iter().sum::<i128>();
    println!("sum = {}", sum);
    Ok(Answer::Num(-1))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { nums } = input;
    let squared_sum = nums.iter().map(|n| n * n).sum::<i128>();
    println!("squared sum = {}", squared_sum);
    Ok(Answer::Num(-1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test.txt");
    static INPUT: LazyLock<String> = local_file!("input.txt");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(-1));
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
