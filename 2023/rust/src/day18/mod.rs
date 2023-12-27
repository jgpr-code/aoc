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
    let count = count_interior(&filled);
    Ok(Answer::Num(count as i128))
}

struct Dig {
    hori_lines: Vec<DigLine>,
    vert_lines: Vec<DigLine>,
}

impl Dig {
    fn new(dig_plan: &Vec<DigInstr>) -> Self {
        let mut new_instr = Vec::new();
        for DigInstr {
            dir: _,
            amount: _,
            color,
        } in dig_plan.iter()
        {
            let amount = i128::from_str_radix(&color[2..7], 16).unwrap();
            let dir = match &color[7..8] {
                "0" => 'R',
                "1" => 'D',
                "2" => 'L',
                "3" => 'U',
                _ => panic!("last color digit invalid"),
            };
            // println!("{} {}", dir, amount);
            new_instr.push((dir, amount));
        }

        let mut hori_lines = Vec::new();
        let mut vert_lines = Vec::new();
        let mut mapper = HashMap::new();
        mapper.insert('U', (-1, 0, true));
        mapper.insert('R', (0, 1, false));
        mapper.insert('D', (1, 0, true));
        mapper.insert('L', (0, -1, false));
        let mut pos = (0, 0);
        for (dir, amount) in new_instr.iter() {
            let delta = mapper.get(dir).unwrap();
            let start = pos;
            let end = (pos.0 + delta.0 * amount, pos.1 + delta.1 * amount);
            pos = end;
            if delta.2 {
                vert_lines.push(DigLine { start, end });
            } else {
                hori_lines.push(DigLine { start, end });
            }
        }
        vert_lines.sort_by(|a, b| {
            assert_eq!(a.start.1, a.end.1);
            assert_eq!(b.start.1, b.end.1);
            a.start.1.cmp(&b.start.1)
        });
        Dig {
            hori_lines,
            vert_lines,
        }
    }
    // fn new_old(dig_plan: &Vec<DigInstr>) -> Self {
    //     let new_instr: Vec<(char, i128)> = dig_plan
    //         .iter()
    //         .map(|dp| (dp.dir, dp.amount as i128))
    //         .collect();
    //     let mut hori_lines = Vec::new();
    //     let mut vert_lines = Vec::new();
    //     let mut mapper = HashMap::new();
    //     mapper.insert('U', (-1, 0, true));
    //     mapper.insert('R', (0, 1, false));
    //     mapper.insert('D', (1, 0, true));
    //     mapper.insert('L', (0, -1, false));
    //     let mut pos = (0, 0);
    //     for (dir, amount) in new_instr.iter() {
    //         let delta = mapper.get(dir).unwrap();
    //         let start = pos;
    //         let end = (pos.0 + delta.0 * amount, pos.1 + delta.1 * amount);
    //         pos = end;
    //         if delta.2 {
    //             vert_lines.push(DigLine { start, end });
    //         } else {
    //             hori_lines.push(DigLine { start, end });
    //         }
    //     }
    //     vert_lines.sort_by(|a, b| {
    //         assert_eq!(a.start.1, a.end.1);
    //         assert_eq!(b.start.1, b.end.1);
    //         a.start.1.cmp(&b.start.1)
    //     });
    //     Dig {
    //         hori_lines,
    //         vert_lines,
    //     }
    // }
    fn is_between(r: i128, s: i128, e: i128) -> bool {
        let mut s = s;
        let mut e = e;
        if e < s {
            std::mem::swap(&mut s, &mut e);
        }
        s <= r && r <= e
    }
    fn count_interior(&self) -> i128 {
        let mut min_r = i128::MAX;
        let mut min_c = i128::MAX;
        let mut max_r = i128::MIN;
        let mut max_c = i128::MIN;
        let mut horimap: HashMap<i128, Vec<(i128, i128)>> = HashMap::new();
        for DigLine { start, end } in self.hori_lines.iter() {
            let (r, c) = start;
            min_r = std::cmp::min(min_r, *r);
            min_c = std::cmp::min(min_c, *c);
            max_r = std::cmp::max(max_r, *r);
            max_c = std::cmp::max(max_c, *c);
            let mut sc = start.1;
            let mut ec = end.1;
            if ec < sc {
                std::mem::swap(&mut sc, &mut ec);
            }
            horimap
                .entry(*r)
                .and_modify(|e| e.push((sc, ec)))
                .or_insert(vec![(sc, ec)]);
        }
        for DigLine { start, end: _ } in self.vert_lines.iter() {
            let (r, c) = start;
            min_r = std::cmp::min(min_r, *r);
            min_c = std::cmp::min(min_c, *c);
            max_r = std::cmp::max(max_r, *r);
            max_c = std::cmp::max(max_c, *c);
        }
        for hl in self.hori_lines.iter() {
            println!("hori {:?} -> {:?}", hl.start, hl.end);
        }
        for vl in self.vert_lines.iter() {
            println!("vert {:?} -> {:?}", vl.start, vl.end);
        }
        let mut total = 0;
        for r in min_r..=max_r {
            // println!("--- row {} ---", r);
            let mut inner = 0;

            // // line on row => just add fully
            // for hl in self.hori_lines.iter() {
            //     if hl.start.0 == r {
            //         inner += i128::abs(hl.start.1 - hl.end.1) + 1;
            //     }
            // }

            // #######
            // #     #
            // # ### #
            // # # # #
            // ### ###
            // line intersects row => add delta to last intersection => lines have to be sorted by column ascending
            let intersecting_lines = self
                .vert_lines
                .iter()
                .filter(|l| Self::is_between(r, l.start.0, l.end.0))
                .collect::<Vec<_>>();
            // assert!(intersecting_lines.len() % 2 == 0);

            let mut hlines = vec![];
            if let Some(l) = horimap.get(&r) {
                hlines = l.clone();
            }
            // println!("hlines {:?}", hlines);
            let mut last_inside = false;
            let mut last_added = false;
            for a in intersecting_lines.windows(2) {
                let left = a[0];
                let mut ls = left.start.0;
                let mut le = left.end.0;
                if le < ls {
                    std::mem::swap(&mut ls, &mut le);
                }
                assert_eq!(left.start.1, left.end.1);
                let lc = left.start.1;
                let right = a[1];
                let mut rs = right.start.0;
                let mut re = right.end.0;
                if re < rs {
                    std::mem::swap(&mut rs, &mut re);
                }
                assert_eq!(right.start.1, right.end.1);
                let rc = right.start.1;
                assert!(lc < rc);
                // println!(
                //     "left ({}, {}), {}; right ({}, {}), {};",
                //     ls, le, lc, rs, re, rc
                // );
                if hlines.contains(&(lc, rc)) {
                    // println!("line contained");
                    inner += rc - lc + 1;
                    if last_added {
                        inner -= 1;
                    }
                    last_added = true;
                    if ls == re || le == rs {
                        // crossing
                        // println!("crossing");
                        //last_inside = !last_inside
                    } else if ls == rs || le == re {
                        // bend
                        // println!("bend");
                        last_inside = !last_inside;
                    } else {
                        panic!("line connects weirdly");
                    }
                } else {
                    if !last_inside {
                        inner += rc - lc + 1;
                        if last_added {
                            inner -= 1;
                        }
                        last_added = true;
                    } else {
                        last_added = false;
                    }
                    last_inside = !last_inside;
                }
            }
            // println!("--- +{} ---", inner);
            total += inner;
        }
        total
    }
}
struct DigLine {
    start: (i128, i128),
    end: (i128, i128),
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { dig_plan } = input;
    let dig = Dig::new(dig_plan);
    // let dig = Dig::new_old(dig_plan);
    let count = dig.count_interior();
    Ok(Answer::Num(count))
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
        assert_eq!(answer, Answer::Num(50746));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(952408144115));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(70086216556038));
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
