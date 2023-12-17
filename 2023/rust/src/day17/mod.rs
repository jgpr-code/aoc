use std::collections::VecDeque;

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
    grid: Vec<Vec<i32>>,
}

fn parse_input(input: &str) -> Result<Input> {
    let grid = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();
    Ok(Input { grid })
}

fn inside(r: i32, c: i32, rows: usize, cols: usize) -> bool {
    r >= 0 && c >= 0 && r < rows as i32 && c < cols as i32
}

fn turn_left(o: usize) -> usize {
    (o + 4 - 1) % 4
}
fn turn_right(o: usize) -> usize {
    (o + 1) % 4
}
fn add_ori(r: usize, c: usize, o: usize, rows: usize, cols: usize) -> Option<(usize, usize)> {
    // n0 e1 s2 w3
    let dr = vec![-1, 0, 1, 0];
    let dc = vec![0, 1, 0, -1];
    let nr = r as i32 + dr[o];
    let nc = c as i32 + dc[o];
    if inside(nr, nc, rows, cols) {
        Some((nr as usize, nc as usize))
    } else {
        None
    }
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { grid } = input;

    let rows = grid.len();
    let cols = grid[0].len();
    // row r
    // col c
    // crucible orientation o
    // orientation move count n
    // [r][c][o][n]
    let mut best = vec![vec![vec![vec![i32::MAX; 4]; 4]; cols]; rows];
    let mut q = VecDeque::new();
    // start does no heat loss
    q.push_back((0, 1, 1, 1, grid[0][1]));
    best[0][1][1][1] = grid[0][1];
    q.push_back((1, 0, 2, 1, grid[1][0]));
    best[1][0][2][1] = grid[1][0];
    while let Some((r, c, o, n, v)) = q.pop_front() {
        if n < 3 {
            // straight s
            if let Some((sr, sc)) = add_ori(r, c, o, rows, cols) {
                let new_v = v + grid[sr][sc];
                if best[sr][sc][o][n + 1] > new_v {
                    best[sr][sc][o][n + 1] = new_v;
                    q.push_back((sr, sc, o, n + 1, new_v));
                }
            }
        }
        let lo = turn_left(o);
        if let Some((lr, lc)) = add_ori(r, c, lo, rows, cols) {
            let new_v = v + grid[lr][lc];
            if best[lr][lc][lo][1] > new_v {
                best[lr][lc][lo][1] = new_v;
                q.push_back((lr, lc, lo, 1, new_v));
            }
        }
        let ro = turn_right(o);
        if let Some((rr, rc)) = add_ori(r, c, ro, rows, cols) {
            let new_v = v + grid[rr][rc];
            if best[rr][rc][ro][1] > new_v {
                best[rr][rc][ro][1] = new_v;
                q.push_back((rr, rc, ro, 1, new_v));
            }
        }
    }
    let mut ans = i32::MAX;
    for o in 0..4 {
        for n in 1..4 {
            ans = std::cmp::min(ans, best[rows - 1][cols - 1][o][n]);
        }
    }
    Ok(Answer::Num(ans as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { grid } = input;

    let rows = grid.len();
    let cols = grid[0].len();
    // row r
    // col c
    // crucible orientation o
    // orientation move count n
    // [r][c][o][n]
    let mut best = vec![vec![vec![vec![i32::MAX; 11]; 4]; cols]; rows];
    let mut q = VecDeque::new();
    // start does no heat loss
    q.push_back((0, 1, 1, 1, grid[0][1]));
    best[0][1][1][1] = grid[0][1];
    q.push_back((1, 0, 2, 1, grid[1][0]));
    best[1][0][2][1] = grid[1][0];
    while let Some((r, c, o, n, v)) = q.pop_front() {
        if n < 10 {
            // straight s
            if let Some((sr, sc)) = add_ori(r, c, o, rows, cols) {
                let new_v = v + grid[sr][sc];
                if best[sr][sc][o][n + 1] > new_v {
                    best[sr][sc][o][n + 1] = new_v;
                    q.push_back((sr, sc, o, n + 1, new_v));
                }
            }
        }
        if n >= 4 {
            let lo = turn_left(o);
            if let Some((lr, lc)) = add_ori(r, c, lo, rows, cols) {
                let new_v = v + grid[lr][lc];
                if best[lr][lc][lo][1] > new_v {
                    best[lr][lc][lo][1] = new_v;
                    q.push_back((lr, lc, lo, 1, new_v));
                }
            }
            let ro = turn_right(o);
            if let Some((rr, rc)) = add_ori(r, c, ro, rows, cols) {
                let new_v = v + grid[rr][rc];
                if best[rr][rc][ro][1] > new_v {
                    best[rr][rc][ro][1] = new_v;
                    q.push_back((rr, rc, ro, 1, new_v));
                }
            }
        }
    }
    let mut ans = i32::MAX;
    for o in 0..4 {
        for n in 1..11 {
            ans = std::cmp::min(ans, best[rows - 1][cols - 1][o][n]);
        }
    }
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
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(102));
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
        assert_eq!(answer, Answer::Num(94));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }

    #[bench]
    fn bench_one(b: &mut Bencher) {
        b.iter(|| part_one())
    }
    #[bench]
    fn bench_two(b: &mut Bencher) {
        b.iter(|| part_two())
    }
}
