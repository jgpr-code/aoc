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
    grids: Vec<Vec<Vec<char>>>,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut grids = Vec::new();
    for block in input.split("\r\n\r\n") {
        let mut grid = Vec::new();
        for line in block.lines() {
            grid.push(line.chars().collect());
        }
        grids.push(grid);
    }
    Ok(Input { grids })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { grids } = input;
    let mut sum = 0;
    for grid in grids {
        let (href, vref) = find_reflection(grid, false);
        // 0 indexed, but needs 1
        if href != -1 {
            sum += (href + 1) * 100;
        }
        if vref != -1 {
            sum += vref + 1;
        }
    }
    Ok(Answer::Num(sum))
}

// convention is to return the last index of the top
fn find_reflection(grid: &Vec<Vec<char>>, smudge: bool) -> (i128, i128) {
    let mut horizontal = Vec::new();
    for line in grid {
        let lstr = String::from_iter(line.iter());
        horizontal.push(lstr);
    }

    let href = if smudge {
        find_refl_smudge(&horizontal)
    } else {
        find_refl(&horizontal)
    };

    let mut vertical = Vec::new();
    assert!(grid.len() > 0);
    for col in 0..grid[0].len() {
        let mut cstr = String::new();
        for row in 0..grid.len() {
            cstr.push(grid[row][col]);
        }
        vertical.push(cstr);
    }
    let vref = if smudge {
        find_refl_smudge(&vertical)
    } else {
        find_refl(&vertical)
    };
    assert!(href == -1 || vref == -1);
    (href, vref)
}

fn find_refl(line: &Vec<String>) -> i128 {
    let mut reflect = -1;
    for try_reflect in 0..line.len() - 1 {
        let mut li = try_reflect as i128;
        let mut ri = try_reflect as i128 + 1;
        let mut success = true;
        while li >= 0 && ri < line.len() as i128 {
            if line[li as usize] != line[ri as usize] {
                success = false;
                break;
            }
            li -= 1;
            ri += 1;
        }
        if success {
            reflect = try_reflect as i128;
        }
    }
    reflect
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { grids } = input;
    let mut sum = 0;
    for grid in grids {
        let (href, vref) = find_reflection(grid, true);
        // 0 indexed, but needs 1
        if href != -1 {
            sum += (href + 1) * 100;
        }
        if vref != -1 {
            sum += vref + 1;
        }
    }
    Ok(Answer::Num(sum))
}

fn find_refl_smudge(line: &Vec<String>) -> i128 {
    let mut reflect = -1;
    for try_reflect in 0..line.len() - 1 {
        let mut li = try_reflect as i128;
        let mut ri = try_reflect as i128 + 1;
        let mut success = true;
        let mut smudge_found = false;
        while li >= 0 && ri < line.len() as i128 {
            let diff = count_diff(&line[li as usize], &line[ri as usize]);
            if diff == 1 {
                if !smudge_found {
                    smudge_found = true
                } else {
                    success = false;
                    break;
                }
            } else if diff > 1 {
                success = false;
                break;
            }
            li -= 1;
            ri += 1;
        }
        if success && smudge_found {
            reflect = try_reflect as i128;
        }
    }
    reflect
}

fn count_diff(l: &str, r: &str) -> usize {
    let mut diff = 0;
    assert_eq!(l.len(), r.len());
    for (lc, rc) in l.chars().zip(r.chars()) {
        if lc != rc {
            diff += 1;
        }
    }
    diff
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
        assert_eq!(answer, Answer::Num(405));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(35521));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(400));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(34795));
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
