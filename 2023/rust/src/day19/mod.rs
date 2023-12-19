use std::collections::{HashMap, HashSet, VecDeque};

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

#[derive(Clone)]
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

#[derive(Clone)]
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
    let mut sum = 0;
    for part in parts.iter() {
        let mut next_wf = String::from("in");
        loop {
            let wf = workflows.get(&next_wf).unwrap();
            print!("{} -> ", next_wf);
            next_wf = wf.apply(part);
            if next_wf == "A" {
                sum += part.x + part.m + part.a + part.s;
                print!("A");
                break;
            } else if next_wf == "R" {
                print!("R");
                break;
            }
        }
        println!();
    }
    println!("sum = {}", sum);
    Ok(Answer::Num(sum))
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct XmasRule {
    x: (i128, i128),
    m: (i128, i128),
    a: (i128, i128),
    s: (i128, i128),
}

impl XmasRule {
    fn from_vec(v: &[(i128, i128)]) -> Self {
        assert_eq!(v.len(), 4);
        XmasRule {
            x: v[0],
            m: v[1],
            a: v[2],
            s: v[3],
        }
    }
    fn to_vec(&self) -> Vec<(i128, i128)> {
        vec![self.x, self.m, self.a, self.s]
    }
    fn idx(c: char) -> usize {
        match c {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!("not expected"),
        }
    }
    fn is_consistent(&self) -> bool {
        let x = self.x;
        let m = self.m;
        let a = self.a;
        let s = self.s;
        x.0 <= x.1 && m.0 <= m.1 && a.0 <= a.1 && s.0 <= s.1
    }
    fn combine(&self, other: &Self) -> Self {
        let x = (
            std::cmp::max(self.x.0, other.x.0),
            std::cmp::min(self.x.1, other.x.1),
        );
        let m = (
            std::cmp::max(self.m.0, other.m.0),
            std::cmp::min(self.m.1, other.m.1),
        );
        let a = (
            std::cmp::max(self.a.0, other.a.0),
            std::cmp::min(self.a.1, other.a.1),
        );
        let s = (
            std::cmp::max(self.s.0, other.s.0),
            std::cmp::min(self.s.1, other.s.1),
        );
        XmasRule { x, m, a, s }
    }
    fn count(&self) -> i128 {
        assert!(self.is_consistent());
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
    // fn apply_rule(&self, rule: &Rule) -> Self {
    //     match rule {
    //         &Rule::LessThan(, , ) => todo!(),
    //         &Rule::MoreThan(, , ) => todo!(),
    //         &Rule::Goto() => todo!(),
    //     }
    // }
}

fn solve(input: &Input) -> Result<Answer> {
    let Input {
        workflows,
        parts: _,
    } = input;

    let answer = solve_helper(
        String::from("in"),
        workflows,
        vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
    );
    Ok(Answer::Num(answer))
}

fn solve_helper(
    node: String,
    wfmap: &HashMap<String, Workflow>,
    allowed: Vec<(i128, i128)>,
) -> i128 {
    if node == "A" {
        return XmasRule::from_vec(&allowed).count();
    }
    if node == "R" {
        return 0;
    }

    let mut sum = 0;
    let wf = wfmap.get(&node).unwrap();
    let mut still_allowed = allowed.clone();
    for rule in wf.rules.iter() {
        let target: String;
        let mut if_taken = still_allowed.clone();
        match rule {
            Rule::LessThan(c, l, t) => {
                target = t.clone();
                let idx = XmasRule::idx(*c);
                let (low, up) = still_allowed[idx];
                if_taken[idx] = (low, *l - 1);
                still_allowed[idx] = (*l, up);
            }
            Rule::MoreThan(c, l, t) => {
                target = t.clone();
                let idx = XmasRule::idx(*c);
                let (low, up) = still_allowed[idx];
                if_taken[idx] = (*l + 1, up);
                still_allowed[idx] = (low, *l);
            }
            Rule::Goto(t) => target = t.clone(),
        }
        if XmasRule::from_vec(&if_taken).is_consistent() {
            sum += solve_helper(target, wfmap, if_taken);
        }
        // or try again
    }
    sum
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input {
        workflows,
        parts: _,
    } = input;
    let mut graph: HashMap<String, Vec<(XmasRule, String)>> = HashMap::new();
    for (k, v) in workflows.iter() {
        let mut x = (1, 4000);
        let mut m = (1, 4000);
        let mut a = (1, 4000);
        let mut s = (1, 4000);
        for rule in v.rules.iter() {
            let mut xmas_rule = XmasRule { x, m, a, s };
            let target: String;
            match rule {
                // k -> t if c < l => t -> k if c < l, c >= l
                Rule::LessThan(c, l, t) => {
                    target = t.clone();
                    match c {
                        'x' => {
                            let nx = (x.0, std::cmp::min(l - 1, x.1));
                            xmas_rule = XmasRule { x: nx, m, a, s };
                            x = (*l, 4000);
                        }
                        'm' => {
                            let nm = (m.0, std::cmp::min(l - 1, m.1));
                            xmas_rule = XmasRule { x, m: nm, a, s };
                            m = (*l, 4000);
                        }
                        'a' => {
                            let na = (a.0, std::cmp::min(l - 1, a.1));
                            xmas_rule = XmasRule { x, m, a: na, s };
                            a = (*l, 4000);
                        }
                        's' => {
                            let ns = (s.0, std::cmp::min(l - 1, s.1));
                            xmas_rule = XmasRule { x, m, a, s: ns };
                            s = (*l, 4000);
                        }
                        _ => panic!("unexpected char"),
                    };
                }
                Rule::MoreThan(c, l, t) => {
                    target = t.clone();
                    match c {
                        'x' => {
                            let nx = (std::cmp::max(l + 1, x.0), x.1);
                            xmas_rule = XmasRule { x: nx, m, a, s };
                            x = (1, *l);
                        }
                        'm' => {
                            let nm = (std::cmp::max(l + 1, m.0), m.1);
                            xmas_rule = XmasRule { x, m: nm, a, s };
                            m = (1, *l);
                        }
                        'a' => {
                            let na = (std::cmp::max(l + 1, a.0), a.1);
                            xmas_rule = XmasRule { x, m, a: na, s };
                            a = (1, *l);
                        }
                        's' => {
                            let ns = (std::cmp::max(l + 1, s.0), s.1);
                            xmas_rule = XmasRule { x, m, a, s: ns };
                            s = (1, *l);
                        }
                        _ => panic!("unexpected char"),
                    };
                }
                Rule::Goto(t) => {
                    target = t.clone();
                }
            }
            if xmas_rule.is_consistent() {
                graph
                    .entry(target)
                    .and_modify(|e| e.push((xmas_rule, k.clone())))
                    .or_insert(vec![(xmas_rule, k.clone())]);
            }
        }
    }
    println!("{:?}", graph);
    // build graph -> do bfs while keeping track of current allowed values -> start from back with all A's invert conditions
    // doesn't make sense to take node twice as every node just restricts more! is it really true? path there could be less restrictive?
    let mut q = VecDeque::new();
    //let mut vis = HashSet::new();
    q.push_back((
        XmasRule {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        String::from("A"),
    ));
    let mut found_ins = HashSet::new();
    while let Some((rule, node)) = q.pop_front() {
        let neighs = graph.get(&node).unwrap();
        for (nrule, nnode) in neighs {
            let crule = rule.combine(&nrule);
            if crule.is_consistent() {
                if nnode == "in" {
                    found_ins.insert(crule);
                } else {
                    q.push_back((crule, nnode.clone()));
                }
            }
        }
    }
    println!();
    println!("{:?}", found_ins);
    let mut sum = 0;
    for found_in in found_ins.iter() {
        sum += found_in.count();
    }
    let fvec = found_ins.iter().collect::<Vec<_>>();
    for i in 0..fvec.len() {
        for j in i + 1..fvec.len() {
            let combine = fvec[i].combine(fvec[j]);
            if combine.is_consistent() {
                println!("{:?} + {:?}", fvec[i], fvec[j]);
                println!("{:?}", combine);
                sum -= combine.count();
            }
        }
    }

    // (1,3) (2,5)
    // (2,3) (3,7)

    // (1) (2,5)
    // (2,3) (6,7)
    // ...

    // maybe graph is DAG?
    // combine node strings into path string and use that to check if path was already done before
    solve(input)
    // Ok(Answer::Num(sum))
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
        assert_eq!(answer, Answer::Num(19114));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(456651));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(167409079868000));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(131899818301477));
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
