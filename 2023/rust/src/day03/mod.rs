use super::common::*;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

#[derive(Debug)]
struct Input {
    lines: Vec<String>,
    numbers: Vec<Vec<Number>>,
    gears: Vec<(usize, usize)>,
}

#[derive(Debug)]
struct Number {
    val: i128,
    start: usize,
    end: usize,
}

impl Input {
    fn sum_adjacent(&self) -> i128 {
        let grid = self
            .lines
            .iter()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<_>>>();
        let dx = vec![-1, 0, 1, -1, 1, -1, 0, 1];
        let dy = vec![-1, -1, -1, 0, 0, 1, 1, 1];

        let mut sum = 0;
        for (y, numbers) in self.numbers.iter().enumerate() {
            for num in numbers {
                'number_loop: for x in num.start..num.end {
                    for i in 0..8 {
                        let cx = x as i128 + dx[i];
                        let cy = y as i128 + dy[i];
                        if cx < 0
                            || cy < 0
                            || cx >= grid[0].len() as i128
                            || cy >= grid.len() as i128
                        {
                            continue;
                        }
                        let c = grid[cy as usize][cx as usize];
                        if c != '.' && !c.is_ascii_digit() {
                            // println!("{:?}", num.val);
                            sum += num.val;
                            break 'number_loop;
                        }
                    }
                }
            }
        }
        sum
    }

    fn compute_gears(&self) -> i128 {
        let mut gear_map: HashMap<(usize, usize), Vec<i128>> = HashMap::new();
        for gear in self.gears.iter() {
            gear_map.insert(*gear, Vec::new());
        }
        let grid = self
            .lines
            .iter()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<_>>>();
        let dx = vec![-1, 0, 1, -1, 1, -1, 0, 1];
        let dy = vec![-1, -1, -1, 0, 0, 1, 1, 1];

        for (y, numbers) in self.numbers.iter().enumerate() {
            for num in numbers {
                let mut added_to: HashSet<(usize, usize)> = HashSet::new();
                for x in num.start..num.end {
                    for i in 0..8 {
                        let cx = x as i128 + dx[i];
                        let cy = y as i128 + dy[i];
                        if cx < 0
                            || cy < 0
                            || cx >= grid[0].len() as i128
                            || cy >= grid.len() as i128
                        {
                            continue;
                        }
                        let uy = cy as usize;
                        let ux = cx as usize;
                        let c = grid[uy][ux];
                        let key = (uy, ux);
                        if c == '*' && !added_to.contains(&key) {
                            // println!("{:?}", num.val);
                            let a = gear_map.get_mut(&key).unwrap();
                            a.push(num.val);
                            added_to.insert(key);
                        }
                    }
                }
            }
        }
        gear_map
            .iter()
            .map(|(_, nums)| {
                // assert_eq!(nums.len(), 2);
                if nums.len() != 2 {
                    0
                } else {
                    let mut prod = 1;
                    for n in nums.iter() {
                        prod *= n;
                    }
                    prod
                }
            })
            .sum()
    }
}

fn parse_input(input: &str) -> Result<Input> {
    // let grid = input
    //     .lines()
    //     .map(|l| l.chars().collect::<Vec<_>>())
    //     .collect::<Vec<_>>();
    let lines: Vec<String> = input.lines().map(|s| String::from(s)).collect();

    let number_regex = regex!(r"\d+");

    let mut numbers = Vec::new(); // TODO with_capacity?
    let mut gears = Vec::new();
    for (line_number, line) in lines.iter().enumerate() {
        let numbers_in_line: Vec<Number> = number_regex
            .find_iter(line)
            .map(|m| Number {
                val: m
                    .as_str()
                    .parse::<i128>()
                    .expect("parse i128 was not possible with found regex"),
                start: m.start(),
                end: m.end(),
            })
            .collect();
        numbers.push(numbers_in_line);
        let mut g: Vec<(usize, usize)> = line
            .match_indices("*")
            .map(|(a, _)| (line_number, a))
            .collect();
        gears.append(&mut g);
    }

    Ok(Input {
        lines,
        numbers,
        gears,
    })
}

fn solve_one(input: &Input) -> Result<Answer> {
    //println!("{:?}", input);
    Ok(Answer::Num(input.sum_adjacent()))
}

fn solve_two(input: &Input) -> Result<Answer> {
    Ok(Answer::Num(input.compute_gears()))
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
        assert_eq!(answer, Answer::Num(4361));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(557705));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(467835));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(84266818));
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
