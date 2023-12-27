use std::collections::HashMap;

use super::common::*;
use anyhow::Result;
use itertools::Itertools;

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

#[derive(Debug)]
struct Brick {
    start: (i128, i128, i128),
    end: (i128, i128, i128),
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        // sx,sy,sz~ex,ey,ez
        let (s, e) = value.split_once("~").unwrap();
        let start = s
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let end = e
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Brick { start, end }
    }
}

impl Brick {
    fn xy_cover(&self) -> Vec<(i128, i128)> {
        let mut cover = Vec::new();
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                cover.push((x, y));
            }
        }
        cover
    }
    fn height(&self) -> i128 {
        i128::abs(self.end.2 - self.start.2 + 1)
    }
}

struct Input {
    bricks: Vec<Brick>,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut bricks: Vec<Brick> = input.lines().map(|l| Brick::from(l)).collect();
    bricks.sort_by(|a, b| a.start.2.cmp(&b.start.2));
    Ok(Input { bricks })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { bricks } = input;
    let mut plays: HashMap<(i128, i128), (i128, usize)> = HashMap::new();
    let mut supporters = vec![0_usize; bricks.len()];
    let mut supports: Vec<Vec<usize>> = vec![Vec::new(); bricks.len()];
    for (i, brick) in bricks.iter().enumerate() {
        let xy_cover = brick.xy_cover();
        let mut max_height = 0;
        for xy in xy_cover.iter() {
            if let Some((height, _)) = plays.get(xy) {
                max_height = std::cmp::max(max_height, *height);
            }
        }
        for xy in xy_cover.iter() {
            if let Some(&(height, block_idx)) = plays.get(xy) {
                if height == max_height {
                    if !supports[block_idx].contains(&i) {
                        supports[block_idx].push(i);
                        supporters[i] += 1;
                    }
                }
            }
            plays.insert(*xy, (max_height + brick.height(), i));
        }
    }
    let mut safe_disintegrate = 0;
    for i in 0..bricks.len() {
        if supports[i]
            .iter()
            .all(|supportee| supporters[*supportee] > 1)
        {
            // println!("{} is safe", i);
            safe_disintegrate += 1;
        }
    }
    // 517 was wrong (too high)
    Ok(Answer::Num(safe_disintegrate))
}

fn solve_two(input: &Input) -> Result<Answer> {
    todo!()
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
        assert_eq!(answer, Answer::Num(5));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(509));
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
