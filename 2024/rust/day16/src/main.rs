#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::Answer;
use core::panic;
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
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

// North, West, South, East
const DROW: [i128; 4] = [-1, 0, 1, 0];
const DCOL: [i128; 4] = [0, 1, 0, -1];

struct Input {
    grid: Vec<Vec<char>>,
    start: (i128, i128),
    end: (i128, i128),
}

fn parse_input(input: &str) -> Result<Input> {
    let mut grid = Vec::new();
    let mut start = None;
    let mut end = None;
    for (row, line) in input.trim().lines().enumerate() {
        let mut grid_row = Vec::new();
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some((row as i128, col as i128));
            }
            if c == 'E' {
                end = Some((row as i128, col as i128));
            }
            grid_row.push(c);
        }
        grid.push(grid_row);
    }
    let grid: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
    Ok(Input {
        grid,
        start: start.ok_or(anyhow!("start is expected"))?,
        end: end.ok_or(anyhow!("end is expected"))?,
    })
}

fn inside(row: i128, col: i128, rows: i128, cols: i128) -> bool {
    0 <= row && row < rows && 0 <= col && col < cols
}

fn turn(dir: i32, orientation: i32) -> i32 {
    let mut new_orientation = orientation + dir;
    if new_orientation < 0 {
        new_orientation += 4;
    }
    if new_orientation >= 4 {
        new_orientation -= 4;
    }
    new_orientation
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { grid, start, end } = input;
    let rows = grid.len() as i128;
    let cols = grid[0].len() as i128;
    // (-cost, row, col, orientation)
    let initial = (0, start.0, start.1, 3);
    let mut visited = HashSet::new();
    let mut prio_queue = BinaryHeap::new();
    prio_queue.push(initial);
    let mut final_cost = None;
    while let Some(node) = prio_queue.pop() {
        let (cost, row, col, orientation) = node;
        if visited.contains(&(row, col, orientation)) {
            continue;
        }
        if (row, col) == *end {
            final_cost = Some(-cost);
            break;
        }
        visited.insert((row, col, orientation));
        let left_orientation = turn(-1, orientation);
        let right_orientation = turn(1, orientation);
        let nrow = row + DROW[orientation as usize];
        let ncol = col + DCOL[orientation as usize];
        if inside(nrow, ncol, rows, cols) && grid[nrow as usize][ncol as usize] != '#' {
            prio_queue.push((cost - 1, nrow, ncol, orientation));
        }
        prio_queue.push((cost - 1000, row, col, left_orientation));
        prio_queue.push((cost - 1000, row, col, right_orientation));
    }

    Ok(Answer::Num(
        final_cost.ok_or(anyhow!("there should be a path to E"))?,
    ))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { grid, start, end } = input;
    let rows = grid.len() as i128;
    let cols = grid[0].len() as i128;
    // (-cost, row, col, orientation, old_row, old_col, old_orientation)
    let initial = (0, start.0, start.1, 3, start.0, start.1, 3);
    let mut best: HashMap<(i128, i128, i32), i128> = HashMap::new();
    let mut visited = HashSet::new();
    let mut prio_queue = BinaryHeap::new();
    let mut best_graph: HashMap<(i128, i128, i32), Vec<(i128, i128, i32)>> = HashMap::new();
    prio_queue.push(initial);
    let mut final_cost = None;
    while let Some(node) = prio_queue.pop() {
        let (cost, row, col, orientation, old_row, old_col, old_orientation) = node;
        if visited.contains(&(row, col, orientation)) {
            let best_val = *best.get(&(row, col, orientation)).unwrap();
            if -cost < best_val && best_val != 0 {
                println!(
                    "best was {}, current was {} at {},{},{}",
                    best_val, -cost, row, col, orientation
                );
                panic!("why")
            }
            if -cost <= *best.get(&(row, col, orientation)).unwrap() {
                best_graph
                    .entry((row, col, orientation))
                    .and_modify(|v| v.push((old_row, old_col, old_orientation)))
                    .or_insert(vec![(old_row, old_col, old_orientation)]);
            }
            continue;
        }
        best.insert((row, col, orientation), -cost);
        best_graph
            .entry((row, col, orientation))
            .and_modify(|v| v.push((old_row, old_col, old_orientation)))
            .or_insert(vec![(old_row, old_col, old_orientation)]);
        visited.insert((row, col, orientation));
        if (row, col) == *end {
            final_cost = Some(-cost);
            break; // E must only be reachable from one previous tile otherwise this is potentially wrong
        }
        let left_orientation = turn(-1, orientation);
        let right_orientation = turn(1, orientation);
        let nrow = row + DROW[orientation as usize];
        let ncol = col + DCOL[orientation as usize];
        if inside(nrow, ncol, rows, cols) && grid[nrow as usize][ncol as usize] != '#' {
            prio_queue.push((cost - 1, nrow, ncol, orientation, row, col, orientation));
        }
        prio_queue.push((
            cost - 1000,
            row,
            col,
            left_orientation,
            row,
            col,
            orientation,
        ));
        prio_queue.push((
            cost - 1000,
            row,
            col,
            right_orientation,
            row,
            col,
            orientation,
        ));
    }

    // now count nodes in best_graph with simple bfs
    let mut queue = VecDeque::new();
    let mut count_visited = HashSet::new();
    queue.push_back((end.0, end.1, 0));
    queue.push_back((end.0, end.1, 1));
    queue.push_back((end.0, end.1, 2));
    queue.push_back((end.0, end.1, 3));
    count_visited.insert((end.0, end.1, 0));
    count_visited.insert((end.0, end.1, 1));
    count_visited.insert((end.0, end.1, 2));
    count_visited.insert((end.0, end.1, 3));
    while let Some((row, col, orientation)) = queue.pop_front() {
        if let Some(neighs) = best_graph.get(&(row, col, orientation)) {
            for neigh in neighs.iter() {
                if !count_visited.contains(neigh) {
                    queue.push_back(*neigh);
                    count_visited.insert(*neigh);
                }
            }
        }
    }
    let count_visited: HashSet<(i128, i128)> =
        count_visited.iter().map(|&(r, c, _)| (r, c)).collect();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if count_visited.contains(&(row as i128, col as i128)) {
                print!("O");
            } else {
                print!("{}", grid[row][col])
            }
        }
        println!();
    }

    Ok(Answer::Num(count_visited.iter().count() as i128))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day16_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(11048));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(89460));
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
        assert_eq!(answer, Answer::Num(64));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
