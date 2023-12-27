use std::collections::VecDeque;

use super::common::*;
use anyhow::Result;

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input, 64)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    start: (i32, i32),
}

fn inside(r: i32, c: i32, rows: usize, cols: usize) -> bool {
    r >= 0 && c >= 0 && r < rows as i32 && c < cols as i32
}

fn parse_input(input: &str) -> Result<Input> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut start = (-1, -1);
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                start = (r as i32, c as i32);
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

fn solve_one(input: &Input, req_steps: i32) -> Result<Answer> {
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
    reach_grid[start.0 as usize][start.1 as usize] = 0;
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

    // print_grid(&reach_grid);

    let mut sum = 0;

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

fn fill_grid(
    starts: Vec<((i32, i32), i32)>,
    grid: &Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    req_steps: i32,
) -> Option<Vec<Vec<i32>>> {
    let dr = vec![-1, 0, 1, 0];
    let dc = vec![0, 1, 0, -1];
    let mut reach_grid: Vec<Vec<i32>> = vec![vec![-1; cols]; rows];
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

// fn print_grid(grid: &Vec<Vec<i32>>) {
//     println!();
//     for r in 0..grid.len() {
//         for c in 0..grid[0].len() {
//             if grid[r][c] == -1 {
//                 print!("{}", format!("{:>3} ", '#'));
//             } else {
//                 print!("{:>3} ", grid[r][c]);
//             }
//         }
//         println!();
//     }
// }

fn solve_two(input: &Input) -> Result<Answer> {
    let Input {
        grid,
        rows,
        cols,
        start,
    } = input;

    let req_steps: i32 = 26501365;
    let initial = fill_grid(vec![(*start, 0)], grid, *rows, *cols, req_steps).unwrap();
    // print_grid(&initial);
    let mut even_corners = 0_usize;
    let mut odd_corners = 0_usize;
    let mut even_full = 0_usize;
    let mut odd_full = 0_usize;
    let mut hash_count = 0_usize;
    for r in 0..*rows {
        for c in 0..*cols {
            let v = initial[r][c];
            if v == -1 {
                hash_count += 1;
                continue;
            }
            if v % 2 == 0 {
                even_full += 1;
                if v > 65 {
                    even_corners += 1;
                }
            } else {
                odd_full += 1;
                if v > 65 {
                    odd_corners += 1;
                }
            }
        }
    }
    assert_eq!(hash_count + even_full + odd_full, 131 * 131);
    // println!("even corners {}", even_corners);
    // println!("odd corners {}", odd_corners);
    // println!("even {}", even_full);
    // println!("odd {}", odd_full);
    let n = 202300; // (26501365 - 65) / 131
    let a = ((n + 1) * (n + 1)) * odd_full;
    let b = (n * n) * even_full;
    let c = (n + 1) * odd_corners;
    let d = n * even_corners;
    let e = a - c;
    let ans = e + b + d;
    Ok(Answer::Num(ans as i128))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test.txt");
    static INPUT: LazyLock<String> = local_file!("input.txt");

    #[test]
    fn test_solve_one() -> Result<()> {
        let input = parse_input(&TEST)?;
        let answer = super::solve_one(&input, 6)?;
        assert_eq!(answer, Answer::Num(16));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(3743));
        Ok(())
    }
    // TODO: make the algorithm also work correctly for test.txt
    // In exactly 6 steps, he can still reach 16 garden plots.
    // In exactly 10 steps, he can reach any of 50 garden plots.
    // In exactly 50 steps, he can reach 1594 garden plots.
    // In exactly 100 steps, he can reach 6536 garden plots.
    // In exactly 500 steps, he can reach 167004 garden plots.
    // In exactly 1000 steps, he can reach 668697 garden plots.
    // In exactly 5000 steps, he can reach 16733044 garden plots.
    // #[test]
    // fn test_two() -> Result<()> {
    //     let answer = super::part_two(&TEST)?;
    //     assert_eq!(answer, Answer::Num(-1));
    //     Ok(())
    // }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(618261433219147));
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
