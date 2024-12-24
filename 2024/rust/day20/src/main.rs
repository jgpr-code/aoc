#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::{
    point::{point, Point},
    Answer,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
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

#[derive(Clone)]
struct Grid {
    content: HashMap<char, HashSet<Point>>,
    size: Point,
}
impl From<&str> for Grid {
    fn from(grid_str: &str) -> Self {
        let mut content: HashMap<char, HashSet<Point>> = HashMap::new();
        let mut xs = None;
        let mut ys = 0i128;
        for (y, line) in grid_str.lines().enumerate() {
            ys += 1;
            if xs.is_none() {
                xs = Some(line.len() as i128);
            }
            for (x, c) in line.chars().enumerate() {
                content
                    .entry(c)
                    .and_modify(|s| {
                        s.insert(point!(x as i128, y as i128));
                    })
                    .or_insert(HashSet::from([point!(x as i128, y as i128)]));
            }
        }
        Self {
            content,
            size: point!(xs.unwrap_or(0), ys),
        }
    }
}
impl Grid {
    fn get_unique_point(&self, c: char) -> Result<Point> {
        let foo = self
            .content
            .get(&c)
            .ok_or(anyhow!("unique point '{}' must be present", c))?;
        let vec: Vec<&Point> = foo.iter().collect();
        if vec.len() != 1 {
            return Err(anyhow!("unique point '{}' must be unique!", c));
        }
        Ok(*vec[0])
    }
    fn bfs_4_neighbors_shortest_paths(
        &self,
        start: Point,
        end: Point,
        obstacle: char,
    ) -> (Option<usize>, HashMap<Point, usize>) {
        let mut queue = VecDeque::new();
        let mut shortest_paths = HashMap::new();
        queue.push_back((start, 0));
        shortest_paths.insert(start, 0);
        while let Some((p, cost)) = queue.pop_front() {
            if p == end {
                return (Some(cost), shortest_paths);
            }
            let neighbors = p.get_4_neighbors(&self.size);
            for neighbor in neighbors {
                if let Some(obstacles) = self.content.get(&obstacle) {
                    if obstacles.contains(&neighbor) {
                        continue;
                    }
                }
                if shortest_paths.contains_key(&neighbor) {
                    continue;
                }
                queue.push_back((neighbor, cost + 1));
                shortest_paths.insert(neighbor, cost + 1);
            }
        }
        (None, shortest_paths)
    }
}

type Input = Grid;

fn parse_input(input: &str) -> Result<Input> {
    Ok(Grid::from(input))
}

fn cheat_statistics(cheats: &[(i128, (Point, Point))]) -> String {
    let mut cheat_map = BTreeMap::new();
    let mut statistic = String::new();
    for cheat in cheats.iter() {
        cheat_map
            .entry(cheat.0)
            .and_modify(|amount| *amount += 1)
            .or_insert(1);
    }
    for (saving, amount) in cheat_map.iter() {
        let description = if *amount == 1 {
            format!("There is one cheat that saves {} picoseconds\n", saving)
        } else {
            format!(
                "There are {} cheats that save {} picoseconds\n",
                amount, saving
            )
        };
        statistic.push_str(description.as_str());
    }
    statistic
}

fn solve_one(grid: &Input) -> Result<Answer> {
    let start = grid.get_unique_point('S')?;
    let end = grid.get_unique_point('E')?;
    let (_, cost_map) = grid.bfs_4_neighbors_shortest_paths(start, end, '#');
    let obstacles = grid
        .content
        .get(&'#')
        .ok_or(anyhow!("# must be in this grid!"))?;
    let mut cheats = Vec::new();
    for (p, cost) in cost_map.iter() {
        for dir in point::NEIGH4 {
            let cheat1 = *p + dir;
            let cheat2 = *p + (2 * dir);
            if obstacles.contains(&cheat1) && cost_map.contains_key(&cheat2) {
                let cost_after_cheat = cost_map[&cheat2] as i128;
                let cost = *cost as i128;
                let savings = cost_after_cheat - (cost + 2);
                if savings >= 0 {
                    cheats.push((savings, (cheat1, cheat2)));
                }
            }
        }
    }
    // cheats.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{}", cheat_statistics(&cheats));
    Ok(Answer::Num(
        cheats.iter().filter(|c| c.0 >= 100).count() as i128
    ))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day20_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(1530));
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
        assert_eq!(answer, Answer::Num(0));
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
