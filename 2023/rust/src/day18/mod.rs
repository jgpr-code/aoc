use std::collections::{HashMap, VecDeque};

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
    dig_plan: Vec<DigInstr>,
}

struct DigInstr {
    dir: char,
    amount: usize,
    color: String,
}

struct DigGrid {
    grid: HashMap<(i128, i128), String>,
}

impl DigGrid {
    fn new(dig_plan: &Vec<DigInstr>) -> Self {
        let mut grid = HashMap::new();
        let mut pos = (0, 0);
        let mut mapper = HashMap::new();
        mapper.insert('U', (0, -1));
        mapper.insert('R', (1, 0));
        mapper.insert('D', (0, 1));
        mapper.insert('L', (-1, 0));
        for DigInstr { dir, amount, color } in dig_plan.iter() {
            let delta = mapper.get(dir).unwrap();
            let mut countdown = *amount;
            while countdown > 0 {
                pos = (pos.0 + delta.0, pos.1 + delta.1);
                grid.insert(pos, color.clone());
                countdown -= 1;
            }
        }
        DigGrid { grid }
    }
    fn get_filled_grid(&self) -> Vec<Vec<char>> {
        let mut min_x = i128::MAX;
        let mut min_y = i128::MAX;
        let mut max_x = i128::MIN;
        let mut max_y = i128::MIN;
        for (x, y) in self.grid.keys() {
            min_x = std::cmp::min(min_x, *x);
            min_y = std::cmp::min(min_y, *y);
            max_x = std::cmp::max(max_x, *x);
            max_y = std::cmp::max(max_y, *y);
        }
        // for empty corners
        min_x -= 1;
        min_y -= 1;
        max_x += 1;
        max_y += 1;
        let rows = max_y - min_y + 1;
        let cols = max_x - min_x + 1;
        let mut filled = vec![vec!['.'; cols as usize]; rows as usize];
        for (x, y) in self.grid.keys() {
            let x = x - min_x;
            let y = y - min_y;
            filled[y as usize][x as usize] = '#';
        }
        filled
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            print!("{}", grid[r][c]);
        }
        println!();
    }
}
fn count_interior(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut vis = vec![vec![false; cols]; rows];
    let mut q = VecDeque::new();
    let mut outer = 0;
    let s: (i128, i128) = (0, 0);
    q.push_back(s);
    vis[0][0] = true;
    outer += 1;
    let dr = vec![-1, 0, 1, 0];
    let dc = vec![0, 1, 0, -1];
    while let Some(p) = q.pop_front() {
        for i in 0..4 {
            let np = (p.0 + dr[i], p.1 + dc[i]);
            let (r, c) = np;
            if r >= 0
                && c >= 0
                && r < rows as i128
                && c < cols as i128
                && !vis[r as usize][c as usize]
                && grid[r as usize][c as usize] != '#'
            {
                q.push_back((r, c));
                vis[r as usize][c as usize] = true;
                outer += 1;
            }
        }
    }
    rows * cols - outer
}

fn parse_input(input: &str) -> Result<Input> {
    let mut dig_plan = Vec::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        assert_eq!(split.len(), 3);
        let dir = split[0].chars().last().unwrap();
        let amount = split[1].parse().unwrap();
        let color = String::from(split[2]);
        dig_plan.push(DigInstr { dir, amount, color })
    }
    Ok(Input { dig_plan })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { dig_plan } = input;
    let grid = DigGrid::new(dig_plan);
    let filled = grid.get_filled_grid();
    print_grid(&filled);
    let mut count = count_interior(&filled);
    Ok(Answer::Num(count as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    todo!();
    Ok(Answer::Num(-1))
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
        assert_eq!(answer, Answer::Num(62));
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
