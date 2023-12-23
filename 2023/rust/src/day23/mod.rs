use std::{collections::HashSet, ops::Index};

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
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    start: (usize, usize),
    goal: (usize, usize),
}

fn parse_input(input: &str) -> Result<Input> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut start = (0, 0);
    let mut goal = (rows - 1, cols - 1);
    for c in 0..cols {
        if grid[0][c] == '.' {
            start = (0, c);
        }
        if grid[rows - 1][c] == '.' {
            goal = (rows - 1, c);
        }
    }
    Ok(Input {
        grid,
        rows,
        cols,
        start,
        goal,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input {
        grid,
        rows,
        cols,
        start,
        goal,
    } = input;
    let mut max_steps = 0;
    let mut visited = HashSet::new();
    visited.insert((start.0 as i32, start.1 as i32));
    dfs(
        (start.0 as i32, start.1 as i32),
        (goal.0 as i32, goal.1 as i32),
        &mut visited,
        0,
        &mut max_steps,
        vec!['^', '>', 'v', '<'],
        grid,
    );
    Ok(Answer::Num(max_steps as i128))
}

fn dfs(
    pos: (i32, i32),
    goal: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
    current_steps: usize,
    max_steps: &mut usize,
    da: Vec<char>,
    grid: &Vec<Vec<char>>,
) {
    if pos == goal {
        println!("reached goal in {} steps", current_steps);
        *max_steps = std::cmp::max(*max_steps, current_steps);
        return;
    }
    // println!("{:?}", pos);
    let rows = grid.len();
    let cols = grid[0].len();
    let dr = vec![-1, 0, 1, 0];
    let dc = vec![0, 1, 0, -1];
    let c = grid[pos.0 as usize][pos.1 as usize];
    let cp = da.iter().position(|c_da| *c_da == c);
    for i in 0..4 {
        let nr = pos.0 + dr[i];
        let nc = pos.1 + dc[i];
        if nr < 0
            || nr >= rows as i32
            || nc < 0
            || nc >= cols as i32
            || grid[nr as usize][nc as usize] == '#'
        {
            continue; //outside field or wall
        }
        if let Some(cp) = cp {
            if i != cp {
                continue;
            }
        }
        let npos = (nr, nc);
        if visited.contains(&npos) {
            continue;
        }
        visited.insert(npos);
        dfs(
            npos,
            goal,
            visited,
            current_steps + 1,
            max_steps,
            da.clone(),
            grid,
        );
        visited.remove(&npos);
    }
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input {
        grid,
        rows,
        cols,
        start,
        goal,
    } = input;
    let mut max_steps = 0;
    let mut visited = HashSet::new();
    visited.insert((start.0 as i32, start.1 as i32));
    dfs(
        (start.0 as i32, start.1 as i32),
        (goal.0 as i32, goal.1 as i32),
        &mut visited,
        0,
        &mut max_steps,
        vec![],
        grid,
    );
    Ok(Answer::Num(max_steps as i128))
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
        assert_eq!(answer, Answer::Num(94));
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
    fn bench_art_two(b: &mut Bencher) {
        b.iter(|| part_two())
    }
}
