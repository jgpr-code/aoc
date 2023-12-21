use std::collections::{HashSet, VecDeque};

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
    start: (i128, i128),
}

fn inside(r: i128, c: i128, rows: usize, cols: usize) -> bool {
    r >= 0 && c >= 0 && r < rows as i128 && c < cols as i128
}

fn parse_input(input: &str) -> Result<Input> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut start = (-1, -1);
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                start = (r as i128, c as i128);
            }
        }
    }
    Ok(Input {
        grid,
        rows,
        cols,
        start,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input {
        grid,
        rows,
        cols,
        start,
    } = input;

    let dr = vec![-1, 0, 1, 0];
    let dc = vec![0, 1, 0, -1];

    let mut reach_grid = vec![vec![-1; *cols]; *rows];
    let mut q = VecDeque::new();
    q.push_back((*start, 0));
    while let Some((pos, steps)) = q.pop_front() {
        for i in 0..4 {
            let nr = pos.0 + dr[i];
            let nc = pos.1 + dc[i];
            if inside(nr, nc, *rows, *cols)
                && reach_grid[nr as usize][nc as usize] == -1
                && grid[nr as usize][nc as usize] != '#'
            {
                reach_grid[nr as usize][nc as usize] = steps + 1;
                q.push_back(((nr, nc), steps + 1));
            }
        }
    }
    let mut sum = 0;

    let req_steps = 64;
    for r in 0..*rows {
        for c in 0..*cols {
            if reach_grid[r][c] >= 0
                && reach_grid[r][c] <= req_steps
                && reach_grid[r][c] % 2 == req_steps % 2
            {
                sum += 1;
            }
        }
    }
    Ok(Answer::Num(sum))
}

fn to_inside(r: i128, c: i128, rows: usize, cols: usize) -> (i128, i128) {
    let mut nr = r;
    let mut nc = c;
    let irows = rows as i128;
    let icols = cols as i128;
    while nr < 0 {
        nr += irows;
    }
    while nr >= irows {
        nr -= irows;
    }
    while nc < 0 {
        nc += icols;
    }
    while nc >= icols {
        nc -= icols;
    }
    (nr, nc)
}

fn count_grid(grid: &Vec<Vec<i128>>, req_steps: i128) -> i128 {
    let mut sum = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] >= 0 && grid[r][c] <= req_steps && grid[r][c] % 2 == req_steps % 2 {
                sum += 1;
            }
        }
    }
    sum
}
fn fill_grid(
    starts: Vec<((i128, i128), i128)>,
    grid: &Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    req_steps: i128,
) -> Option<Vec<Vec<i128>>> {
    let dr = vec![-1, 0, 1, 0];
    let dc = vec![0, 1, 0, -1];
    let mut reach_grid: Vec<Vec<i128>> = vec![vec![-1; cols]; rows];
    let mut q = VecDeque::new();
    for (start, steps) in starts {
        if steps <= req_steps {
            reach_grid[start.0 as usize][start.1 as usize] = steps;
            q.push_back((start, steps));
        }
    }
    if q.len() == 0 {
        return None;
    }
    while let Some((pos, steps)) = q.pop_front() {
        for i in 0..4 {
            let nr = pos.0 + dr[i];
            let nc = pos.1 + dc[i];
            if inside(nr, nc, rows, cols)
                && (reach_grid[nr as usize][nc as usize] == -1
                    || reach_grid[nr as usize][nc as usize] > steps + 1)
                && grid[nr as usize][nc as usize] != '#'
            {
                reach_grid[nr as usize][nc as usize] = steps + 1;
                q.push_back(((nr, nc), steps + 1));
            }
        }
    }
    Some(reach_grid)
}

fn print_grid(grid: &Vec<Vec<i128>>) {
    println!();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            print!("{} ", grid[r][c]);
        }
        println!();
    }
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input {
        grid,
        rows,
        cols,
        start,
    } = input;

    // let req_steps = 26501365;
    let req_steps = 5000;

    let mut seen_grids = HashSet::new();

    let mut sum = 0;
    let reach_grid = fill_grid(vec![(*start, 0_i128)], grid, *rows, *cols, req_steps);
    let mut q = VecDeque::new();
    if let Some(grid) = reach_grid {
        let pos = (
            0,
            0,
            grid[0][0] % 2,
            grid[0][*cols - 1] % 2,
            grid[*rows - 1][0] % 2,
            grid[*rows - 1][*cols - 1] % 2,
        );
        q.push_back((grid, pos));
        seen_grids.insert(pos);
    }
    // let mut seen = 0;
    while let Some((reach_grid, grid_pos)) = q.pop_front() {
        sum += count_grid(&reach_grid, req_steps);

        // print_grid(&reach_grid);
        // seen += 1;
        // if seen == 10000 {
        //     break;
        // }

        // top
        let mut top_starts = Vec::new();
        for c in 0..*cols {
            let old_value = reach_grid[0][c];
            if old_value > -1 {
                let nr = *rows - 1;
                top_starts.push(((nr as i128, c as i128), old_value + 1));
            }
        }
        let top_grid = fill_grid(top_starts, grid, *rows, *cols, req_steps);
        if let Some(grid) = top_grid {
            let top_pos = (
                grid_pos.0 - 1,
                grid_pos.1,
                grid[0][0] % 2,
                grid[0][*cols - 1] % 2,
                grid[*rows - 1][0] % 2,
                grid[*rows - 1][*cols - 1] % 2,
            );
            if !seen_grids.contains(&top_pos) {
                seen_grids.insert(top_pos);
                q.push_back((grid, top_pos));
            }
        }

        // right
        let mut right_starts = Vec::new();
        for r in 0..*rows {
            let old_value = reach_grid[r][*cols - 1];
            if old_value > -1 {
                let nc = 0;
                right_starts.push(((r as i128, nc as i128), old_value + 1));
            }
        }
        let right_grid = fill_grid(right_starts, grid, *rows, *cols, req_steps);
        if let Some(grid) = right_grid {
            let right_pos = (
                grid_pos.0,
                grid_pos.1 + 1,
                grid[0][0] % 2,
                grid[0][*cols - 1] % 2,
                grid[*rows - 1][0] % 2,
                grid[*rows - 1][*cols - 1] % 2,
            );
            if !seen_grids.contains(&right_pos) {
                seen_grids.insert(right_pos);
                q.push_back((grid, right_pos));
            }
        }

        // bottom
        let mut bottom_starts = Vec::new();
        for c in 0..*cols {
            let old_value = reach_grid[*rows - 1][c];
            if old_value > -1 {
                let nr = 0;
                bottom_starts.push(((nr as i128, c as i128), old_value + 1));
            }
        }
        let bottom_grid = fill_grid(bottom_starts, grid, *rows, *cols, req_steps);
        if let Some(grid) = bottom_grid {
            let bottom_pos = (
                grid_pos.0 + 1,
                grid_pos.1,
                grid[0][0] % 2,
                grid[0][*cols - 1] % 2,
                grid[*rows - 1][0] % 2,
                grid[*rows - 1][*cols - 1] % 2,
            );
            if !seen_grids.contains(&bottom_pos) {
                seen_grids.insert(bottom_pos);
                q.push_back((grid, bottom_pos));
            }
        }

        // left
        let mut left_starts = Vec::new();
        for r in 0..*rows {
            let old_value = reach_grid[r][0];
            if old_value > -1 {
                let nc = *cols - 1;
                left_starts.push(((r as i128, nc as i128), old_value + 1));
            }
        }
        let left_grid = fill_grid(left_starts, grid, *rows, *cols, req_steps);
        if let Some(grid) = left_grid {
            let left_pos = (
                grid_pos.0,
                grid_pos.1 - 1,
                grid[0][0] % 2,
                grid[0][*cols - 1] % 2,
                grid[*rows - 1][0] % 2,
                grid[*rows - 1][*cols - 1] % 2,
            );
            if !seen_grids.contains(&left_pos) {
                seen_grids.insert(left_pos);
                q.push_back((grid, left_pos));
            }
        }
    }

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
        assert_eq!(answer, Answer::Num(16));
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
