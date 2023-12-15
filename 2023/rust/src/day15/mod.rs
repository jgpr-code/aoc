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
    sequence: Vec<String>,
}

fn parse_input(input: &str) -> Result<Input> {
    let sequence: Vec<String> = input.split(",").map(|s| String::from(s)).collect();
    Ok(Input { sequence })
}

// Determine the ASCII code for the current character of the string.
// Increase the current value by the ASCII code you just determined.
// Set the current value to itself multiplied by 17.
// Set the current value to the remainder of dividing itself by 256.
fn hash(step: &str) -> usize {
    let mut curr = 0;
    for c in step.chars() {
        let cu32 = c as u32;
        curr += cu32 as usize;
        curr *= 17;
        curr %= 256;
    }
    curr
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { sequence } = input;
    let mut sum = 0;
    for seq in sequence {
        sum += hash(&seq);
    }
    Ok(Answer::Num(sum as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    todo!();
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
    #[test]
    fn test_hash() {
        let answer = hash("HASH");
        assert_eq!(answer, 52);
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
