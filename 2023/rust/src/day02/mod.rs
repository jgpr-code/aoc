use super::common::*;
use anyhow::{anyhow, Result};

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

#[derive(Debug)]
struct Game {
    id: i128,
    sets: Vec<GameSet>,
}

#[derive(Debug)]
struct GameSet {
    red: i128,
    green: i128,
    blue: i128,
}

#[derive(Debug)]
struct Input {
    games: Vec<Game>,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.sets.iter().all(|s| s.is_possible())
    }
    fn fewest_possible(&self) -> GameSet {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for set in self.sets.iter() {
            max_red = std::cmp::max(max_red, set.red);
            max_green = std::cmp::max(max_green, set.green);
            max_blue = std::cmp::max(max_blue, set.blue);
        }
        GameSet {
            red: max_red,
            green: max_green,
            blue: max_blue,
        }
    }
}

impl GameSet {
    fn is_possible(&self) -> bool {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }
    fn power(&self) -> i128 {
        self.red * self.green * self.blue
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let games = input
        .lines()
        .map(|l| parse_game(l))
        .collect::<Result<Vec<Game>>>()?;
    Ok(Input { games })
}

fn parse_game(line: &str) -> Result<Game> {
    // println!("{:?}", line);
    let v: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
    // println!("{:?}", v);
    let id = v[0]
        .split(" ")
        .nth(1)
        .ok_or(anyhow!("nth(1) was None"))?
        .parse::<i128>()?;
    let sets = v[1];
    let s: Vec<&str> = sets.split(";").map(|s| s.trim()).collect();
    let sets: Vec<GameSet> = s
        .into_iter()
        .map(|s| parse_set(s))
        .collect::<Result<Vec<GameSet>>>()?;
    Ok(Game { id, sets })
}

fn parse_set(set: &str) -> Result<GameSet> {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    let components: Vec<&str> = set.split(",").map(|s| s.trim()).collect();
    for c in components {
        let v: Vec<&str> = c.split(" ").collect();
        let amount = v[0].parse::<i128>()?;
        match v[1] {
            "red" => red += amount,
            "green" => green += amount,
            "blue" => blue += amount,
            _ => panic!("parse_set error"),
        }
    }
    Ok(GameSet { red, green, blue })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { games } = input;
    let sum = games.iter().filter(|g| g.is_valid()).map(|g| g.id).sum();
    Ok(Answer::Num(sum))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { games } = input;
    let sum = games.iter().map(|g| g.fewest_possible().power()).sum();
    Ok(Answer::Num(sum))
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
        assert_eq!(answer, Answer::Num(8));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(2720));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(2286));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(71535));
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
