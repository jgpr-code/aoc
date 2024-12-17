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

// UP RIGHT DOWN LEFT
const DROW: [i128; 4] = [-1, 0, 1, 0];
const DCOL: [i128; 4] = [0, 1, 0, -1];

fn next_dir(dir: usize) -> usize {
    (dir + 1) % 4
}

struct Input {
    grid: Vec<Vec<char>>,
    guard_start: (usize, usize),
    row_obstacles: HashMap<usize, Vec<usize>>, // obstacles cols arranged by row
    col_obstacles: HashMap<usize, Vec<usize>>, // obstacles rows arranged by col
}

struct WalkingGuard {
    field: Vec<Vec<char>>,
    guard: Guard,
    row_obstacles: HashMap<usize, Vec<usize>>,
    col_obstacles: HashMap<usize, Vec<usize>>,
    loop_possibilities: usize,
}

struct Guard {
    pos: (i128, i128),
    dir: usize,
}

impl Guard {
    fn turn_right(&mut self) {
        self.dir = (self.dir + 1) % 4;
    }
}

impl WalkingGuard {
    fn create(
        field: &Vec<Vec<char>>,
        guard_start: (usize, usize),
        row_obstacles: &HashMap<usize, Vec<usize>>,
        col_obstacles: &HashMap<usize, Vec<usize>>,
    ) -> Self {
        Self {
            field: field.clone(),
            guard: Guard {
                pos: (guard_start.0 as i128, guard_start.1 as i128),
                dir: 0,
            },
            row_obstacles: row_obstacles.clone(),
            col_obstacles: col_obstacles.clone(),
            loop_possibilities: 0,
        }
    }
    // false if guard left the field
    fn walk(&mut self) -> bool {
        let drow = vec![-1, 0, 1, 0];
        let dcol = vec![0, 1, 0, -1];
        // mark field on
        let (row, col) = self.guard.pos;
        self.field[row as usize][col as usize] = 'X';
        if self.loop_possible() {
            self.loop_possibilities += 1;
        }
        // determine walk
        let nrow = row + drow[self.guard.dir];
        let ncol = col + dcol[self.guard.dir];
        if !self.inside(nrow, ncol) {
            return false; // guard left
        }
        if self.field[nrow as usize][ncol as usize] == '#' {
            self.guard.turn_right();
            // unmark field
            self.field[row as usize][col as usize] = '.';
        } else {
            self.guard.pos = (nrow, ncol);
        }
        true
    }
    fn loop_possible(&self) -> bool {
        // 0 is UP
        let next_dir = (self.guard.dir + 1) % 4;
        if next_dir % 2 == 0 {
            // check col_obstacles
        } else {
            // check row_obstacles
        }
        false
    }
    fn loop_walk(&mut self) {
        while self.walk() {}
    }
    fn inside(&self, row: i128, col: i128) -> bool {
        0 <= row && row < self.field.len() as i128 && 0 <= col && col < self.field[0].len() as i128
    }
    fn count_walked(&self) -> usize {
        self.field
            .iter()
            .map(|v| v.iter().filter(|&c| *c == 'X').count())
            .sum()
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let mut guard_pos = None;
    let mut grid = Vec::new();
    let mut row_obstacles: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut col_obstacles: HashMap<usize, Vec<usize>> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        let mut line_chars = Vec::new();
        for (col, c) in line.chars().enumerate() {
            if c == '^' {
                guard_pos = Some((row, col));
            }
            if c == '#' {
                row_obstacles
                    .entry(row)
                    .and_modify(|v| v.push(col))
                    .or_insert(vec![col]);
                col_obstacles
                    .entry(col)
                    .and_modify(|v| v.push(row))
                    .or_insert(vec![row]);
            }
            line_chars.push(c);
        }
        grid.push(line_chars);
    }
    for v in row_obstacles.values_mut() {
        v.sort()
    }
    for v in col_obstacles.values_mut() {
        v.sort();
    }
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let guard_start = guard_pos.ok_or(anyhow!("no guard on field"))?;
    Ok(Input {
        grid,
        guard_start,
        row_obstacles,
        col_obstacles,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input {
        grid,
        guard_start,
        row_obstacles,
        col_obstacles,
    } = input;
    let mut walking_guard = WalkingGuard::create(grid, *guard_start, row_obstacles, col_obstacles);
    walking_guard.loop_walk();

    Ok(Answer::Num(walking_guard.count_walked() as i128))
}

struct Line {
    start: (usize, usize),
    end: (usize, usize),
    dir: usize,
}

impl Line {
    fn new(start: (usize, usize), end: (usize, usize), dir: usize) -> Self {
        Self { start, end, dir }
    }
    fn intersect(
        &self,
        other: &Line,
        row_obstacles: &HashMap<usize, Vec<usize>>,
        col_obstacles: &HashMap<usize, Vec<usize>>,
    ) -> Option<(usize, usize)> {
        if next_dir(self.dir) != other.dir {
            return None;
        }
        let (s_row, s_col) = self.start;
        let (e_row, e_col) = self.end;
        let (os_row, os_col) = other.start;
        let (oe_row, oe_col) = other.end;
        match self.dir {
            0 => {
                // UP -> RIGHT
                assert!(os_row == oe_row);
                assert!(s_col == e_col);
                assert!(e_row < s_row);
                if e_row <= os_row && os_row <= s_row && s_col < oe_col {
                    let intersection = Some((os_row, s_col));
                    let row_obstacles = row_obstacles
                        .get(&os_row)
                        .expect("there should be a row obstacle");
                    let partition = row_obstacles.partition_point(|&col| col <= s_col);
                    if row_obstacles[partition] == oe_col + 1 {
                        return intersection;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            1 => {
                // RIGHT -> DOWN
                assert!(os_col == oe_col);
                assert!(s_row == e_row);
                assert!(s_col < e_col);
                if s_col <= os_col && os_col <= e_col && s_row < oe_row {
                    let intersection = Some((s_row, os_col));
                    let col_obstacles = col_obstacles
                        .get(&os_col)
                        .expect("there should be a col obstacle");
                    let partition = col_obstacles.partition_point(|&row| row <= s_row);
                    if col_obstacles[partition] == oe_row + 1 {
                        return intersection;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            2 => {
                // DOWN -> LEFT
                assert!(os_row == oe_row);
                assert!(s_col == e_col);
                assert!(s_row < e_row);
                if s_row <= os_row && os_row <= e_row && oe_col < s_col {
                    let intersection = Some((os_row, s_col));
                    let row_obstacles = row_obstacles
                        .get(&os_row)
                        .expect("there should be a row obstacle");
                    let partition = row_obstacles.partition_point(|&col| col <= s_col);
                    if row_obstacles[partition - 1] == oe_col - 1 {
                        return intersection;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            3 => {
                // LEFT -> UP
                assert!(os_col == oe_col);
                assert!(s_row == e_row);
                assert!(e_col < s_col);
                if e_col <= os_col && os_col <= s_col && oe_row < s_row {
                    let intersection = Some((s_row, os_col));
                    let col_obstacles = col_obstacles
                        .get(&os_col)
                        .expect("there should be a col obstacle");
                    let partition = col_obstacles.partition_point(|&row| row <= s_row);
                    if col_obstacles[partition - 1] == oe_row - 1 {
                        return intersection;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            _ => panic!("not a direction"),
        }
    }
}

fn inside(row: i128, col: i128, rows: i128, cols: i128) -> bool {
    0 <= row && row < rows && 0 <= col && col < cols
}

fn solve_two(input: &Input) -> Result<Answer> {
    // idea is:
    // for current line:
    // check if earlier lines with correct direction intersect and then check if one after this intersection is not visited
    // walk the line

    let Input {
        grid,
        guard_start,
        row_obstacles,
        col_obstacles,
    } = input;

    let mut grid_copy = grid.clone();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut row_guard = guard_start.0;
    let mut col_guard = guard_start.1;
    let mut guard_dir = 0;
    let mut existing_lines = Vec::new();
    let mut count_positions = 0;
    let mut count_loops = 0;
    let mut loop_obstacles = HashSet::new();
    loop {
        let mut break_loop = false;
        let (above, below) = vertical_obstacles(row_guard, col_guard, col_obstacles);
        let (left, right) = horizontal_obstacles(row_guard, col_guard, row_obstacles);
        match guard_dir {
            0 => {
                // UP
                let new_row = if let Some(obstacle_row) = above {
                    obstacle_row + 1
                } else {
                    break_loop = true;
                    0
                };
                let walked_line =
                    Line::new((row_guard, col_guard), (new_row, col_guard), guard_dir);
                for other_line in existing_lines.iter() {
                    if let Some((irow, icol)) =
                        walked_line.intersect(other_line, row_obstacles, col_obstacles)
                    {
                        let oirow = irow as i128 + DROW[guard_dir];
                        let oicol = icol as i128 + DCOL[guard_dir];
                        if inside(oirow, oicol, rows as i128, cols as i128)
                            && grid_copy[oirow as usize][oicol as usize] == '.'
                        {
                            count_loops += 1;
                            loop_obstacles.insert((oirow, oicol));
                        }
                    }
                }
                for row in new_row..=row_guard {
                    if grid_copy[row][col_guard] != 'X' {
                        grid_copy[row][col_guard] = 'X';
                        count_positions += 1;
                    }
                }
                row_guard = new_row;
                guard_dir = next_dir(guard_dir);
                existing_lines.push(walked_line);
            }
            1 => {
                // RIGHT
                let new_col = if let Some(obstacle_col) = right {
                    obstacle_col - 1
                } else {
                    break_loop = true;
                    cols - 1
                };
                let walked_line =
                    Line::new((row_guard, col_guard), (row_guard, new_col), guard_dir);
                for other_line in existing_lines.iter() {
                    if let Some((irow, icol)) =
                        walked_line.intersect(other_line, row_obstacles, col_obstacles)
                    {
                        let oirow = irow as i128 + DROW[guard_dir];
                        let oicol = icol as i128 + DCOL[guard_dir];
                        if inside(oirow, oicol, rows as i128, cols as i128)
                            && grid_copy[oirow as usize][oicol as usize] == '.'
                        {
                            count_loops += 1;
                            loop_obstacles.insert((oirow, oicol));
                        }
                    }
                }
                for col in col_guard..=new_col {
                    if grid_copy[row_guard][col] != 'X' {
                        grid_copy[row_guard][col] = 'X';
                        count_positions += 1;
                    }
                }
                col_guard = new_col;
                guard_dir = next_dir(guard_dir);
                existing_lines.push(walked_line);
            }
            2 => {
                // DOWN
                let new_row = if let Some(obstacle_row) = below {
                    obstacle_row - 1
                } else {
                    break_loop = true;
                    rows - 1
                };
                let walked_line =
                    Line::new((row_guard, col_guard), (new_row, col_guard), guard_dir);
                for other_line in existing_lines.iter() {
                    if let Some((irow, icol)) =
                        walked_line.intersect(other_line, row_obstacles, col_obstacles)
                    {
                        let oirow = irow as i128 + DROW[guard_dir];
                        let oicol = icol as i128 + DCOL[guard_dir];
                        if inside(oirow, oicol, rows as i128, cols as i128)
                            && grid_copy[oirow as usize][oicol as usize] == '.'
                        {
                            count_loops += 1;
                            loop_obstacles.insert((oirow, oicol));
                        }
                    }
                }
                for row in row_guard..=new_row {
                    if grid_copy[row][col_guard] != 'X' {
                        grid_copy[row][col_guard] = 'X';
                        count_positions += 1;
                    }
                }
                row_guard = new_row;
                guard_dir = next_dir(guard_dir);
                existing_lines.push(walked_line);
            }
            3 => {
                // LEFT
                let new_col = if let Some(obstacle_col) = left {
                    obstacle_col + 1
                } else {
                    break_loop = true;
                    0
                };
                let walked_line =
                    Line::new((row_guard, col_guard), (row_guard, new_col), guard_dir);
                for other_line in existing_lines.iter() {
                    if let Some((irow, icol)) =
                        walked_line.intersect(other_line, row_obstacles, col_obstacles)
                    {
                        let oirow = irow as i128 + DROW[guard_dir];
                        let oicol = icol as i128 + DCOL[guard_dir];
                        if inside(oirow, oicol, rows as i128, cols as i128)
                            && grid_copy[oirow as usize][oicol as usize] == '.'
                        {
                            count_loops += 1;
                            loop_obstacles.insert((oirow, oicol));
                        }
                    }
                }
                for col in new_col..=col_guard {
                    if grid_copy[row_guard][col] != 'X' {
                        grid_copy[row_guard][col] = 'X';
                        count_positions += 1;
                    }
                }
                col_guard = new_col;
                guard_dir = next_dir(guard_dir);
                existing_lines.push(walked_line);
            }
            _ => {
                panic!("not a direction!")
            }
        }
        if break_loop {
            break;
        }
    }
    println!("count positions: {}", count_positions);
    println!("count loops: {}", count_loops);
    for row in 0..rows {
        for col in 0..cols {
            if loop_obstacles.contains(&(row as i128, col as i128)) {
                print!("O");
            } else {
                print!("{}", grid_copy[row][col]);
            }
        }
        println!();
    }
    Ok(Answer::Num(count_loops))
}

fn vertical_obstacles(
    row_guard: usize,
    col_guard: usize,
    col_obstacles: &HashMap<usize, Vec<usize>>,
) -> (Option<usize>, Option<usize>) {
    if let Some(col_obstacles) = col_obstacles.get(&col_guard) {
        let hit_below = col_obstacles.partition_point(|&row| row < row_guard);
        let above = if hit_below > 0 {
            Some(col_obstacles[hit_below - 1])
        } else {
            None
        };
        let below = if hit_below < col_obstacles.len() {
            Some(col_obstacles[hit_below])
        } else {
            None
        };
        (above, below)
    } else {
        (None, None)
    }
}

fn horizontal_obstacles(
    row_guard: usize,
    col_guard: usize,
    row_obstacles: &HashMap<usize, Vec<usize>>,
) -> (Option<usize>, Option<usize>) {
    if let Some(row_obstacles) = row_obstacles.get(&row_guard) {
        let hit_right = row_obstacles.partition_point(|&col| col < col_guard);
        let left = if hit_right > 0 {
            Some(row_obstacles[hit_right - 1])
        } else {
            None
        };
        let right = if hit_right < row_obstacles.len() {
            Some(row_obstacles[hit_right])
        } else {
            None
        };
        (left, right)
    } else {
        (None, None)
    }
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day06_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(41));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(5551));
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
        assert_eq!(answer, Answer::Num(7)); // 6
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
