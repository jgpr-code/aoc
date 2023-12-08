use super::common::*;
use anyhow::Result;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    directions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

fn parse_input(input: &str) -> Result<Input> {
    let (directions, nodes_str) = input.split_once("\r\n\r\n").unwrap();
    let directions = directions.trim().chars().collect::<Vec<_>>();
    let re = regex!(r"(\w+) = \((\w+), (\w+)\)");
    let mut nodes = HashMap::new();
    for node in nodes_str.lines() {
        let caps = re.captures(node).unwrap();
        nodes.insert(
            String::from(&caps[1]),
            (String::from(&caps[2]), String::from(&caps[3])),
        );
    }

    Ok(Input { directions, nodes })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let mut current = String::from("AAA");
    let mut directions = input.directions.iter().cycle();
    let mut steps = 0;
    loop {
        if current == "ZZZ" {
            break;
        }
        let dir = directions.next().unwrap();
        let (left, right) = input.nodes.get(&current).unwrap();
        match dir {
            'L' => current = left.clone(),
            'R' => current = right.clone(),
            _ => unreachable!(),
        }
        steps += 1;
    }
    Ok(Answer::Num(steps))
}

fn solve_two(input: &Input) -> Result<Answer> {
    for (k, v) in input.nodes.iter() {}
    Ok(Answer::Num(0))
}

fn solve_helper(input: &Input, start: &str) {}

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
        assert_eq!(answer, Answer::Num(1));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(1));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST2)?;
        assert_eq!(answer, Answer::Num(2));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(2));
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
