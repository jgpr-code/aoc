use super::common::*;
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

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
    xsize: i128,
    ysize: i128,
    neighbours: HashMap<(i128, i128), Vec<(i128, i128)>>,
    spos: (i128, i128),
}

impl Input {
    fn farthest_from_start(&self) -> i128 {
        // bfs
        let mut distance = 0;
        let mut visited: HashSet<(i128, i128)> = HashSet::new();
        let mut queue: VecDeque<(i128, (i128, i128))> = VecDeque::new();
        queue.push_back((0, self.spos));
        while let Some((dist, pos)) = queue.pop_front() {
            visited.insert(pos);
            distance = std::cmp::max(distance, dist);
            let neighs = self.neighbours.get(&pos).unwrap();
            for neigh in neighs.iter().filter(|&n| !visited.contains(n)) {
                let bneighs = self.neighbours.get(neigh).unwrap();
                if bneighs.contains(&pos) {
                    queue.push_back((dist + 1, *neigh));
                }
            }
        }
        distance
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let ysize = grid.iter().count() as i128;
    let xsize = grid.first().unwrap().iter().count() as i128;
    let mut spos = (-1, -1);
    let mut neighbours = HashMap::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            let x = x as i128;
            let y = y as i128;
            match &c {
                '|' => {
                    let ret = neighbours.insert((x, y), vec![(x, y - 1), (x, y + 1)]);
                    assert_eq!(ret, None);
                }
                '-' => {
                    let ret = neighbours.insert((x, y), vec![(x - 1, y), (x + 1, y)]);
                    assert_eq!(ret, None);
                }
                'L' => {
                    let ret = neighbours.insert((x, y), vec![(x, y - 1), (x + 1, y)]);
                    assert_eq!(ret, None);
                }
                'J' => {
                    let ret = neighbours.insert((x, y), vec![(x, y - 1), (x - 1, y)]);
                    assert_eq!(ret, None);
                }
                '7' => {
                    let ret = neighbours.insert((x, y), vec![(x, y + 1), (x - 1, y)]);
                    assert_eq!(ret, None);
                }
                'F' => {
                    let ret = neighbours.insert((x, y), vec![(x, y + 1), (x + 1, y)]);
                    assert_eq!(ret, None);
                }
                '.' => {
                    let ret = neighbours.insert((x, y), vec![]);
                    assert_eq!(ret, None);
                }
                'S' => {
                    let ret = neighbours
                        .insert((x, y), vec![(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)]);
                    assert_eq!(ret, None);
                    spos = (x, y);
                }
                _ => unreachable!(),
            }
        }
    }
    for neighs in neighbours.values_mut() {
        neighs.retain(|&(x, y)| x >= 0 && y >= 0 && x < xsize && y < ysize);
    }
    assert_ne!((-1, -1), spos);
    Ok(Input {
        grid,
        xsize,
        ysize,
        neighbours,
        spos,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    Ok(Answer::Num(input.farthest_from_start()))
}

fn solve_two(input: &Input) -> Result<Answer> {
    Ok(Answer::Num(0))
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
        assert_eq!(answer, Answer::Num(8));
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
