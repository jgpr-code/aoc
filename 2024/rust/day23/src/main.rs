#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::Answer;
use std::{
    collections::{HashMap, HashSet},
    io,
};

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
    nodes: HashSet<String>,
    graph: HashMap<String, HashSet<String>>,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    let mut nodes = HashSet::new();
    for line in input.trim().lines() {
        let (a, b) = line
            .split_once("-")
            .ok_or(anyhow!("not a valid connection"))?;
        let a = a.to_string();
        let b = b.to_string();
        graph
            .entry(a.clone())
            .and_modify(|s| {
                s.insert(b.clone());
            })
            .or_insert(HashSet::from([b.clone()]));
        graph
            .entry(b.clone())
            .and_modify(|s| {
                s.insert(a.clone());
            })
            .or_insert(HashSet::from([a.clone()]));
        nodes.insert(a);
        nodes.insert(b);
    }
    Ok(Input { nodes, graph })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { nodes, graph } = input;
    let nodes_vec = Vec::from_iter(nodes.iter().cloned());
    let n = nodes_vec.len();
    let mut clique3 = HashSet::new();
    for i in 0..n {
        let node_i = nodes_vec[i].clone();
        for j in 0..n {
            if i == j {
                continue;
            }
            let node_j = nodes_vec[j].clone();
            let j_set = graph
                .get(&node_j.clone())
                .ok_or(anyhow!("no node {}", node_j))?;
            // i and j must be connected
            if !j_set.contains(&node_i) {
                continue;
            }
            for k in 0..n {
                if i == k || j == k {
                    continue;
                }
                let node_k = nodes_vec[k].clone();
                let k_set = graph
                    .get(&node_k.clone())
                    .ok_or(anyhow!("no node {}", node_k))?;
                // j and k must be connected
                // i and k must be connected
                if !k_set.contains(&node_i) || !k_set.contains(&node_j) {
                    continue;
                }
                let mut v = vec![node_i.clone(), node_j.clone(), node_k.clone()];
                v.sort();
                clique3.insert(v);
            }
        }
    }
    // for clique in clique3.iter() {
    //     println!("{},{},{}", clique[0], clique[1], clique[2]);
    // }
    let answer = clique3
        .iter()
        .filter(|v| v.iter().any(|node| node.starts_with("t")))
        .count();
    Ok(Answer::Num(answer as i128))
}

fn maximal_clique_with(
    node: &String,
    all_nodes: &HashSet<String>,
    graph: &HashMap<String, HashSet<String>>,
) -> Result<HashSet<String>> {
    let mut clique = HashSet::from([node.clone()]);
    for next_node in all_nodes.iter() {
        // next_node must be connected to all nodes in clique
        let next_node_set = graph
            .get(next_node)
            .ok_or(anyhow!("no node {}", next_node))?;
        if clique.iter().all(|elem| next_node_set.contains(elem)) {
            clique.insert(next_node.clone());
        }
    }
    Ok(clique)
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { nodes, graph } = input;
    let (_size, maximum) = nodes
        .iter()
        .map(|n| {
            let maximal =
                maximal_clique_with(n, nodes, graph).expect("maximal clique should not fail");
            (maximal.len(), maximal)
        })
        .max_by(|(asize, _), (bsize, _)| asize.cmp(&bsize))
        .ok_or(anyhow!("there must be a maximum"))?;
    let mut maximum = Vec::from_iter(maximum.into_iter());
    maximum.sort();
    Ok(Answer::Str(maximum.join(",")))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day23_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(7));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(1269));
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
        assert_eq!(answer, Answer::from("co,de,ka,ta"));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(
            answer,
            Answer::from("ad,jw,kt,kz,mt,nc,nr,sb,so,tg,vs,wh,yh")
        );
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
