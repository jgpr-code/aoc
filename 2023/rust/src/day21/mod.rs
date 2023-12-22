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

    print_grid(&reach_grid);

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

fn to_inside(r: i32, c: i32, rows: usize, cols: usize) -> (i32, i32) {
    let mut nr = r;
    let mut nc = c;
    let irows = rows as i32;
    let icols = cols as i32;
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

fn count_grid(grid: &Vec<Vec<i32>>, req_steps: i32) -> (i128, i128) {
    let mut even = 0;
    let mut odd = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] >= 0 && grid[r][c] <= req_steps {
                if grid[r][c] % 2 == 0 {
                    even += 1;
                } else {
                    odd += 1;
                }
            }
        }
    }
    (even, odd)
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

fn print_grid(grid: &Vec<Vec<i32>>) {
    println!();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == -1 {
                print!("{}", format!("{:>3} ", '#'));
            } else {
                print!("{:>3} ", grid[r][c]);
            }
        }
        println!();
    }
}
fn printit(grid: &Vec<Vec<i32>>) {
    println!();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == -1 {
                print!("#");
            } else if grid[r][c] % 2 == 1 {
                print!("O");
            } else {
                print!(".");
            }
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

    // 618259911308724
    // 618259911308724

    // let req_steps = 26501365;
    // 26501365
    let req_steps: i32 = 26501365; //365;

    let initial = fill_grid(vec![(*start, 0)], grid, *rows, *cols, req_steps).unwrap();
    print_grid(&initial);
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
    println!("even corners {}", even_corners);
    println!("odd corners {}", odd_corners);
    println!("even {}", even_full);
    println!("odd {}", odd_full);
    // let even_corners = visited
    //     .values()
    //     .filter(|v| **v % 2 == 0 && **v > 65)
    //     .count();
    // let odd_corners = visited
    //     .values()
    //     .filter(|v| **v % 2 == 1 && **v > 65)
    //     .count();

    // let even_full = visited.values().filter(|v| **v % 2 == 0).count();
    // let odd_full = visited.values().filter(|v| **v % 2 == 1).count();
    let n = 202300;
    assert_eq!(n, 202300);
    let a = ((n + 1) * (n + 1)) * odd_full;
    let b = (n * n) * even_full;
    let c = (n + 1) * odd_corners;
    let d = n * even_corners;
    let e = a - c;
    let ans = e + b + d;
    println!("{}", ans); // 618261433219147 is right
    let p2 = ((n + 1) * (n * 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners
        + n * even_corners;
    println!("{}", p2);

    // TODO
    // coming from top,right,bot,left calc delta to top,right,bot,left and max
    // speed up the process by a lot

    // look at input!
    // closest entrance for (r, c) is either  ..S..|.....|.
    // + 66 + (r-1*131) if r or c == 0
    // or +66 +66 + (r-1*131)
    // if r < 0 start in bottom row if r > 0 start in top row
    // if c < 0 start on right if c > 0 start on left
    // +66 + (r-1)*131 +66 + (c-1)*131

    let manysteps = 10000;
    let lcol = *cols as i32 - 1;
    let lrow = *rows as i32 - 1;

    let ftopleft = fill_grid(vec![((0, 0), 0)], grid, *rows, *cols, manysteps).unwrap();
    let ctopleft = count_grid(&ftopleft, manysteps);

    let ftopmid = fill_grid(vec![((0, start.1), 0)], grid, *rows, *cols, manysteps).unwrap();
    let ctopmid = count_grid(&ftopmid, manysteps);

    let ftopright = fill_grid(vec![((0, lcol), 0)], grid, *rows, *cols, manysteps).unwrap();
    let ctopright = count_grid(&ftopright, manysteps);

    let fmidleft = fill_grid(vec![((start.0, 0), 0)], grid, *rows, *cols, manysteps).unwrap();
    let cmidleft = count_grid(&fmidleft, manysteps);

    let fmidright = fill_grid(vec![((start.0, lcol), 0)], grid, *rows, *cols, manysteps).unwrap();
    let cmidright = count_grid(&fmidright, manysteps);

    let fbotleft = fill_grid(vec![((lrow, 0), 0)], grid, *rows, *cols, manysteps).unwrap();
    let cbotleft = count_grid(&fbotleft, manysteps);

    let fbotmid = fill_grid(vec![((lrow, start.1), 0)], grid, *rows, *cols, manysteps).unwrap();
    let cbotmid = count_grid(&fbotmid, manysteps);

    let fbotright = fill_grid(vec![((lrow, lcol), 0)], grid, *rows, *cols, manysteps).unwrap();
    let cbotright = count_grid(&fbotright, manysteps);

    let mut sum = 0;
    for r in -202301..=202301 {
        for c in -202301..=202301 {
            if r == 0 && c == 0 {
                let initial = fill_grid(vec![(*start, 0)], grid, *rows, *cols, req_steps).unwrap();
                printit(&initial);
                sum += count_grid(&initial, req_steps).1;
                continue;
            }
            if r == 0 {
                let entrance_steps = 66 + (i32::abs(c) - 1) * 131;
                if entrance_steps > req_steps {
                    continue;
                }
                let be_safe = entrance_steps + 500 >= req_steps;
                let use_even = entrance_steps % 2 == req_steps % 2;
                if c < 0 {
                    if be_safe {
                        let fillgrid = fill_grid(
                            vec![((start.0, lcol), entrance_steps)],
                            grid,
                            *rows,
                            *cols,
                            req_steps,
                        )
                        .unwrap();
                        let count = count_grid(&fillgrid, req_steps);
                        if use_even {
                            sum += count.0;
                        } else {
                            sum += count.1;
                        }
                    } else if use_even {
                        sum += cmidright.0;
                    } else {
                        sum += cmidright.1;
                    }
                } else {
                    if be_safe {
                        let fillgrid = fill_grid(
                            vec![((start.0, 0), entrance_steps)],
                            grid,
                            *rows,
                            *cols,
                            req_steps,
                        )
                        .unwrap();
                        let count = count_grid(&fillgrid, req_steps);
                        if use_even {
                            sum += count.0;
                        } else {
                            sum += count.1;
                        }
                    } else if use_even {
                        sum += cmidleft.0;
                    } else {
                        sum += cmidleft.1;
                    }
                }
                continue;
            }
            if c == 0 {
                let entrance_steps = 66 + (i32::abs(r) - 1) * 131;
                if entrance_steps > req_steps {
                    continue;
                }
                let be_safe = entrance_steps + 500 >= req_steps;
                let use_even = entrance_steps % 2 == req_steps % 2;
                if r < 0 {
                    if be_safe {
                        let fillgrid = fill_grid(
                            vec![((lrow, start.1), entrance_steps)],
                            grid,
                            *rows,
                            *cols,
                            req_steps,
                        )
                        .unwrap();
                        let count = count_grid(&fillgrid, req_steps);
                        if use_even {
                            sum += count.0;
                        } else {
                            sum += count.1;
                        }
                    } else if use_even {
                        sum += cbotmid.0;
                    } else {
                        sum += cbotmid.1;
                    }
                } else {
                    if be_safe {
                        let fillgrid = fill_grid(
                            vec![((0, start.1), entrance_steps)],
                            grid,
                            *rows,
                            *cols,
                            req_steps,
                        )
                        .unwrap();
                        let count = count_grid(&fillgrid, req_steps);
                        if use_even {
                            sum += count.0;
                        } else {
                            sum += count.1;
                        }
                    } else if use_even {
                        sum += ctopmid.0;
                    } else {
                        sum += ctopmid.1;
                    }
                }
                continue;
            }

            let entrance_steps = 132 + (i32::abs(r) - 1) * 131 + (i32::abs(c) - 1) * 131;
            if entrance_steps > req_steps {
                continue;
            }
            let be_safe = entrance_steps + 500 >= req_steps;
            let use_even = entrance_steps % 2 == req_steps % 2;
            match (r < 0, c < 0) {
                (false, false) => {
                    // topleft
                    if be_safe {
                        let fillgrid = fill_grid(
                            vec![((0, 0), entrance_steps)],
                            grid,
                            *rows,
                            *cols,
                            req_steps,
                        )
                        .unwrap();
                        let count = count_grid(&fillgrid, req_steps);
                        if use_even {
                            sum += count.0;
                        } else {
                            sum += count.1;
                        }
                    } else if use_even {
                        sum += ctopleft.0;
                    } else {
                        sum += ctopleft.1;
                    }
                }
                (false, true) => {
                    // topright
                    if be_safe {
                        let fillgrid = fill_grid(
                            vec![((0, lcol), entrance_steps)],
                            grid,
                            *rows,
                            *cols,
                            req_steps,
                        )
                        .unwrap();
                        let count = count_grid(&fillgrid, req_steps);
                        if use_even {
                            sum += count.0;
                        } else {
                            sum += count.1;
                        }
                    } else if use_even {
                        sum += ctopright.0;
                    } else {
                        sum += ctopright.1;
                    }
                }
                (true, false) => {
                    // botleft
                    if be_safe {
                        let fillgrid = fill_grid(
                            vec![((lrow, 0), entrance_steps)],
                            grid,
                            *rows,
                            *cols,
                            req_steps,
                        )
                        .unwrap();
                        let count = count_grid(&fillgrid, req_steps);
                        if use_even {
                            sum += count.0;
                        } else {
                            sum += count.1;
                        }
                    } else if use_even {
                        sum += cbotleft.0;
                    } else {
                        sum += cbotleft.1;
                    }
                }
                (true, true) => {
                    // botright
                    if be_safe {
                        let fillgrid = fill_grid(
                            vec![((lrow, lcol), entrance_steps)],
                            grid,
                            *rows,
                            *cols,
                            req_steps,
                        )
                        .unwrap();
                        let count = count_grid(&fillgrid, req_steps);
                        if use_even {
                            sum += count.0;
                        } else {
                            sum += count.1;
                        }
                    } else if use_even {
                        sum += cbotright.0;
                    } else {
                        sum += cbotright.1;
                    }
                }
            }
        }
    }

    // 618261350479093 is also wrong...
    // 618261433219147 is right
    // 618259911308724 is wrong surprisingly
    // 54387012847523 still wrong :(
    // 54387012843743 still too low
    // 27193506427523 still wrong...
    // 27193506423743 is wrong (still too low)
    // 21220183577171 is wrong!
    Ok(Answer::Num(sum))
    // let mut seen_grids = HashSet::new();
    // let mut sum = 0;
    // let reach_grid = fill_grid(vec![(*start, 0)], grid, *rows, *cols, req_steps);
    // let mut q = VecDeque::new();
    // if let Some(grid) = reach_grid {
    //     let pos = (
    //         0,
    //         0,
    //         grid[0][0] % 2,
    //         grid[0][*cols - 1] % 2,
    //         grid[*rows - 1][0] % 2,
    //         grid[*rows - 1][*cols - 1] % 2,
    //     );
    //     q.push_back((grid, pos));
    //     seen_grids.insert(pos);
    // }
    // // let mut seen = 0;
    // while let Some((reach_grid, grid_pos)) = q.pop_front() {
    //     sum += count_grid(&reach_grid, req_steps);

    //     // print_grid(&reach_grid);
    //     // seen += 1;
    //     // if seen == 10000 {
    //     //     break;
    //     // }

    //     // top
    //     let mut top_starts = Vec::new();
    //     for c in 0..*cols {
    //         let old_value = reach_grid[0][c];
    //         if old_value > -1 {
    //             let nr = *rows - 1;
    //             top_starts.push(((nr as i32, c as i32), old_value + 1));
    //         }
    //     }
    //     let top_grid = fill_grid(top_starts, grid, *rows, *cols, req_steps);
    //     if let Some(grid) = top_grid {
    //         let top_pos = (
    //             grid_pos.0 - 1,
    //             grid_pos.1,
    //             grid[0][0] % 2,
    //             grid[0][*cols - 1] % 2,
    //             grid[*rows - 1][0] % 2,
    //             grid[*rows - 1][*cols - 1] % 2,
    //         );
    //         if !seen_grids.contains(&top_pos) {
    //             seen_grids.insert(top_pos);
    //             q.push_back((grid, top_pos));
    //         }
    //     }

    //     // right
    //     let mut right_starts = Vec::new();
    //     for r in 0..*rows {
    //         let old_value = reach_grid[r][*cols - 1];
    //         if old_value > -1 {
    //             let nc = 0;
    //             right_starts.push(((r as i32, nc as i32), old_value + 1));
    //         }
    //     }
    //     let right_grid = fill_grid(right_starts, grid, *rows, *cols, req_steps);
    //     if let Some(grid) = right_grid {
    //         let right_pos = (
    //             grid_pos.0,
    //             grid_pos.1 + 1,
    //             grid[0][0] % 2,
    //             grid[0][*cols - 1] % 2,
    //             grid[*rows - 1][0] % 2,
    //             grid[*rows - 1][*cols - 1] % 2,
    //         );
    //         if !seen_grids.contains(&right_pos) {
    //             seen_grids.insert(right_pos);
    //             q.push_back((grid, right_pos));
    //         }
    //     }

    //     // bottom
    //     let mut bottom_starts = Vec::new();
    //     for c in 0..*cols {
    //         let old_value = reach_grid[*rows - 1][c];
    //         if old_value > -1 {
    //             let nr = 0;
    //             bottom_starts.push(((nr as i32, c as i32), old_value + 1));
    //         }
    //     }
    //     let bottom_grid = fill_grid(bottom_starts, grid, *rows, *cols, req_steps);
    //     if let Some(grid) = bottom_grid {
    //         let bottom_pos = (
    //             grid_pos.0 + 1,
    //             grid_pos.1,
    //             grid[0][0] % 2,
    //             grid[0][*cols - 1] % 2,
    //             grid[*rows - 1][0] % 2,
    //             grid[*rows - 1][*cols - 1] % 2,
    //         );
    //         if !seen_grids.contains(&bottom_pos) {
    //             seen_grids.insert(bottom_pos);
    //             q.push_back((grid, bottom_pos));
    //         }
    //     }

    //     // left
    //     let mut left_starts = Vec::new();
    //     for r in 0..*rows {
    //         let old_value = reach_grid[r][0];
    //         if old_value > -1 {
    //             let nc = *cols - 1;
    //             left_starts.push(((r as i32, nc as i32), old_value + 1));
    //         }
    //     }
    //     let left_grid = fill_grid(left_starts, grid, *rows, *cols, req_steps);
    //     if let Some(grid) = left_grid {
    //         let left_pos = (
    //             grid_pos.0,
    //             grid_pos.1 - 1,
    //             grid[0][0] % 2,
    //             grid[0][*cols - 1] % 2,
    //             grid[*rows - 1][0] % 2,
    //             grid[*rows - 1][*cols - 1] % 2,
    //         );
    //         if !seen_grids.contains(&left_pos) {
    //             seen_grids.insert(left_pos);
    //             q.push_back((grid, left_pos));
    //         }
    //     }
    // }

    // Ok(Answer::Num(sum))
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
        assert_eq!(answer, Answer::Num(3743));
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
