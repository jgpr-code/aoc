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
        let (href, vref) = find_reflection(grid);
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
fn find_reflection(grid: &Vec<Vec<char>>) -> (i128, i128) {
    let mut horizontal = Vec::new();
    for line in grid {
        let lstr = String::from_iter(line.iter());
        horizontal.push(lstr);
    }
    let href = find_refl(&horizontal);

    let mut vertical = Vec::new();
    assert!(grid.len() > 0);
    for col in 0..grid[0].len() {
        let mut cstr = String::new();
        for row in 0..grid.len() {
            cstr.push(grid[row][col]);
        }
        vertical.push(cstr);
    }
    let vref = find_refl(&vertical);
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
    Ok(Answer::Num(todo!()))
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
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(todo!()));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(todo!()));
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
