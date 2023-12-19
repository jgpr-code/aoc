use std::collections::HashMap;

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

struct Part {
    x: i128,
    m: i128,
    a: i128,
    s: i128,
}

impl Part {
    fn get(&self, c: char) -> i128 {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("can't get given char"),
        }
    }
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn apply(&self, part: &Part) -> String {
        for rule in self.rules.iter() {
            match rule {
                Rule::LessThan(c, l, t) if part.get(*c) < *l => return t.clone(),
                Rule::MoreThan(c, l, t) if part.get(*c) > *l => return t.clone(),
                Rule::Goto(t) => return t.clone(),
                _ => {}
            };
        }
        panic!("rule does not apply to part");
    }
}

enum Rule {
    LessThan(char, i128, String),
    MoreThan(char, i128, String),
    Goto(String),
}

struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

fn parse_input(input: &str) -> Result<Input> {
    let (workflows_str, parts_str) = input.split_once("\r\n\r\n").unwrap();

    // vnz{m>1432:kj,m<699:kgb,a<3578:lk,sdf}
    let rule_re = regex!(r"([xmas])([<>])(\d+):([a-zA-Z]+)");

    let mut workflows = HashMap::new();
    for wf in workflows_str.lines() {
        let (name, rules_str) = wf.split_once("{").unwrap();
        let mut rules = Vec::new();
        for rule_str in rules_str.split(",") {
            if let Some(caps) = rule_re.captures(rule_str) {
                let c = caps.get(1).unwrap().as_str().chars().last().unwrap();
                let compare = caps.get(2).unwrap().as_str().chars().last().unwrap();
                let limit = caps.get(3).unwrap().as_str().parse::<i128>().unwrap();
                let target = caps.get(4).unwrap().as_str();
                let rule = match compare {
                    '<' => Rule::LessThan(c, limit, String::from(target)),
                    '>' => Rule::MoreThan(c, limit, String::from(target)),
                    _ => panic!("unexpected compare"),
                };
                rules.push(rule);
            } else {
                let target = rule_str.replace("}", "");
                rules.push(Rule::Goto(target));
            }
        }
        workflows.insert(String::from(name), Workflow { rules });
    }

    // {x=2195,m=736,a=244,s=439}
    let part_re = regex!(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}");

    let mut parts = Vec::new();
    for part_str in parts_str.lines() {
        let caps = part_re.captures(part_str).unwrap();
        let x = caps.get(1).unwrap().as_str().parse().unwrap();
        let m = caps.get(2).unwrap().as_str().parse().unwrap();
        let a = caps.get(3).unwrap().as_str().parse().unwrap();
        let s = caps.get(4).unwrap().as_str().parse().unwrap();
        parts.push(Part { x, m, a, s });
    }
    Ok(Input { workflows, parts })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { workflows, parts } = input;
    let mut accepted = 0;
    let mut rejected = 0;
    let mut sum = 0;
    for part in parts.iter() {
        let mut next_wf = String::from("in");
        loop {
            let wf = workflows.get(&next_wf).unwrap();
            print!("{} -> ", next_wf);
            next_wf = wf.apply(part);
            if next_wf == "A" {
                accepted += 1;
                sum += part.x + part.m + part.a + part.s;
                print!("A");
                break;
            } else if next_wf == "R" {
                rejected += 1;
                print!("R");
                break;
            }
        }
        println!();
    }
    println!("sum = {}", sum);
    Ok(Answer::Num(sum))
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

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        b.iter(|| part_one())
    }
    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        b.iter(|| part_two())
    }
}
