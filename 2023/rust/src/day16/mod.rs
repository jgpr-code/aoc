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
    grid: Vec<Vec<char>>,
}

fn parse_input(input: &str) -> Result<Input> {
    let grid = input.lines().map(|l| l.chars().collect()).collect();
    Ok(Input { grid })
}

fn inside(r: i32, c: i32, rows: usize, cols: usize) -> bool {
    r >= 0 && r < rows as i32 && c >= 0 && c < cols as i32
}

fn energize_from(sr: i32, sc: i32, sd: usize, grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();
    // n e s w
    let mut vis = vec![vec![vec![false; 4]; cols]; rows];
    let s = (sr, sc, sd);
    let mut q = VecDeque::new();
    let mut energized = vec![vec!['.'; cols]; rows];
    q.push_back(s);
    vis[sr as usize][sc as usize][sd] = true;
    while let Some((r, c, d)) = q.pop_front() {
        let dr = vec![-1, 0, 1, 0];
        let dc = vec![0, 1, 0, -1];
        let mut np = vec![None; 4];
        for i in 0..4 {
            let nr = r + dr[i];
            let nc = c + dc[i];
            if inside(nr, nc, rows, cols) {
                np[i] = Some((nr, nc));
            }
        }
        match grid[r as usize][c as usize] {
            '|' if d == 1 || d == 3 => {
                if let Some((nr, nc)) = np[0] {
                    if !vis[nr as usize][nc as usize][0] {
                        q.push_back((nr, nc, 0));
                        vis[nr as usize][nc as usize][0] = true;
                    }
                }
                if let Some((nr, nc)) = np[2] {
                    if !vis[nr as usize][nc as usize][2] {
                        q.push_back((nr, nc, 2));
                        vis[nr as usize][nc as usize][2] = true;
                    }
                }
            }
            '-' if d == 0 || d == 2 => {
                if let Some((nr, nc)) = np[3] {
                    if !vis[nr as usize][nc as usize][3] {
                        q.push_back((nr, nc, 3));
                        vis[nr as usize][nc as usize][3] = true;
                    }
                }
                if let Some((nr, nc)) = np[1] {
                    if !vis[nr as usize][nc as usize][1] {
                        q.push_back((nr, nc, 1));
                        vis[nr as usize][nc as usize][1] = true;
                    }
                }
            }
            '/' => {
                // n0 -> e1; e1 -> n0; s2 -> w3; w3 -> s2;
                match d {
                    0 => {
                        if let Some((nr, nc)) = np[1] {
                            if !vis[nr as usize][nc as usize][1] {
                                q.push_back((nr, nc, 1));
                                vis[nr as usize][nc as usize][1] = true;
                            }
                        }
                    }
                    1 => {
                        if let Some((nr, nc)) = np[0] {
                            if !vis[nr as usize][nc as usize][0] {
                                q.push_back((nr, nc, 0));
                                vis[nr as usize][nc as usize][0] = true;
                            }
                        }
                    }
                    2 => {
                        if let Some((nr, nc)) = np[3] {
                            if !vis[nr as usize][nc as usize][3] {
                                q.push_back((nr, nc, 3));
                                vis[nr as usize][nc as usize][3] = true;
                            }
                        }
                    }
                    3 => {
                        if let Some((nr, nc)) = np[2] {
                            if !vis[nr as usize][nc as usize][2] {
                                q.push_back((nr, nc, 2));
                                vis[nr as usize][nc as usize][2] = true;
                            }
                        }
                    }
                    _ => panic!("unknown dir"),
                }
            }
            '\\' => {
                // n0 -> w3; w3 -> n0; e1 -> s2; s2 -> e1;
                match d {
                    0 => {
                        if let Some((nr, nc)) = np[3] {
                            if !vis[nr as usize][nc as usize][3] {
                                q.push_back((nr, nc, 3));
                                vis[nr as usize][nc as usize][3] = true;
                            }
                        }
                    }
                    1 => {
                        if let Some((nr, nc)) = np[2] {
                            if !vis[nr as usize][nc as usize][2] {
                                q.push_back((nr, nc, 2));
                                vis[nr as usize][nc as usize][2] = true;
                            }
                        }
                    }
                    2 => {
                        if let Some((nr, nc)) = np[1] {
                            if !vis[nr as usize][nc as usize][1] {
                                q.push_back((nr, nc, 1));
                                vis[nr as usize][nc as usize][1] = true;
                            }
                        }
                    }
                    3 => {
                        if let Some((nr, nc)) = np[0] {
                            if !vis[nr as usize][nc as usize][0] {
                                q.push_back((nr, nc, 0));
                                vis[nr as usize][nc as usize][0] = true;
                            }
                        }
                    }
                    _ => panic!("unknown dir"),
                }
            }
            _ => {
                if let Some((nr, nc)) = np[d] {
                    if !vis[nr as usize][nc as usize][d] {
                        q.push_back((nr, nc, d));
                        vis[nr as usize][nc as usize][d] = true;
                    }
                }
            }
        }
        energized[r as usize][c as usize] = '#';
    }
    energized
}

fn count_energy(energized: &Vec<Vec<char>>) -> usize {
    energized
        .iter()
        .map(|v| v.iter().filter(|&c| *c == '#').count())
        .sum()
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { grid } = input;
    let energized = energize_from(0, 0, 1, &grid);
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            print!("{}", energized[r][c]);
        }
        println!();
    }
    let count = count_energy(&energized);
    Ok(Answer::Num(count as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { grid } = input;
    let rows = grid.len();
    let cols = grid[0].len();
    let mut max_energized = 0;
    for c in 0..cols {
        // top => r = 0, d = s2; r = rows-1, d = n0
        let energized = energize_from(0, c as i32, 2, grid);
        max_energized = std::cmp::max(max_energized, count_energy(&energized));
        let energized = energize_from(rows as i32 - 1, c as i32, 0, grid);
        max_energized = std::cmp::max(max_energized, count_energy(&energized));
    }
    for r in 0..rows {
        // left => c = 0, d = e1; c = cols -1, d = w3
        let energized = energize_from(r as i32, 0, 1, grid);
        max_energized = std::cmp::max(max_energized, count_energy(&energized));
        let energized = energize_from(r as i32, cols as i32 - 1, 3, grid);
        max_energized = std::cmp::max(max_energized, count_energy(&energized));
    }

    Ok(Answer::Num(max_energized as i128))
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
        assert_eq!(answer, Answer::Num(46));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(7199));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(51));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(7438));
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
