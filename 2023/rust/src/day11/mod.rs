use std::collections::{HashMap, HashSet};

use super::common::*;
use anyhow::Result;

pub fn part_one(input: &str) -> Result<Answer> {
    let mut input = parse_input(input)?;
    solve_one(&mut input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let mut input = parse_input(input)?;
    solve_two(&mut input)
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos(usize, usize);

#[derive(Debug)]
struct Input {
    universe: Vec<Vec<char>>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
    galaxy_distances: HashMap<(Pos, Pos), i128>,
}

fn parse_input(input: &str) -> Result<Input> {
    let universe: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = universe.len();
    let cols = universe.first().unwrap().len();
    let mut empty_cols = HashSet::new();
    for c in 0..cols {
        let mut found = false;
        for r in 0..rows {
            if universe[r][c] != '.' {
                found = true;
                break;
            }
        }
        if !found {
            empty_cols.insert(c);
        }
    }
    let mut empty_rows = HashSet::new();
    for r in 0..rows {
        let mut found = false;
        for c in 0..cols {
            if universe[r][c] != '.' {
                found = true;
                break;
            }
        }
        if !found {
            empty_rows.insert(r);
        }
    }
    let mut galaxy_distances: HashMap<(Pos, Pos), i128> = HashMap::new();
    let mut galaxies: Vec<(Pos, i128, i128)> = Vec::new();
    let mut topleft_distances: Vec<Vec<i128>> = vec![vec![i128::MAX; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            if r == 0 && c == 0 {
                topleft_distances[r][c] = 0; // top-left corner always should stay 0 (no empty row or col fortunately)
                continue;
            }
            let mut topdist = i128::MAX;
            let mut leftdist = i128::MAX;
            if r >= 1 {
                let r_above = r - 1;
                topdist = topleft_distances[r_above][c] + 1;
                if empty_rows.contains(&r_above) {
                    topdist += 1000000 - 1;
                }
            }
            if c >= 1 {
                let c_left = c - 1;
                leftdist = topleft_distances[r][c_left] + 1;
                if empty_cols.contains(&c_left) {
                    leftdist += 1000000 - 1;
                }
            }
            topleft_distances[r][c] = std::cmp::min(topdist, leftdist);
        }
    }
    let mut topright_distances: Vec<Vec<i128>> = vec![vec![i128::MAX; cols]; rows];
    for r in 0..rows {
        for c in (0..cols).rev() {
            if r == 0 && c == cols - 1 {
                topright_distances[r][c] = 0; // top-right corner always should stay 0 (no empty row or col fortunately)
                continue;
            }
            let mut topdist = i128::MAX;
            let mut rightdist = i128::MAX;
            if r >= 1 {
                let r_above = r - 1;
                topdist = topright_distances[r_above][c] + 1;
                if empty_rows.contains(&r_above) {
                    topdist += 1000000 - 1;
                }
            }
            if c + 1 < cols {
                let c_right = c + 1;
                rightdist = topright_distances[r][c_right] + 1;
                if empty_cols.contains(&c_right) {
                    rightdist += 1000000 - 1;
                }
            }
            topright_distances[r][c] = std::cmp::min(topdist, rightdist);
        }
    }
    for r in 0..rows {
        for c in 0..cols {
            if universe[r][c] == '#' {
                galaxies.push((Pos(c, r), topleft_distances[r][c], topright_distances[r][c]));
            }
        }
    }
    // for r in 0..rows {
    //     for c in 0..cols {
    //         print!("{:02} ", topleft_distances[r][c]);
    //     }
    //     println!();
    // }
    // println!("-------");
    // for r in 0..rows {
    //     for c in 0..cols {
    //         print!("{:02} ", topright_distances[r][c]);
    //     }
    //     println!();
    // }
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (from_pos, topleft_from_dist, topright_from_dist) = &galaxies[i];
            let (to_pos, topleft_to_dist, topright_to_dist) = &galaxies[j];
            let k = (*from_pos, *to_pos);
            let v = std::cmp::max(
                i128::abs(topleft_from_dist - topleft_to_dist),
                i128::abs(topright_from_dist - topright_to_dist),
            );
            // println!("{} ({:?}) to {} ({:?}) = {}", i, from_pos, j, to_pos, v);
            galaxy_distances.insert(k, v);
        }
    }
    Ok(Input {
        universe,
        empty_rows,
        empty_cols,
        galaxy_distances,
    })
}

fn solve_one(input: &mut Input) -> Result<Answer> {
    // println!("{:?}", input);
    let answer = input.galaxy_distances.values().sum();
    Ok(Answer::Num(answer))
}

fn solve_two(input: &mut Input) -> Result<Answer> {
    todo!()
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
        assert_eq!(answer, Answer::Num(-2));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(-2));
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
