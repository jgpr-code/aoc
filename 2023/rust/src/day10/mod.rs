use super::common::*;
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_one(input: &str) -> Result<Answer> {
    let mut input = parse_input(input)?;
    solve_one(&mut input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let mut input = parse_input(input)?;
    solve_two(&mut input)
}

struct Input {
    grid: Vec<Vec<char>>,
    xsize: i128,
    ysize: i128,
    neighbours: HashMap<(i128, i128), Vec<(i128, i128)>>,
    spos: (i128, i128),
    the_pipe: HashSet<(i128, i128)>,
    intersection_count_vertical: HashMap<(i128, i128), i128>,
    intersection_count_horizontal: HashMap<(i128, i128), i128>,
}

impl Input {
    fn farthest_from_start(&mut self) -> i128 {
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
        self.the_pipe = visited;
        distance
    }

    fn replace_s(&mut self) {
        // nesw
        let dx = vec![0, 1, 0, -1];
        let dy = vec![-1, 0, 1, 0];
        let mut connector = vec![];
        for i in 0..4 {
            let (x, y) = self.spos;
            let npos = (x + dx[i], y + dy[i]);
            if let Some(neighs) = self.neighbours.get(&npos) {
                if neighs.contains(&self.spos) {
                    connector.push(i);
                }
            }
        }
        assert_eq!(connector.len(), 2);
        let replace = match (connector[0], connector[1]) {
            (0, 1) => 'L',
            (0, 2) => '|',
            (0, 3) => 'J',
            (1, 2) => 'F',
            (1, 3) => '-',
            (2, 3) => '7',
            _ => unreachable!(),
        };
        self.grid[self.spos.1 as usize][self.spos.0 as usize] = replace;
    }
    fn count_inside(&mut self) -> i128 {
        self.replace_s();
        // horizontal: | or L---7 or F---J count towards intersection
        for y in 0..self.ysize {
            let mut intersection_count = 0;
            let mut started_with = '.';
            for x in 0..self.xsize {
                let c = self.grid[y as usize][x as usize];
                if !self.the_pipe.contains(&(x, y)) {
                    self.intersection_count_horizontal
                        .insert((x, y), intersection_count);
                } else {
                    match c {
                        '|' => intersection_count += 1,
                        'L' => started_with = 'L',
                        '7' if started_with == 'L' => {
                            intersection_count += 1;
                            started_with = '.';
                        }
                        'F' => started_with = 'F',
                        'J' if started_with == 'F' => {
                            intersection_count += 1;
                            started_with = '.';
                        }
                        '-' => {}
                        _ => started_with = '.',
                    }
                }
            }
        }
        // vertical: - or 7 or F
        //                |    |
        //                |    |
        //                |    |
        //                L    J
        for x in 0..self.xsize {
            let mut intersection_count = 0;
            let mut started_with = '.';
            for y in 0..self.ysize {
                let c = self.grid[y as usize][x as usize];
                if !self.the_pipe.contains(&(x, y)) {
                    self.intersection_count_vertical
                        .insert((x, y), intersection_count);
                } else {
                    match c {
                        '-' => intersection_count += 1,
                        '7' => started_with = '7',
                        'L' if started_with == '7' => {
                            intersection_count += 1;
                            started_with = '.';
                        }
                        'F' => started_with = 'F',
                        'J' if started_with == 'F' => {
                            intersection_count += 1;
                            started_with = '.';
                        }
                        '|' => {}
                        _ => started_with = '.',
                    }
                }
            }
        }
        let mut inner_count = 0;
        for (y, line) in self.grid.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                let xi = x as i128;
                let yi = y as i128;
                if let Some(hori) = self.intersection_count_horizontal.get(&(xi, yi)) {
                    let vert = self.intersection_count_vertical.get(&(xi, yi)).unwrap();
                    if hori % 2 != 0 && vert % 2 != 0 {
                        print!("I");
                        inner_count += 1;
                    } else {
                        print!("O");
                    }
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
        inner_count
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
        the_pipe: HashSet::new(),
        intersection_count_vertical: HashMap::new(),
        intersection_count_horizontal: HashMap::new(),
    })
}

fn solve_one(input: &mut Input) -> Result<Answer> {
    Ok(Answer::Num(input.farthest_from_start()))
}

fn solve_two(input: &mut Input) -> Result<Answer> {
    input.farthest_from_start(); // computes the_pipe...
    Ok(Answer::Num(input.count_inside()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test.txt");
    static TEST_TWO_EASY: LazyLock<String> = local_file!("test_two_easy.txt");
    static TEST_TWO_HARD: LazyLock<String> = local_file!("test_two_hard.txt");
    static TEST_TWO_HARDEST: LazyLock<String> = local_file!("test_two_hardest.txt");
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
        assert_eq!(answer, Answer::Num(7145));
        Ok(())
    }
    #[test]
    fn test_two_easy() -> Result<()> {
        let answer = super::part_two(&TEST_TWO_EASY)?;
        assert_eq!(answer, Answer::Num(4));
        Ok(())
    }
    #[test]
    fn test_two_hard() -> Result<()> {
        let answer = super::part_two(&TEST_TWO_HARD)?;
        assert_eq!(answer, Answer::Num(8));
        Ok(())
    }
    #[test]
    fn test_two_hardest() -> Result<()> {
        let answer = super::part_two(&TEST_TWO_HARDEST)?;
        assert_eq!(answer, Answer::Num(10));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(445));
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
