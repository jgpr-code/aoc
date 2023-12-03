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
    lines: Vec<String>,
}

fn parse_input(input: &str) -> Result<Input> {
    let lines: Vec<String> = input.lines().map(|l| String::from(l)).collect();
    Ok(Input { lines })
}

fn parse_line(line: &str) -> i128 {
    let mut first = None;
    let mut last = None;
    for c in line.chars() {
        if c.is_digit(10) {
            if first == None {
                first = Some(c);
            }
            last = Some(c);
        }
    }
    let fd = first.unwrap();
    let ld = last.unwrap();
    let ifd = fd.to_digit(10).unwrap() as i128;
    let ild = ld.to_digit(10).unwrap() as i128;
    ifd * 10 + ild
}

// doesn't work due to overlap
fn parse_advanced(line: &str) -> i128 {
    let spelled_digits = vec![
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ];
    let mut line = String::from(line);
    for (spelled, replacement) in spelled_digits {
        line = line.replace(spelled, replacement);
    }
    // println!("{:?}", line);
    let result = parse_line(&line);
    // println!("{:?}", result);
    result
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { lines } = input;

    let mut sum = 0;
    for line in lines {
        sum += parse_line(line);
    }
    Ok(Answer::Num(sum))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { lines } = input;

    let mut sum = 0;
    for line in lines {
        sum += parse_advanced(line);
    }
    Ok(Answer::Num(sum))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test.txt");
    static TEST2: LazyLock<String> = local_file!("test2.txt");
    static INPUT: LazyLock<String> = local_file!("input.txt");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(142));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(54450));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST2)?;
        assert_eq!(answer, Answer::Num(281));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(54265));
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
