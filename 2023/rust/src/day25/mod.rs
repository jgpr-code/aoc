use std::collections::HashSet;

use super::common::*;
use anyhow::Result;
use pathfinding::directed::edmonds_karp::*;
use pathfinding::undirected::connected_components::*;

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    nodes: Vec<String>,
    edges: Vec<Edge<usize, i128>>,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut node_set = HashSet::new();
    for line in input.lines() {
        let (a, b) = line.split_once(":").unwrap();
        node_set.insert(a.trim().to_string());
        for bb in b.split_whitespace() {
            node_set.insert(bb.to_string());
        }
    }
    let mut nodes = Vec::from_iter(node_set.into_iter());
    nodes.sort();
    let mut edges = Vec::new();
    for line in input.lines() {
        let (a, b) = line.split_once(":").unwrap();
        let from = nodes.iter().position(|x| x == a.trim()).unwrap();
        for bb in b.split_whitespace() {
            let to = nodes.iter().position(|x| x == bb).unwrap();
            edges.push(((from, to), 1));
            edges.push(((to, from), 1));
        }
    }
    Ok(Input { nodes, edges })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { nodes, edges } = input;
    let inodes = nodes
        .iter()
        .enumerate()
        .map(|(i, _n)| i)
        .collect::<Vec<_>>();
    let mut answer = 0;
    'outer: for s in 0..inodes.len() {
        for t in s + 1..inodes.len() {
            let (_, _, min_cut) = edmonds_karp_sparse(&inodes, &s, &t, edges.clone().into_iter());
            // println!(
            //     "{}({}) -> {}({}) = {}",
            //     nodes[s],
            //     s,
            //     nodes[t],
            //     t,
            //     min_cut.len()
            // );
            if min_cut.len() == 3 {
                let mut new_edges = edges.clone();
                // println!("{:?}", new_edges.len());
                let mut starts: Vec<usize> = Vec::new();
                for ((cut_a, cut_b), _) in min_cut.iter() {
                    starts.push(*cut_a);
                    starts.push(*cut_b);
                    // println!(
                    //     "removing {}({}), {}({})",
                    //     nodes[*cut_a], *cut_a, nodes[*cut_b], *cut_b
                    // );
                    new_edges = new_edges
                        .into_iter()
                        .filter(|((a, b), _)| {
                            (a != cut_a || b != cut_b) && (a != cut_b || b != cut_a)
                        })
                        .collect();
                }
                // println!("{:?}", new_edges.len());

                let components = connected_components(&inodes, |n| {
                    let neighs: Vec<usize> = new_edges
                        .iter()
                        .filter(|((a, _), _)| *a == *n)
                        .map(|((_, b), _)| *b)
                        .collect();
                    neighs
                });
                // for com in components.iter() {
                //     println!("com count {}", com.len());
                //     for k in com.iter() {
                //         print!("{}({}), ", nodes[*k], k);
                //     }
                //     println!();
                //     println!("com is {:?}", com);
                // }
                assert_eq!(components.len(), 2);
                answer = components[0].len() * components[1].len();
                break 'outer;
            }
        }
    }
    Ok(Answer::Num(answer as i128))
}

fn solve_two(_input: &Input) -> Result<Answer> {
    Ok(Answer::Str(String::from("All tasks are done :)")))
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
        assert_eq!(answer, Answer::Num(54));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(538560));
        Ok(())
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        b.iter(|| part_one())
    }
}
